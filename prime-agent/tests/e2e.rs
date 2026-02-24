#![allow(clippy::pedantic, clippy::restriction, clippy::nursery)]

use std::error::Error;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

const TS_AGENTS_MD: &str = "* every change made to the project should run:\n * typecheck\n * lint\n * prettier\n * tests\n\nif everything passes, then bump the version patch version.\n\n* the project should always use bun to run stuff and install libraries\n * bun run dev : to run the dev server.\n * bun run db:local : to run any local postgres server if that is needed\n * bun run test|typecheck|lint|format|bump : those are the base operations\n";

fn skills_dir() -> Result<PathBuf, Box<dyn Error>> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("skills");
    let canonical = path.canonicalize()?;
    Ok(canonical)
}

fn sorted_skill_names(skills_root: &Path) -> Result<Vec<String>, Box<dyn Error>> {
    let mut names: Vec<String> = Vec::new();

    for entry_result in fs::read_dir(skills_root)? {
        let entry = entry_result?;
        if !entry.file_type()?.is_dir() {
            continue;
        }

        if let Some(name) = entry.file_name().to_str() {
            names.push(name.to_owned());
        }
    }

    names.sort();
    Ok(names)
}

fn path_exists(path: &Path) -> bool {
    fs::symlink_metadata(path).is_ok()
}

