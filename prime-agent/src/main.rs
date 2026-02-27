//! Prime agent command-line interface.

#![allow(clippy::pedantic, clippy::restriction, clippy::nursery)]

use std::collections::BTreeSet;
use std::env;
use std::ffi::{OsStr, OsString};
use std::fs;
use std::io::{self, IsTerminal, Write};
use std::path::{Path, PathBuf};
use std::process;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::terminal;

const TARGET_DIR_FLAG: &str = "--target-dir";
const HELP_FLAG: &str = "--help";
const SHORT_HELP_FLAG: &str = "-h";
const INIT_COMMAND: &str = "init";
const CLEAR_COMMAND: &str = "clear";
const STATUS_COMMAND: &str = "status";
const SYNC_COMMAND: &str = "sync";
const TS_TEMPLATE: &str = "ts";
const TEST_INPUT_ENV: &str = "PRIME_AGENT_TEST_INPUT";
const SKILLS_DIR_ENV: &str = "PRIME_AGENT_SKILLS_DIR";
const ANSI_YELLOW: &str = "\x1B[33m";
const ANSI_GREEN: &str = "\x1B[32m";
const ANSI_RESET: &str = "\x1B[0m";
const TS_AGENTS_MD: &str = "* every change made to the project should run:\n * typecheck\n * lint\n * prettier\n * tests\n\nif everything passes, then bump the version patch version.\n\n* the project should always use bun to run stuff and install libraries\n * bun run dev : to run the dev server.\n * bun run db:local : to run any local postgres server if that is needed\n * bun run test|typecheck|lint|format|bump : those are the base operations\n";

type AppResult<T> = Result<T, String>;

struct Config {
    command: CommandKind,
}

enum CommandKind {
    Link { target_dir: Option<PathBuf> },
    Init { template: String },
    Clear { target_dir: Option<PathBuf> },
    Status { target_dir: Option<PathBuf> },
    Sync { target_dir: Option<PathBuf> },
}

struct ApplySelectionResult {
    added_count: usize,
    removed_count: usize,
}

struct SkillStatusReport {
    managed_count: usize,
    outdated_skills: Vec<OutdatedSkill>,
}

struct OutdatedSkill {
    name: String,
    agents_outdated: bool,
    cursor_outdated: bool,
}

struct Skill {
    name: String,
    source_path: PathBuf,
}

struct TargetLayout {
    agents_skills_dir: PathBuf,
    cursor_rules_dir: PathBuf,
}

struct RawModeGuard;

struct PickerState {
    selection: Vec<bool>,
    query: String,
    visible_indices: Vec<usize>,
    cursor: usize,
}

impl RawModeGuard {
    fn activate() -> AppResult<Self> {
        terminal::enable_raw_mode()
            .map_err(|error| format!("failed to enable raw mode: {error}"))?;
        Ok(Self)
    }
}

impl Drop for RawModeGuard {
    fn drop(&mut self) {
        let _ = terminal::disable_raw_mode();
    }
}

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        process::exit(1);
    }
}

fn run() -> AppResult<()> {
    let config = parse_args()?;
    match config.command {
        CommandKind::Link { target_dir } => {
            let target_dir = resolve_target_dir(target_dir)?;
            let skills_dir = resolve_skills_dir()?;
            let skills = load_skills(&skills_dir)?;

            if skills.is_empty() {
                return Err(format!("no skills were found in {}", skills_dir.display()));
            }

            let layout = ensure_target_layout(&target_dir)?;
            let preselected = detect_preselected(&skills, &layout);
            let selected = select_skills(&skills, preselected)?;
            let result = apply_selection(&skills, &selected, &layout)?;

            println!(
                "Updated selection in {} (added {}, removed {})",
                target_dir.display(),
                result.added_count,
                result.removed_count
            );

            let report = collect_skill_status_report(&skills, &layout)?;
            print_skill_status_report(&target_dir, &report);
        }
        CommandKind::Init { template } => run_init(&template)?,
        CommandKind::Clear { target_dir } => run_clear(target_dir)?,
        CommandKind::Status { target_dir } => run_status(target_dir)?,
        CommandKind::Sync { target_dir } => run_sync(target_dir)?,
    }

    Ok(())
}

