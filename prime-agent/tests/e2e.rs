#![allow(clippy::pedantic, clippy::restriction, clippy::nursery)]

use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

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

fn assert_command_succeeds(output: std::process::Output) {
    assert!(
        output.status.success(),
        "command failed with status {:?}\nstdout:\n{}\nstderr:\n{}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

fn assert_symlink_points_to(path: &Path, expected_target: &Path) -> Result<(), Box<dyn Error>> {
    let metadata = fs::symlink_metadata(path)?;
    assert!(
        metadata.file_type().is_symlink(),
        "expected symlink at {}",
        path.display()
    );

    let actual_target = fs::read_link(path)?;
    assert_eq!(
        actual_target,
        expected_target,
        "symlink {} did not point at {}",
        path.display(),
        expected_target.display()
    );

    Ok(())
}

#[cfg(unix)]
fn create_symlink(source: &Path, destination: &Path) -> Result<(), Box<dyn Error>> {
    std::os::unix::fs::symlink(source, destination)?;
    Ok(())
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

    create_symlink(&first_source, &agents_skills_dir.join(&first_skill))?;
    create_symlink(&first_source, &cursor_rules_dir.join(&first_skill))?;

    let mut command = Command::new(env!("CARGO_BIN_EXE_prime-agent"));
    command
        .arg("--target-dir")
        .arg(&project_dir)
        .env("PRIME_AGENT_TEST_INPUT", "space,down,space,enter")
        .env("PRIME_AGENT_SKILLS_DIR", &skills_root);

    let output = command.output()?;
    assert_command_succeeds(output);

    assert!(
        !path_exists(&agents_skills_dir.join(&first_skill)),
        "expected {} to be removed from .agents",
        first_skill
    );
    assert!(
        !path_exists(&cursor_rules_dir.join(&first_skill)),
        "expected {} to be removed from .cursor/rules",
        first_skill
    );

    assert_symlink_points_to(&agents_skills_dir.join(&second_skill), &second_source)?;
    assert_symlink_points_to(&cursor_rules_dir.join(&second_skill), &second_source)?;

    Ok(())
}

#[test]
fn cwd_mode_creates_layout_and_symlinks_selected_skill() -> Result<(), Box<dyn Error>> {
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
    assert_command_succeeds(output);

    let agents_link = project_dir
        .join(".agents")
        .join("skills")
        .join(&first_skill);
    let cursor_link = project_dir.join(".cursor").join("rules").join(&first_skill);

    assert!(
        path_exists(&project_dir.join(".agents").join("skills")),
        "expected .agents/skills to be created"
    );
    assert!(
        path_exists(&project_dir.join(".cursor").join("rules")),
        "expected .cursor/rules to be created"
    );

    assert_symlink_points_to(&agents_link, &first_source)?;
    assert_symlink_points_to(&cursor_link, &first_source)?;

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
    assert_command_succeeds(output);

    let agents_link = project_dir
        .join(".agents")
        .join("skills")
        .join(&target_skill);
    let cursor_link = project_dir
        .join(".cursor")
        .join("rules")
        .join(&target_skill);

    assert_symlink_points_to(&agents_link, &target_source)?;
    assert_symlink_points_to(&cursor_link, &target_source)?;

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
    assert_command_succeeds(output);

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
