use super::{args::Args, helpers};
use clap::ArgMatches;
use gitlabapi::{CreateMRBody, GLApi, GetUsersQuery, MergeRequest, UserState};
use std::io::{stdin, stdout, Write};

#[derive(Debug)]
pub struct CreateMRArgsData<'a> {
  source_branch: Option<&'a str>,
  target_branch: Option<&'a str>,
  assignee_id: Option<&'a str>,
  assignee_name: Option<&'a str>,
  title: Option<&'a str>,
}

pub fn collect_args<'a>(m: &'a ArgMatches) -> Args<'a> {
  // pub id: &'a str,
  // pub source_branch: &'a str,
  // pub target_branch: &'a str,
  // pub title: &'a str,
  // pub assignee_id: Option<u32>,
  // pub description: Option<&'a str>,
  // pub remove_source_branch: Option<bool>,
  // pub squash: Option<bool>,

  let source_branch = m.value_of("source-branch");
  let target_branch = m.value_of("target-branch");
  let assignee_id = m.value_of("assignee-id");
  let assignee_name = m.value_of("assignee-name");
  let title = m.value_of("title");

  let args_data = CreateMRArgsData {
    source_branch,
    target_branch,
    assignee_id,
    assignee_name,
    title,
  };
  Args::CreateMR(args_data)
}

pub fn fill_mr_create_data<'a>(
  glapi: &GLApi,
  project: &'a str,
  args_data: &'a CreateMRArgsData,
) -> CreateMRBody<'a> {
  let source_branch = if let Some(s) = args_data.source_branch {
    s.to_owned()
  } else {
    helpers::get_current_branch()
  };

  let target_branch = if let Some(s) = args_data.target_branch {
    s.to_owned()
  } else {
    helpers::get_default_project_branch(glapi, project)
  };

  let title = if let Some(t) = args_data.title {
    t.to_owned()
  } else {
    helpers::get_git_ref_msg(&source_branch)
  };

  let assignee_id = get_assignee_id(glapi, args_data);

  CreateMRBody {
    id: project.to_owned(),
    source_branch: source_branch,
    target_branch: target_branch,
    title,
    assignee_id,
    description: None,
  }
}

fn get_assignee_id<'a>(glapi: &'a GLApi, args_data: &'a CreateMRArgsData) -> Option<u32> {
  if let Some(assignee_name) = args_data.assignee_name {
    let uq = GetUsersQuery::new()
      .username(assignee_name)
      .state(UserState::Active);

    let users = glapi.get_users(&uq).unwrap_or_else(|err| {
      eprintln!(
        "[ERROR] You specify assignee name, but users request failed. {}",
        err
      );
      std::process::exit(1);
    });
    let user = users.get(0).unwrap_or_else(|| {
      eprintln!("[ERROR] Cannot find user with name: `{}`", assignee_name);
      std::process::exit(1);
    });
    return Some(user.id);
  } else if let Some(id_str) = args_data.assignee_id {
    let id: u32 = id_str.parse().unwrap_or_else(|err| {
      eprintln!(
        "[ERROR] You specify assignee id, but it is not a valid id. {}",
        err
      );
      std::process::exit(1);
    });
    return Some(id);
  };

  None
}

fn get_assignee_str(id: Option<u32>, args: &CreateMRArgsData) -> String {
  if let Some(i) = id {
    let mut s = format!("(ID: {})", i);
    if let Some(name) = args.assignee_name {
      s = format!("{} {}", name, s);
    }
    s
  } else {
    "None".to_owned()
  }
}

fn prompt() -> bool {
  print!("Do you want to continue? [Y/n]");
  let mut s = String::new();
  let _ = stdout().flush();
  stdin()
    .read_line(&mut s)
    .expect("Did not enter a correct string");

  let s = s.trim();

  s == "" || s == "y" || s == "Y"
}

pub fn confirm_mr(mr_data: &CreateMRBody, args: &CreateMRArgsData) {
  println!("You creating merge requests with this parameters:");
  println!("  Source branch: — {}", mr_data.source_branch);
  println!("  Target branch: — {}", mr_data.target_branch);
  println!("  Title branch:  — {}", mr_data.title);
  let assignee = get_assignee_str(mr_data.assignee_id, args);
  println!("  Assignee:    —   {}", assignee);

  if !prompt() {
    println!("Canceling...");
    std::process::exit(1);
  }
}

pub fn log_new_mr(mr: &MergeRequest) {
  println!("\nYour merge request is created. You can see it here:");
  println!("{}", mr.web_url);
  println!("Status: {}", mr.merge_status);
}