fn parse_args() -> AppResult<Config> {
    let mut args = env::args_os();
    let _binary_name = args.next();

    let Some(first_argument) = args.next() else {
        return Ok(Config {
            command: CommandKind::Link { target_dir: None },
        });
    };

    if first_argument == OsStr::new(HELP_FLAG) || first_argument == OsStr::new(SHORT_HELP_FLAG) {
        print_help();
        process::exit(0);
    }

    if first_argument == OsStr::new(INIT_COMMAND) {
        let Some(template_argument) = args.next() else {
            return Err("init requires a template".to_owned());
        };

        if let Some(extra_argument) = args.next() {
            return Err(format!(
                "unknown argument: {}",
                extra_argument.to_string_lossy()
            ));
        }

        let template = os_string_to_utf8(template_argument, "init template")?.to_ascii_lowercase();

        return Ok(Config {
            command: CommandKind::Init { template },
        });
    }

    if first_argument == OsStr::new(CLEAR_COMMAND) {
        let mut remaining_arguments: Vec<OsString> = Vec::new();
        remaining_arguments.extend(args);
        let target_dir = parse_target_dir_args(remaining_arguments)?;
        return Ok(Config {
            command: CommandKind::Clear { target_dir },
        });
    }

    if first_argument == OsStr::new(STATUS_COMMAND) {
        let mut remaining_arguments: Vec<OsString> = Vec::new();
        remaining_arguments.extend(args);
        let target_dir = parse_target_dir_args(remaining_arguments)?;
        return Ok(Config {
            command: CommandKind::Status { target_dir },
        });
    }

    if first_argument == OsStr::new(SYNC_COMMAND) {
        let mut remaining_arguments: Vec<OsString> = Vec::new();
        remaining_arguments.extend(args);
        let target_dir = parse_target_dir_args(remaining_arguments)?;
        return Ok(Config {
            command: CommandKind::Sync { target_dir },
        });
    }

    let mut remaining_arguments: Vec<OsString> = vec![first_argument];
    remaining_arguments.extend(args);

    let target_dir = parse_target_dir_args(remaining_arguments)?;

    Ok(Config {
        command: CommandKind::Link { target_dir },
    })
}

fn parse_target_dir_args(remaining_arguments: Vec<OsString>) -> AppResult<Option<PathBuf>> {
    let mut target_dir: Option<PathBuf> = None;

    let mut remaining_iter = remaining_arguments.into_iter();
    while let Some(argument) = remaining_iter.next() {
        if argument == OsStr::new(TARGET_DIR_FLAG) {
            let Some(value) = remaining_iter.next() else {
                return Err(format!("{TARGET_DIR_FLAG} requires a value"));
            };
            target_dir = Some(PathBuf::from(value));
            continue;
        }

        return Err(format!("unknown argument: {}", argument.to_string_lossy()));
    }

    Ok(target_dir)
}

fn print_help() {
    println!("prime-agent");
    println!("prime-agent --target-dir <path>");
    println!("prime-agent init ts");
    println!("prime-agent clear");
    println!("prime-agent clear --target-dir <path>");
    println!("prime-agent status");
    println!("prime-agent status --target-dir <path>");
    println!("prime-agent sync");
    println!("prime-agent sync --target-dir <path>");
}

fn run_init(template: &str) -> AppResult<()> {
    if template != TS_TEMPLATE {
        return Err("Unsupported init".to_owned());
    }

    let project_dir =
        env::current_dir().map_err(|error| format!("failed to read current directory: {error}"))?;
    let agents_path = project_dir.join("AGENTS.md");

    fs::write(&agents_path, TS_AGENTS_MD)
        .map_err(|error| format!("failed to write {}: {error}", agents_path.display()))?;

    println!("Initialized ts project in {}", project_dir.display());
    Ok(())
}

fn run_clear(explicit_target_dir: Option<PathBuf>) -> AppResult<()> {
    let target_dir = resolve_target_dir(explicit_target_dir)?;
    let skills_dir = resolve_skills_dir()?;
    let skills = load_skills(&skills_dir)?;

    let layout = target_layout(&target_dir);
    let removed_count = clear_prime_agent_skills(&skills, &layout)?;

    println!(
        "Cleared {removed_count} prime-agent skill(s) from {}",
        target_dir.display()
    );
    Ok(())
}

fn run_status(explicit_target_dir: Option<PathBuf>) -> AppResult<()> {
    let target_dir = resolve_target_dir(explicit_target_dir)?;
    let skills_dir = resolve_skills_dir()?;
    let skills = load_skills(&skills_dir)?;
    let layout = target_layout(&target_dir);
    let report = collect_skill_status_report(&skills, &layout)?;

    print_skill_status_report(&target_dir, &report);
    Ok(())
}

