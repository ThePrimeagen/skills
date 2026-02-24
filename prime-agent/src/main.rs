//! Prime agent command-line interface.

#![allow(clippy::pedantic, clippy::restriction, clippy::nursery)]

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
            let linked_count = apply_selection(&skills, &selected, &layout)?;

            println!(
                "Linked {linked_count} skill(s) into {}",
                target_dir.display()
            );
        }
        CommandKind::Init { template } => run_init(&template)?,
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

    let mut remaining_arguments: Vec<OsString> = vec![first_argument];
    remaining_arguments.extend(args);

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

    Ok(Config {
        command: CommandKind::Link { target_dir },
    })
}

fn print_help() {
    println!("prime-agent");
    println!("prime-agent --target-dir <path>");
    println!("prime-agent init ts");
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
    let agents_skills_dir = target_dir.join(".agents").join("skills");
    let cursor_rules_dir = target_dir.join(".cursor").join("rules");

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

    Ok(TargetLayout {
        agents_skills_dir,
        cursor_rules_dir,
    })
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
) -> AppResult<usize> {
    if skills.len() != selection.len() {
        return Err("selection size did not match available skills".to_owned());
    }

    let mut linked_count = 0_usize;

    for (skill, is_selected) in skills.iter().zip(selection.iter()) {
        let agents_link = layout.agents_skills_dir.join(&skill.name);
        let cursor_link = layout.cursor_rules_dir.join(&skill.name);

        if *is_selected {
            sync_link(&skill.source_path, &agents_link)?;
            sync_link(&skill.source_path, &cursor_link)?;
            linked_count += 1;
            continue;
        }

        remove_path_if_exists(&agents_link)?;
        remove_path_if_exists(&cursor_link)?;
    }

    Ok(linked_count)
}

fn sync_link(source: &Path, destination: &Path) -> AppResult<()> {
    if let Ok(metadata) = fs::symlink_metadata(destination) {
        if metadata.file_type().is_symlink() {
            let existing_target = fs::read_link(destination).map_err(|error| {
                format!(
                    "failed to read existing symlink {}: {error}",
                    destination.display()
                )
            })?;

            if existing_target == source {
                return Ok(());
            }
        }

        remove_path(destination)?;
    }

    create_symlink(source, destination).map_err(|error| {
        format!(
            "failed to create symlink {} -> {}: {error}",
            destination.display(),
            source.display()
        )
    })?;

    Ok(())
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

#[cfg(unix)]
fn create_symlink(source: &Path, destination: &Path) -> io::Result<()> {
    std::os::unix::fs::symlink(source, destination)
}

#[cfg(not(unix))]
fn create_symlink(_source: &Path, _destination: &Path) -> io::Result<()> {
    Err(io::Error::new(
        io::ErrorKind::Other,
        "prime-agent currently supports symlinks on Unix only",
    ))
}