fn assert_command_succeeds(output: &Output) {
    assert!(
        output.status.success(),
        "command failed with status {:?}\nstdout:\n{}\nstderr:\n{}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

fn assert_regular_directory(path: &Path) -> Result<(), Box<dyn Error>> {
    let metadata = fs::symlink_metadata(path)?;
    assert!(
        metadata.file_type().is_dir(),
        "expected directory at {}",
        path.display()
    );
    assert!(
        !metadata.file_type().is_symlink(),
        "expected a copied directory (not symlink) at {}",
        path.display()
    );
    Ok(())
}

fn assert_regular_file(path: &Path) -> Result<(), Box<dyn Error>> {
    let metadata = fs::symlink_metadata(path)?;
    assert!(
        metadata.file_type().is_file(),
        "expected file at {}",
        path.display()
    );
    assert!(
        !metadata.file_type().is_symlink(),
        "expected a copied file (not symlink) at {}",
        path.display()
    );
    Ok(())
}

fn copy_directory(source: &Path, destination: &Path) -> Result<(), Box<dyn Error>> {
    fs::create_dir_all(destination)?;

    for entry_result in fs::read_dir(source)? {
        let entry = entry_result?;
        let source_path = entry.path();
        let destination_path = destination.join(entry.file_name());
        let file_type = entry.file_type()?;

        if file_type.is_dir() {
            copy_directory(&source_path, &destination_path)?;
            continue;
        }

        if file_type.is_file() {
            fs::copy(&source_path, &destination_path)?;
            continue;
        }
    }

    Ok(())
}

fn first_markdown_file(skill_dir: &Path) -> Result<PathBuf, Box<dyn Error>> {
    let mut markdown_paths: Vec<PathBuf> = Vec::new();
    collect_markdown_files(skill_dir, &mut markdown_paths)?;
    markdown_paths.sort();

    markdown_paths
        .into_iter()
        .next()
        .ok_or("expected at least one markdown file in skill".into())
}

fn collect_markdown_files(
    directory: &Path,
    output: &mut Vec<PathBuf>,
) -> Result<(), Box<dyn Error>> {
    for entry_result in fs::read_dir(directory)? {
        let entry = entry_result?;
        let path = entry.path();
        let file_type = entry.file_type()?;

        if file_type.is_dir() {
            collect_markdown_files(&path, output)?;
            continue;
        }

        if !file_type.is_file() {
            continue;
        }

        if path.extension() == Some(OsStr::new("md")) {
            output.push(path);
        }
    }

    Ok(())
}

fn cursor_header_body_start(content: &str) -> Option<usize> {
    let mut lines = content.split_inclusive('\n');
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

fn has_cursor_header(content: &str) -> bool {
    cursor_header_body_start(content).is_some()
}

fn strip_cursor_header(content: &str) -> String {
    if let Some(body_start) = cursor_header_body_start(content) {
        return content[body_start..].to_owned();
    }

    content.to_owned()
}

fn find_skill_with_cursor_header(skills_root: &Path) -> Result<String, Box<dyn Error>> {
    for skill_name in sorted_skill_names(skills_root)? {
        let skill_path = skills_root.join(&skill_name);
        let markdown_file = first_markdown_file(&skill_path)?;
        let contents = fs::read_to_string(markdown_file)?;
        if has_cursor_header(&contents) {
            return Ok(skill_name);
        }
    }

    Err("expected at least one skill with a cursor header".into())
}

fn find_skill_without_cursor_header(skills_root: &Path) -> Result<String, Box<dyn Error>> {
    for skill_name in sorted_skill_names(skills_root)? {
        let skill_path = skills_root.join(&skill_name);
        let markdown_file = first_markdown_file(&skill_path)?;
        let contents = fs::read_to_string(markdown_file)?;
        if !has_cursor_header(&contents) {
            return Ok(skill_name);
        }
    }

    Err("expected at least one skill without a cursor header".into())
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

fn unique_full_name_query(skills: &[String]) -> Result<String, Box<dyn Error>> {
    for skill_name in skills {
        let matches = skills
            .iter()
            .filter(|candidate| fuzzy_matches(candidate, skill_name))
            .count();

        if matches == 1 {
            return Ok(skill_name.clone());
        }
    }

    Err("expected at least one skill with a unique fuzzy query".into())
}

#[test]
fn target_dir_mode_updates_preselected_skills() -> Result<(), Box<dyn Error>> {
    let skills_root = skills_dir()?;
    let mut names = sorted_skill_names(&skills_root)?.into_iter();
    let first_skill = names.next().ok_or("expected at least one skill")?;
    let second_skill = names.next().ok_or("expected at least two skills")?;

    let first_source = skills_root.join(&first_skill).canonicalize()?;
    let second_source = skills_root.join(&second_skill).canonicalize()?;

    let temp_dir = tempfile::Builder::new()
        .prefix("prime-agent-target-")
        .tempdir_in("/tmp")?;
    let project_dir = temp_dir.path().join("project");
    fs::create_dir_all(&project_dir)?;

    let agents_skills_dir = project_dir.join(".agents").join("skills");
    let cursor_rules_dir = project_dir.join(".cursor").join("rules");
    fs::create_dir_all(&agents_skills_dir)?;
    fs::create_dir_all(&cursor_rules_dir)?;

    copy_directory(&first_source, &agents_skills_dir.join(&first_skill))?;
    copy_directory(&first_source, &cursor_rules_dir.join(&first_skill))?;

    let mut command = Command::new(env!("CARGO_BIN_EXE_prime-agent"));
    command
        .arg("--target-dir")
        .arg(&project_dir)
        .env("PRIME_AGENT_TEST_INPUT", "space,down,space,enter")
        .env("PRIME_AGENT_SKILLS_DIR", &skills_root);

    let output = command.output()?;
    assert_command_succeeds(&output);

    assert!(
        !path_exists(&agents_skills_dir.join(&first_skill)),
        "expected {first_skill} to be removed from .agents"
    );
    assert!(
        !path_exists(&cursor_rules_dir.join(&first_skill)),
        "expected {first_skill} to be removed from .cursor/rules"
    );

    let second_agents_path = agents_skills_dir.join(&second_skill);
    let second_cursor_path = cursor_rules_dir.join(&second_skill);
    assert_regular_directory(&second_agents_path)?;
    assert_regular_directory(&second_cursor_path)?;

    let second_markdown_source = first_markdown_file(&second_source)?;
    let relative_markdown = second_markdown_source.strip_prefix(&second_source)?;
    assert_regular_file(&second_agents_path.join(relative_markdown))?;
    assert_regular_file(&second_cursor_path.join(relative_markdown))?;

    Ok(())
}

#[test]
fn cwd_mode_creates_layout_and_copies_selected_skill() -> Result<(), Box<dyn Error>> {
    let skills_root = skills_dir()?;
    let first_skill = sorted_skill_names(&skills_root)?
        .into_iter()
        .next()
        .ok_or("expected at least one skill")?;
    let first_source = skills_root.join(&first_skill).canonicalize()?;

    let temp_dir = tempfile::Builder::new()
        .prefix("prime-agent-cwd-")
        .tempdir_in("/tmp")?;
    let project_dir = temp_dir.path().join("project");
    fs::create_dir_all(&project_dir)?;

    let mut command = Command::new(env!("CARGO_BIN_EXE_prime-agent"));
    command
        .current_dir(&project_dir)
        .env("PRIME_AGENT_TEST_INPUT", "space,enter")
        .env("PRIME_AGENT_SKILLS_DIR", &skills_root);

    let output = command.output()?;
    assert_command_succeeds(&output);

    let agents_skill_path = project_dir
        .join(".agents")
        .join("skills")
        .join(&first_skill);
    let cursor_skill_path = project_dir.join(".cursor").join("rules").join(&first_skill);

    assert_regular_directory(&project_dir.join(".agents").join("skills"))?;
    assert_regular_directory(&project_dir.join(".cursor").join("rules"))?;
    assert_regular_directory(&agents_skill_path)?;
    assert_regular_directory(&cursor_skill_path)?;

    let markdown_source = first_markdown_file(&first_source)?;
    let relative_markdown = markdown_source.strip_prefix(&first_source)?;
    assert_regular_file(&agents_skill_path.join(relative_markdown))?;
    assert_regular_file(&cursor_skill_path.join(relative_markdown))?;

    Ok(())
}

#[test]
fn typing_query_filters_before_selection() -> Result<(), Box<dyn Error>> {
    let skills_root = skills_dir()?;
    let skill_names = sorted_skill_names(&skills_root)?;
    let target_skill = unique_full_name_query(&skill_names)?;
    let target_source = skills_root.join(&target_skill).canonicalize()?;

    let temp_dir = tempfile::Builder::new()
        .prefix("prime-agent-fuzzy-")
        .tempdir_in("/tmp")?;
    let project_dir = temp_dir.path().join("project");
    fs::create_dir_all(&project_dir)?;

    let script = format!("type:{target_skill},space,enter");
    let mut command = Command::new(env!("CARGO_BIN_EXE_prime-agent"));
    command
        .current_dir(&project_dir)
        .env("PRIME_AGENT_TEST_INPUT", &script)
        .env("PRIME_AGENT_SKILLS_DIR", &skills_root);

    let output = command.output()?;
    assert_command_succeeds(&output);

    let agents_skill_path = project_dir
        .join(".agents")
        .join("skills")
        .join(&target_skill);
    let cursor_skill_path = project_dir
        .join(".cursor")
        .join("rules")
        .join(&target_skill);

    assert_regular_directory(&agents_skill_path)?;
    assert_regular_directory(&cursor_skill_path)?;

    let markdown_source = first_markdown_file(&target_source)?;
    let relative_markdown = markdown_source.strip_prefix(&target_source)?;
    assert_regular_file(&agents_skill_path.join(relative_markdown))?;
    assert_regular_file(&cursor_skill_path.join(relative_markdown))?;

    Ok(())
}

#[test]
fn agents_remove_cursor_header_cursor_keeps_header() -> Result<(), Box<dyn Error>> {
    let skills_root = skills_dir()?;
    let skill_name = find_skill_with_cursor_header(&skills_root)?;
    let skill_source = skills_root.join(&skill_name);
    let markdown_source = first_markdown_file(&skill_source)?;
    let source_contents = fs::read_to_string(&markdown_source)?;
    assert!(
        has_cursor_header(&source_contents),
        "expected selected source skill to include cursor header"
    );

    let temp_dir = tempfile::Builder::new()
        .prefix("prime-agent-header-transform-")
        .tempdir_in("/tmp")?;
    let project_dir = temp_dir.path().join("project");
    fs::create_dir_all(&project_dir)?;

    let script = format!("type:{skill_name},space,enter");
    let mut command = Command::new(env!("CARGO_BIN_EXE_prime-agent"));
    command
        .arg("--target-dir")
        .arg(&project_dir)
        .env("PRIME_AGENT_TEST_INPUT", &script)
        .env("PRIME_AGENT_SKILLS_DIR", &skills_root);

    let output = command.output()?;
    assert_command_succeeds(&output);

    let relative_markdown = markdown_source.strip_prefix(&skill_source)?;
    let agents_markdown_path = project_dir
        .join(".agents")
        .join("skills")
        .join(&skill_name)
        .join(relative_markdown);
    let cursor_markdown_path = project_dir
        .join(".cursor")
        .join("rules")
        .join(&skill_name)
        .join(relative_markdown);

    let agents_contents = fs::read_to_string(agents_markdown_path)?;
    let cursor_contents = fs::read_to_string(cursor_markdown_path)?;

    assert!(
        has_cursor_header(&cursor_contents),
        "expected cursor copy to keep cursor header"
    );
    assert!(
        !has_cursor_header(&agents_contents),
        "expected .agents copy to remove cursor header"
    );
    assert_eq!(cursor_contents, source_contents);
    assert_eq!(agents_contents, strip_cursor_header(&source_contents));

    Ok(())
}

#[test]
fn cursor_adds_header_when_source_is_missing_one() -> Result<(), Box<dyn Error>> {
    let skills_root = skills_dir()?;
    let skill_name = find_skill_without_cursor_header(&skills_root)?;
    let skill_source = skills_root.join(&skill_name);
    let markdown_source = first_markdown_file(&skill_source)?;
    let source_contents = fs::read_to_string(&markdown_source)?;
    assert!(
        !has_cursor_header(&source_contents),
        "expected selected source skill to not include cursor header"
    );

    let temp_dir = tempfile::Builder::new()
        .prefix("prime-agent-header-add-")
        .tempdir_in("/tmp")?;
    let project_dir = temp_dir.path().join("project");
    fs::create_dir_all(&project_dir)?;

    let script = format!("type:{skill_name},space,enter");
    let mut command = Command::new(env!("CARGO_BIN_EXE_prime-agent"));
    command
        .arg("--target-dir")
        .arg(&project_dir)
        .env("PRIME_AGENT_TEST_INPUT", &script)
        .env("PRIME_AGENT_SKILLS_DIR", &skills_root);

    let output = command.output()?;
    assert_command_succeeds(&output);

    let relative_markdown = markdown_source.strip_prefix(&skill_source)?;
    let agents_markdown_path = project_dir
        .join(".agents")
        .join("skills")
        .join(&skill_name)
        .join(relative_markdown);
    let cursor_markdown_path = project_dir
        .join(".cursor")
        .join("rules")
        .join(&skill_name)
        .join(relative_markdown);

    let agents_contents = fs::read_to_string(agents_markdown_path)?;
    let cursor_contents = fs::read_to_string(cursor_markdown_path)?;

    assert_eq!(agents_contents, source_contents);
    assert!(!has_cursor_header(&agents_contents));
    assert!(has_cursor_header(&cursor_contents));
    assert_eq!(strip_cursor_header(&cursor_contents), source_contents);

    Ok(())
}

#[test]
fn clear_removes_prime_agent_skills_only() -> Result<(), Box<dyn Error>> {
    let skills_root = skills_dir()?;
    let mut names = sorted_skill_names(&skills_root)?.into_iter();
    let first_skill = names.next().ok_or("expected at least one skill")?;
    let second_skill = names.next().ok_or("expected at least two skills")?;

    let first_source = skills_root.join(&first_skill).canonicalize()?;
    let second_source = skills_root.join(&second_skill).canonicalize()?;

    let temp_dir = tempfile::Builder::new()
        .prefix("prime-agent-clear-")
        .tempdir_in("/tmp")?;
    let project_dir = temp_dir.path().join("project");
    fs::create_dir_all(&project_dir)?;

    let agents_skills_dir = project_dir.join(".agents").join("skills");
    let cursor_rules_dir = project_dir.join(".cursor").join("rules");
    fs::create_dir_all(&agents_skills_dir)?;
    fs::create_dir_all(&cursor_rules_dir)?;

    copy_directory(&first_source, &agents_skills_dir.join(&first_skill))?;
    copy_directory(&first_source, &cursor_rules_dir.join(&first_skill))?;
    copy_directory(&second_source, &agents_skills_dir.join(&second_skill))?;
    copy_directory(&second_source, &cursor_rules_dir.join(&second_skill))?;

    let custom_agents_path = agents_skills_dir.join("custom-skill");
    let custom_cursor_path = cursor_rules_dir.join("custom-skill");
    fs::create_dir_all(&custom_agents_path)?;
    fs::create_dir_all(&custom_cursor_path)?;
    fs::write(custom_agents_path.join("note.txt"), "keep me")?;
    fs::write(custom_cursor_path.join("note.txt"), "keep me")?;

    let mut command = Command::new(env!("CARGO_BIN_EXE_prime-agent"));
    command
        .current_dir(&project_dir)
        .arg("clear")
        .env("PRIME_AGENT_SKILLS_DIR", &skills_root);

    let output = command.output()?;
    assert_command_succeeds(&output);

    assert!(!path_exists(&agents_skills_dir.join(&first_skill)));
    assert!(!path_exists(&cursor_rules_dir.join(&first_skill)));
    assert!(!path_exists(&agents_skills_dir.join(&second_skill)));
    assert!(!path_exists(&cursor_rules_dir.join(&second_skill)));

    assert_regular_directory(&custom_agents_path)?;
    assert_regular_directory(&custom_cursor_path)?;
    assert_regular_file(&custom_agents_path.join("note.txt"))?;
    assert_regular_file(&custom_cursor_path.join("note.txt"))?;

    Ok(())
}

#[test]
fn init_ts_writes_agents_markdown() -> Result<(), Box<dyn Error>> {
    let temp_dir = tempfile::Builder::new()
        .prefix("prime-agent-init-ts-")
        .tempdir_in("/tmp")?;
    let project_dir = temp_dir.path().join("project");
    fs::create_dir_all(&project_dir)?;

    let mut command = Command::new(env!("CARGO_BIN_EXE_prime-agent"));
    command.current_dir(&project_dir).arg("init").arg("ts");

    let output = command.output()?;
    assert_command_succeeds(&output);

    let agents_path = project_dir.join("AGENTS.md");
    let contents = fs::read_to_string(&agents_path)?;
    assert_eq!(contents, TS_AGENTS_MD);

    Ok(())
}

#[test]
fn init_unsupported_template_fails() -> Result<(), Box<dyn Error>> {
    let temp_dir = tempfile::Builder::new()
        .prefix("prime-agent-init-unsupported-")
        .tempdir_in("/tmp")?;
    let project_dir = temp_dir.path().join("project");
    fs::create_dir_all(&project_dir)?;

    let mut command = Command::new(env!("CARGO_BIN_EXE_prime-agent"));
    command.current_dir(&project_dir).arg("init").arg("go");

    let output = command.output()?;
    assert!(
        !output.status.success(),
        "expected unsupported init to fail"
    );

    let stderr_text = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr_text.contains("Unsupported init"),
        "stderr did not include unsupported init message\nstderr:\n{stderr_text}"
    );

    Ok(())
}