fn run_sync(explicit_target_dir: Option<PathBuf>) -> AppResult<()> {
    let target_dir = resolve_target_dir(explicit_target_dir)?;
    let skills_dir = resolve_skills_dir()?;
    let skills = load_skills(&skills_dir)?;

    if skills.is_empty() {
        return Err(format!("no skills were found in {}", skills_dir.display()));
    }

    let layout = ensure_target_layout(&target_dir)?;
    let managed_selection = detect_preselected(&skills, &layout);
    let synced_count = sync_selected_skills(&skills, &managed_selection, &layout)?;
    let report = collect_skill_status_report(&skills, &layout)?;

    println!("Synced {synced_count} skill(s) in {}", target_dir.display());
    print_skill_status_report(&target_dir, &report);
    Ok(())
}

fn resolve_target_dir(explicit_target_dir: Option<PathBuf>) -> AppResult<PathBuf> {
    let current_dir =
        env::current_dir().map_err(|error| format!("failed to read current directory: {error}"))?;

    let resolved = match explicit_target_dir {
        Some(path) if path.is_absolute() => path,
        Some(path) => current_dir.join(path),
        None => current_dir,
    };

    if !resolved.exists() {
        fs::create_dir_all(&resolved).map_err(|error| {
            format!(
                "failed to create target directory {}: {error}",
                resolved.display()
            )
        })?;
    }

    Ok(resolved)
}

fn resolve_skills_dir() -> AppResult<PathBuf> {
    let configured_dir = env::var_os(SKILLS_DIR_ENV)
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("..")
                .join("skills")
        });

    let absolute_dir = if configured_dir.is_absolute() {
        configured_dir
    } else {
        env::current_dir()
            .map_err(|error| format!("failed to read current directory: {error}"))?
            .join(configured_dir)
    };

    if !absolute_dir.is_dir() {
        return Err(format!(
            "skills directory does not exist: {}",
            absolute_dir.display()
        ));
    }

    absolute_dir
        .canonicalize()
        .map_err(|error| format!("failed to resolve skills directory: {error}"))
}

fn load_skills(skills_dir: &Path) -> AppResult<Vec<Skill>> {
    let entries = fs::read_dir(skills_dir).map_err(|error| {
        format!(
            "failed to read skills directory {}: {error}",
            skills_dir.display()
        )
    })?;

    let mut skills: Vec<Skill> = Vec::new();

    for entry_result in entries {
        let entry = entry_result
            .map_err(|error| format!("failed to read a skills directory entry: {error}"))?;
        let file_type = entry
            .file_type()
            .map_err(|error| format!("failed to inspect a skills directory entry: {error}"))?;

        if !file_type.is_dir() {
            continue;
        }

        let file_name = entry.file_name();
        let Some(skill_name) = file_name.to_str() else {
            continue;
        };

        let source_path = entry.path().canonicalize().map_err(|error| {
            format!(
                "failed to resolve skill directory {}: {error}",
                entry.path().display()
            )
        })?;

        skills.push(Skill {
            name: skill_name.to_owned(),
            source_path,
        });
    }

    skills.sort_by(|left, right| left.name.cmp(&right.name));
    Ok(skills)
}

fn ensure_target_layout(target_dir: &Path) -> AppResult<TargetLayout> {
    let layout = target_layout(target_dir);
    let agents_skills_dir = layout.agents_skills_dir.clone();
    let cursor_rules_dir = layout.cursor_rules_dir.clone();

    fs::create_dir_all(&agents_skills_dir).map_err(|error| {
        format!(
            "failed to create agents skills directory {}: {error}",
            agents_skills_dir.display()
        )
    })?;

    fs::create_dir_all(&cursor_rules_dir).map_err(|error| {
        format!(
            "failed to create cursor rules directory {}: {error}",
            cursor_rules_dir.display()
        )
    })?;

    Ok(layout)
}

fn target_layout(target_dir: &Path) -> TargetLayout {
    TargetLayout {
        agents_skills_dir: target_dir.join(".agents").join("skills"),
        cursor_rules_dir: target_dir.join(".cursor").join("rules"),
    }
}

fn clear_prime_agent_skills(skills: &[Skill], layout: &TargetLayout) -> AppResult<usize> {
    let mut removed_count = 0_usize;

    for skill in skills {
        let agents_path = layout.agents_skills_dir.join(&skill.name);
        let cursor_path = layout.cursor_rules_dir.join(&skill.name);
        let mut removed_any = false;

        if path_exists(&agents_path) {
            remove_path(&agents_path)?;
            removed_any = true;
        }

        if path_exists(&cursor_path) {
            remove_path(&cursor_path)?;
            removed_any = true;
        }

        if removed_any {
            removed_count += 1;
        }
    }

    Ok(removed_count)
}

