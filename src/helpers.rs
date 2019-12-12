use crate::api::GLApi;
use std::process::Command;

fn exec_get_string(cmd: &str, err_msg: &str) -> String {
  let output = if cfg!(target_os = "windows") {
    Command::new("cmd").args(&["/C", cmd]).output()
  } else {
    Command::new("sh").arg("-c").arg(cmd).output()
  };

  let output = match output {
    Ok(o) => o,
    Err(e) => {
      eprintln!("{} {}", err_msg, e);
      std::process::exit(1);
    }
  };

  let out_string = String::from_utf8_lossy(&output.stdout);
  out_string.trim().to_owned()
}

pub fn get_current_branch() -> String {
  let cmd = "git rev-parse --abbrev-ref HEAD";
  let err_msg = "[ERROR] Cannot get current branch";
  exec_get_string(cmd, err_msg)
}

pub fn get_default_project_branch(gl: &GLApi, project: &str) -> String {
  match gl.get_project(project) {
    Ok(pr) => pr.default_branch,
    Err(e) => {
      eprintln!("[ERROR] Cannot get default project branch. {}", e);
      std::process::exit(1);
    }
  }
}

pub fn get_git_ref_msg(git_ref: &str) -> String {
  let cmd = format!("git log --format=%B -n 1 {}", git_ref);
  let err_msg = "[ERROR] Cannot get current branch.";
  exec_get_string(&cmd, err_msg)
}

pub fn get_one_line(s: &str) -> String {
  let count = s.lines().count();
  if count < 2 {
    s.to_owned()
  } else {
    s.lines().next().unwrap_or_default().to_owned() + "..."
  }
}