fn detect_preselected(skills: &[Skill], layout: &TargetLayout) -> Vec<bool> {
    let mut selected: Vec<bool> = Vec::with_capacity(skills.len());

    for skill in skills {
        let in_agents = path_exists(&layout.agents_skills_dir.join(&skill.name));
        let in_cursor = path_exists(&layout.cursor_rules_dir.join(&skill.name));
        selected.push(in_agents || in_cursor);
    }

    selected
}

fn collect_skill_status_report(
    skills: &[Skill],
    layout: &TargetLayout,
) -> AppResult<SkillStatusReport> {
    let mut managed_count = 0_usize;
    let mut outdated_skills: Vec<OutdatedSkill> = Vec::new();

    for skill in skills {
        let agents_path = layout.agents_skills_dir.join(&skill.name);
        let cursor_path = layout.cursor_rules_dir.join(&skill.name);
        let has_agents = path_exists(&agents_path);
        let has_cursor = path_exists(&cursor_path);

        if !has_agents && !has_cursor {
            continue;
        }

        managed_count += 1;

        let agents_outdated = if has_agents {
            target_copy_is_outdated(
                &skill.source_path,
                &skill.name,
                &agents_path,
                CopyTarget::Agents,
            )?
        } else {
            true
        };

        let cursor_outdated = if has_cursor {
            target_copy_is_outdated(
                &skill.source_path,
                &skill.name,
                &cursor_path,
                CopyTarget::Cursor,
            )?
        } else {
            true
        };

        if agents_outdated || cursor_outdated {
            outdated_skills.push(OutdatedSkill {
                name: skill.name.clone(),
                agents_outdated,
                cursor_outdated,
            });
        }
    }

    Ok(SkillStatusReport {
        managed_count,
        outdated_skills,
    })
}

fn print_skill_status_report(target_dir: &Path, report: &SkillStatusReport) {
    if report.managed_count == 0 {
        println!(
            "No prime-agent-managed skills found in {}",
            target_dir.display()
        );
        return;
    }

    if report.outdated_skills.is_empty() {
        println!(
            "All {} managed skill(s) are up to date in {}",
            report.managed_count,
            target_dir.display()
        );
        return;
    }

    println!(
        "Out-of-date skill(s) in {}: {}",
        target_dir.display(),
        report.outdated_skills.len()
    );
    for outdated_skill in &report.outdated_skills {
        let location = match (
            outdated_skill.agents_outdated,
            outdated_skill.cursor_outdated,
        ) {
            (true, true) => ".agents and .cursor",
            (true, false) => ".agents",
            (false, true) => ".cursor",
            (false, false) => "none",
        };

        println!("- {} ({location})", outdated_skill.name);
    }
}

fn target_copy_is_outdated(
    source: &Path,
    skill_name: &str,
    destination: &Path,
    target: CopyTarget,
) -> AppResult<bool> {
    let metadata = fs::symlink_metadata(destination).map_err(|error| {
        format!(
            "failed to inspect destination path {}: {error}",
            destination.display()
        )
    })?;

    if !metadata.file_type().is_dir() || metadata.file_type().is_symlink() {
        return Ok(true);
    }

    directory_contents_are_outdated(source, destination, skill_name, target)
}

fn directory_contents_are_outdated(
    source: &Path,
    destination: &Path,
    skill_name: &str,
    target: CopyTarget,
) -> AppResult<bool> {
    let entries = fs::read_dir(source).map_err(|error| {
        format!(
            "failed to read source directory {}: {error}",
            source.display()
        )
    })?;
    let mut expected_names: BTreeSet<OsString> = BTreeSet::new();

    for entry_result in entries {
        let entry = entry_result
            .map_err(|error| format!("failed to read source directory entry: {error}"))?;
        let source_path = entry.path();
        let file_name = entry.file_name();
        expected_names.insert(file_name.clone());
        let destination_path = destination.join(&file_name);
        let source_file_type = entry.file_type().map_err(|error| {
            format!(
                "failed to inspect source entry {}: {error}",
                source_path.display()
            )
        })?;

        let destination_metadata = match fs::symlink_metadata(&destination_path) {
            Ok(metadata) => metadata,
            Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(true),
            Err(error) => {
                return Err(format!(
                    "failed to inspect destination entry {}: {error}",
                    destination_path.display()
                ));
            }
        };

        if source_file_type.is_dir() {
            if !destination_metadata.file_type().is_dir()
                || destination_metadata.file_type().is_symlink()
            {
                return Ok(true);
            }

            if directory_contents_are_outdated(&source_path, &destination_path, skill_name, target)?
            {
                return Ok(true);
            }
            continue;
        }

        if source_file_type.is_file() {
            if !destination_metadata.file_type().is_file()
                || destination_metadata.file_type().is_symlink()
            {
                return Ok(true);
            }

            if file_contents_are_outdated(&source_path, &destination_path, skill_name, target)? {
                return Ok(true);
            }
            continue;
        }
    }

    let destination_entries = fs::read_dir(destination).map_err(|error| {
        format!(
            "failed to read destination directory {}: {error}",
            destination.display()
        )
    })?;

    for entry_result in destination_entries {
        let entry = entry_result
            .map_err(|error| format!("failed to read destination directory entry: {error}"))?;

        if !expected_names.contains(&entry.file_name()) {
            return Ok(true);
        }
    }

    Ok(false)
}

fn file_contents_are_outdated(
    source: &Path,
    destination: &Path,
    skill_name: &str,
    target: CopyTarget,
) -> AppResult<bool> {
    let expected_bytes = expected_file_bytes(source, skill_name, target)?;
    let destination_bytes = fs::read(destination).map_err(|error| {
        format!(
            "failed to read destination file {}: {error}",
            destination.display()
        )
    })?;

    Ok(expected_bytes != destination_bytes)
}

fn expected_file_bytes(source: &Path, skill_name: &str, target: CopyTarget) -> AppResult<Vec<u8>> {
    if source.extension() == Some(OsStr::new("md")) {
        let source_contents = fs::read_to_string(source).map_err(|error| {
            format!("failed to read markdown file {}: {error}", source.display())
        })?;
        let transformed_contents = match target {
            CopyTarget::Agents => remove_cursor_header(&source_contents),
            CopyTarget::Cursor => ensure_cursor_header(&source_contents, skill_name),
        };
        return Ok(transformed_contents.into_bytes());
    }

    fs::read(source).map_err(|error| format!("failed to read file {}: {error}", source.display()))
}

fn path_exists(path: &Path) -> bool {
    fs::symlink_metadata(path).is_ok()
}

fn select_skills(skills: &[Skill], preselected: Vec<bool>) -> AppResult<Vec<bool>> {
    if let Some(scripted_input) = env::var_os(TEST_INPUT_ENV) {
        let utf8_input = os_string_to_utf8(scripted_input, TEST_INPUT_ENV)?;
        return run_scripted_selection(skills, preselected, &utf8_input);
    }

    if !io::stdin().is_terminal() || !io::stdout().is_terminal() {
        return Err(
            "interactive selection requires a terminal; set PRIME_AGENT_TEST_INPUT for scripted mode"
                .to_owned(),
        );
    }

    run_interactive_selection(skills, preselected)
}

fn os_string_to_utf8(value: OsString, variable_name: &str) -> AppResult<String> {
    value
        .into_string()
        .map_err(|_| format!("{variable_name} must be valid UTF-8"))
}

fn run_scripted_selection(
    skills: &[Skill],
    initial_selection: Vec<bool>,
    script: &str,
) -> AppResult<Vec<bool>> {
    let mut state = PickerState {
        selection: initial_selection,
        query: String::new(),
        visible_indices: Vec::new(),
        cursor: 0usize,
    };

    if state.selection.is_empty() {
        return Ok(state.selection);
    }

    refresh_visible_indices(&mut state, skills);
    let mut confirmed = false;

    for token in script
        .split(|character: char| character == ',' || character.is_whitespace())
        .filter(|token| !token.is_empty())
    {
        let command = token.to_ascii_lowercase();

        if command.starts_with("type:") {
            if let Some(typed_text) = token.get(5..) {
                state.query.push_str(typed_text);
                refresh_visible_indices(&mut state, skills);
            }
            continue;
        }

        match command.as_str() {
            "up" | "k" => move_cursor_up(&mut state.cursor, state.visible_indices.len()),
            "down" | "j" => move_cursor_down(&mut state.cursor, state.visible_indices.len()),
            "space" | "toggle" => toggle_current_selection(
                &mut state.selection,
                &state.visible_indices,
                state.cursor,
            )?,
            "backspace" => {
                state.query.pop();
                refresh_visible_indices(&mut state, skills);
            }
            "esc" | "escape" => {
                if state.query.is_empty() {
                    return Err("selection cancelled".to_owned());
                }

                state.query.clear();
                refresh_visible_indices(&mut state, skills);
            }
            "enter" | "return" => {
                confirmed = true;
                break;
            }
            _ => {
                return Err(format!(
                    "unknown scripted input token: {token}; expected up/down/space/backspace/enter/type:<text>"
                ));
            }
        }
    }

    if !confirmed {
        return Err("scripted input must include enter to confirm selection".to_owned());
    }

    Ok(state.selection)
}

fn run_interactive_selection(
    skills: &[Skill],
    initial_selection: Vec<bool>,
) -> AppResult<Vec<bool>> {
    let mut state = PickerState {
        selection: initial_selection,
        query: String::new(),
        visible_indices: Vec::new(),
        cursor: 0usize,
    };

    if state.selection.is_empty() {
        return Ok(state.selection);
    }

    refresh_visible_indices(&mut state, skills);

    let mut stdout = io::stdout();
    let raw_mode_guard = RawModeGuard::activate()?;

    loop {
        render_menu(&mut stdout, skills, &state)
            .map_err(|error| format!("failed to render menu: {error}"))?;

        let event =
            event::read().map_err(|error| format!("failed to read input event: {error}"))?;

        if let Event::Key(key_event) = event {
            if is_ctrl_c(&key_event) {
                clear_menu(&mut stdout).map_err(|error| {
                    format!("failed to clear terminal menu before interrupt: {error}")
                })?;
                drop(raw_mode_guard);
                send_interrupt_signal();
            }

            match key_event.code {
                KeyCode::Up | KeyCode::Char('k') => {
                    move_cursor_up(&mut state.cursor, state.visible_indices.len())
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    move_cursor_down(&mut state.cursor, state.visible_indices.len())
                }
                KeyCode::Char(' ') => toggle_current_selection(
                    &mut state.selection,
                    &state.visible_indices,
                    state.cursor,
                )?,
                KeyCode::Backspace => {
                    state.query.pop();
                    refresh_visible_indices(&mut state, skills);
                }
                KeyCode::Enter => {
                    clear_menu(&mut stdout).map_err(|error| {
                        format!("failed to clear terminal menu before exit: {error}")
                    })?;
                    return Ok(state.selection);
                }
                KeyCode::Esc => {
                    if state.query.is_empty() {
                        return Err("selection cancelled".to_owned());
                    }

                    state.query.clear();
                    refresh_visible_indices(&mut state, skills);
                }
                KeyCode::Char(character) => {
                    if should_add_to_query(&key_event, character) {
                        state.query.push(character);
                        refresh_visible_indices(&mut state, skills);
                    }
                }
                _ => {}
            }
        }
    }
}

fn should_add_to_query(key_event: &KeyEvent, character: char) -> bool {
    if character == ' ' || character.is_control() {
        return false;
    }

    let disallowed_modifiers = KeyModifiers::CONTROL
        | KeyModifiers::ALT
        | KeyModifiers::SUPER
        | KeyModifiers::HYPER
        | KeyModifiers::META;

    key_event.modifiers & disallowed_modifiers == KeyModifiers::NONE
}

fn refresh_visible_indices(state: &mut PickerState, skills: &[Skill]) {
    state.visible_indices = build_visible_indices(skills, &state.query);

    if state.visible_indices.is_empty() {
        state.cursor = 0usize;
        return;
    }

    let max_cursor = state.visible_indices.len().saturating_sub(1);
    if state.cursor > max_cursor {
        state.cursor = max_cursor;
    }
}

fn build_visible_indices(skills: &[Skill], query: &str) -> Vec<usize> {
    let mut visible_indices: Vec<usize> = Vec::new();

    for (index, skill) in skills.iter().enumerate() {
        if fuzzy_matches(&skill.name, query) {
            visible_indices.push(index);
        }
    }

    visible_indices
}

fn fuzzy_matches(skill_name: &str, query: &str) -> bool {
    if query.is_empty() {
        return true;
    }

    let mut query_chars = query.chars();
    let Some(mut query_char) = query_chars.next() else {
        return true;
    };

    for skill_char in skill_name.chars() {
        if skill_char.eq_ignore_ascii_case(&query_char) {
            let Some(next_query_char) = query_chars.next() else {
                return true;
            };

            query_char = next_query_char;
        }
    }

    false
}

fn is_ctrl_c(key_event: &KeyEvent) -> bool {
    if key_event.modifiers & KeyModifiers::CONTROL == KeyModifiers::NONE {
        return false;
    }

    matches!(key_event.code, KeyCode::Char('c') | KeyCode::Char('C'))
}

#[cfg(unix)]
fn send_interrupt_signal() -> ! {
    let signal_result =
        nix::sys::signal::kill(nix::unistd::Pid::this(), nix::sys::signal::Signal::SIGINT);

    if signal_result.is_err() {
        process::exit(130);
    }

    process::exit(130)
}

#[cfg(not(unix))]
fn send_interrupt_signal() -> ! {
    process::exit(130)
}

fn move_cursor_up(cursor: &mut usize, item_count: usize) {
    if item_count == 0 {
        return;
    }

    if *cursor == 0 {
        *cursor = item_count.saturating_sub(1);
        return;
    }

    *cursor -= 1;
}

fn move_cursor_down(cursor: &mut usize, item_count: usize) {
    if item_count == 0 {
        return;
    }

    let last_index = item_count.saturating_sub(1);
    if *cursor >= last_index {
        *cursor = 0;
        return;
    }

    *cursor += 1;
}

fn toggle_current_selection(
    selection: &mut [bool],
    visible_indices: &[usize],
    cursor: usize,
) -> AppResult<()> {
    let Some(skill_index) = visible_indices.get(cursor) else {
        return Ok(());
    };

    let Some(value) = selection.get_mut(*skill_index) else {
        return Err("selection index out of bounds".to_owned());
    };

    *value = !*value;
    Ok(())
}

fn render_menu(stdout: &mut impl Write, skills: &[Skill], state: &PickerState) -> io::Result<()> {
    clear_menu(stdout)?;
    write!(stdout, "Select skills to symlink:\r\n")?;

    write!(stdout, "Filter: {}\r\n\r\n", state.query)?;

    if state.visible_indices.is_empty() {
        write!(stdout, "  no matching skills\r\n")?;
    } else {
        for (visible_index, skill_index) in state.visible_indices.iter().enumerate() {
            let skill = &skills[*skill_index];
            let is_selected = state.selection[*skill_index];
            let pointer = if visible_index == state.cursor {
                '-'
            } else {
                ' '
            };
            let marker = if is_selected { "[*]" } else { "[ ]" };
            let row = format!("{pointer} {marker} - {}", skill.name);

            if visible_index == state.cursor {
                write!(stdout, "{ANSI_YELLOW}{row}{ANSI_RESET}\r\n")?;
            } else if is_selected {
                write!(stdout, "{ANSI_GREEN}{row}{ANSI_RESET}\r\n")?;
            } else {
                write!(stdout, "{row}\r\n")?;
            }
        }
    }

    write!(stdout, "\r\n")?;
    write!(
        stdout,
        "Use up/down to move, space to toggle, enter to apply.\r\n"
    )?;
    write!(
        stdout,
        "Type to fuzzy find, backspace to edit, esc to clear/cancel.\r\n"
    )?;
    stdout.flush()
}

fn clear_menu(stdout: &mut impl Write) -> io::Result<()> {
    write!(stdout, "\x1B[2J\x1B[H")?;
    stdout.flush()
}

fn apply_selection(
    skills: &[Skill],
    selection: &[bool],
    layout: &TargetLayout,
) -> AppResult<ApplySelectionResult> {
    if skills.len() != selection.len() {
        return Err("selection size did not match available skills".to_owned());
    }

    let mut added_count = 0_usize;
    let mut removed_count = 0_usize;

    for (skill, is_selected) in skills.iter().zip(selection.iter()) {
        let agents_path = layout.agents_skills_dir.join(&skill.name);
        let cursor_path = layout.cursor_rules_dir.join(&skill.name);
        let has_agents = path_exists(&agents_path);
        let has_cursor = path_exists(&cursor_path);

        if *is_selected {
            let mut added_any = false;

            if !has_agents {
                sync_skill_copy(
                    &skill.source_path,
                    &skill.name,
                    &agents_path,
                    CopyTarget::Agents,
                )?;
                added_any = true;
            }

            if !has_cursor {
                sync_skill_copy(
                    &skill.source_path,
                    &skill.name,
                    &cursor_path,
                    CopyTarget::Cursor,
                )?;
                added_any = true;
            }

            if added_any {
                added_count += 1;
            }
            continue;
        }

        let mut removed_any = false;

        if has_agents {
            remove_path(&agents_path)?;
            removed_any = true;
        }

        if has_cursor {
            remove_path(&cursor_path)?;
            removed_any = true;
        }

        if removed_any {
            removed_count += 1;
        }
    }

    Ok(ApplySelectionResult {
        added_count,
        removed_count,
    })
}

fn sync_selected_skills(
    skills: &[Skill],
    selection: &[bool],
    layout: &TargetLayout,
) -> AppResult<usize> {
    if skills.len() != selection.len() {
        return Err("selection size did not match available skills".to_owned());
    }

    let mut synced_count = 0_usize;
    for (skill, is_selected) in skills.iter().zip(selection.iter()) {
        if !*is_selected {
            continue;
        }

        let agents_path = layout.agents_skills_dir.join(&skill.name);
        let cursor_path = layout.cursor_rules_dir.join(&skill.name);

        sync_skill_copy(
            &skill.source_path,
            &skill.name,
            &agents_path,
            CopyTarget::Agents,
        )?;
        sync_skill_copy(
            &skill.source_path,
            &skill.name,
            &cursor_path,
            CopyTarget::Cursor,
        )?;
        synced_count += 1;
    }

    Ok(synced_count)
}

#[derive(Clone, Copy)]
enum CopyTarget {
    Agents,
    Cursor,
}

fn sync_skill_copy(
    source: &Path,
    skill_name: &str,
    destination: &Path,
    target: CopyTarget,
) -> AppResult<()> {
    remove_path_if_exists(destination)?;
    copy_directory_recursive(source, destination, skill_name, &target)
}

fn copy_directory_recursive(
    source: &Path,
    destination: &Path,
    skill_name: &str,
    target: &CopyTarget,
) -> AppResult<()> {
    fs::create_dir_all(destination).map_err(|error| {
        format!(
            "failed to create destination directory {}: {error}",
            destination.display()
        )
    })?;

    let entries = fs::read_dir(source).map_err(|error| {
        format!(
            "failed to read source directory {}: {error}",
            source.display()
        )
    })?;

    for entry_result in entries {
        let entry = entry_result
            .map_err(|error| format!("failed to read source directory entry: {error}"))?;
        let source_path = entry.path();
        let destination_path = destination.join(entry.file_name());
        let file_type = entry.file_type().map_err(|error| {
            format!(
                "failed to inspect source entry {}: {error}",
                source_path.display()
            )
        })?;

        if file_type.is_dir() {
            copy_directory_recursive(&source_path, &destination_path, skill_name, target)?;
            continue;
        }

        if file_type.is_file() {
            copy_file_for_target(&source_path, &destination_path, skill_name, target)?;
            continue;
        }
    }

    Ok(())
}

fn copy_file_for_target(
    source: &Path,
    destination: &Path,
    skill_name: &str,
    target: &CopyTarget,
) -> AppResult<()> {
    if source.extension() == Some(OsStr::new("md")) {
        let contents = fs::read_to_string(source).map_err(|error| {
            format!("failed to read markdown file {}: {error}", source.display())
        })?;

        let transformed_contents = match target {
            CopyTarget::Agents => remove_cursor_header(&contents),
            CopyTarget::Cursor => ensure_cursor_header(&contents, skill_name),
        };

        fs::write(destination, transformed_contents).map_err(|error| {
            format!(
                "failed to write markdown file {}: {error}",
                destination.display()
            )
        })?;
        return Ok(());
    }

    fs::copy(source, destination).map_err(|error| {
        format!(
            "failed to copy file {} to {}: {error}",
            source.display(),
            destination.display()
        )
    })?;

    Ok(())
}

fn remove_cursor_header(contents: &str) -> String {
    if let Some(body_start) = cursor_header_body_start(contents) {
        return contents[body_start..].to_owned();
    }

    contents.to_owned()
}

fn ensure_cursor_header(contents: &str, skill_name: &str) -> String {
    if cursor_header_body_start(contents).is_some() {
        return contents.to_owned();
    }

    format!("---\ndescription: {skill_name}\nglobs: \"**/*\"\nalwaysApply: false\n---\n{contents}")
}

fn cursor_header_body_start(contents: &str) -> Option<usize> {
    let mut lines = contents.split_inclusive('\n');
    let first_line = lines.next()?;
    if trim_line(first_line) != "---" {
        return None;
    }

    let mut offset = first_line.len();
    for line in lines {
        offset += line.len();
        if trim_line(line) == "---" {
            return Some(offset);
        }
    }

    None
}

fn trim_line(line: &str) -> &str {
    let without_newline = line.strip_suffix('\n').unwrap_or(line);
    without_newline
        .strip_suffix('\r')
        .unwrap_or(without_newline)
}

fn remove_path_if_exists(path: &Path) -> AppResult<()> {
    if !path_exists(path) {
        return Ok(());
    }

    remove_path(path)
}

fn remove_path(path: &Path) -> AppResult<()> {
    let metadata = fs::symlink_metadata(path)
        .map_err(|error| format!("failed to inspect path {}: {error}", path.display()))?;

    if metadata.file_type().is_dir() && !metadata.file_type().is_symlink() {
        fs::remove_dir_all(path)
            .map_err(|error| format!("failed to remove directory {}: {error}", path.display()))?;
        return Ok(());
    }

    fs::remove_file(path)
        .map_err(|error| format!("failed to remove path {}: {error}", path.display()))?;
    Ok(())
}
