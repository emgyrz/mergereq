use super::helpers;
use crate::api::{CreateMRBody, GLApi, GetUsersQuery, MergeRequest, UserState};
use clap::ArgMatches;
use std::io::{stdin, stdout, Write};

pub fn fill_mr_create_data<'a>(
  glapi: &GLApi,
  project: &'a str,
  args_matches: &'a ArgMatches,
) -> CreateMRBody<'a> {
  let source_branch = args_matches.value_of("source-branch");
  let target_branch = args_matches.value_of("target-branch");
  let assignee_id = args_matches.value_of("assignee-id");
  let assignee_name = args_matches.value_of("assignee-name");
  let title = args_matches.value_of("title");

  let source_branch = if let Some(s) = source_branch {
    s.to_owned()
  } else {
    helpers::get_current_branch()
  };

  let target_branch = if let Some(s) = target_branch {
    s.to_owned()
  } else {
    helpers::get_default_project_branch(glapi, project)
  };

  let title = if let Some(t) = title {
    t.to_owned()
  } else {
    helpers::get_git_ref_msg(&source_branch)
  };

  let assignee_id = get_assignee_id(glapi, assignee_id, assignee_name);

  CreateMRBody {
    id: project.to_owned(),
    source_branch,
    target_branch,
    title,
    assignee_id,
    description: args_matches.value_of("description"),
    remove_source_branch: Some(args_matches.is_present("remove-source-branch")),
    squash: Some(args_matches.is_present("squash")),
  }
}

fn get_assignee_id<'a>(
  glapi: &'a GLApi,
  assignee_id: Option<&str>,
  assignee_name: Option<&str>,
) -> Option<u32> {
  if let Some(name) = assignee_name {
    let uq = GetUsersQuery::new().username(name).state(UserState::Active);

    let users = glapi.get_users(&uq).unwrap_or_else(|err| {
      eprintln!(
        "[ERROR] You specify assignee name, but users request failed. {}",
        err
      );
      std::process::exit(1);
    });
    let user = users.get(0).unwrap_or_else(|| {
      eprintln!("[ERROR] Cannot find user with name: `{}`", name);
      std::process::exit(1);
    });
    return Some(user.id);
  } else if let Some(id_str) = assignee_id {
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

fn get_assignee_str(id: Option<u32>, name: Option<&str>) -> String {
  if let Some(i) = id {
    let mut s = format!("(ID: {})", i);
    if let Some(name) = name {
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

pub fn confirm_mr(mr_data: &CreateMRBody, args: &ArgMatches) {
  println!("You creating merge requests with this parameters:");
  println!("  Source branch: — {}", mr_data.source_branch);
  println!("  Target branch: — {}", mr_data.target_branch);
  let title = helpers::get_one_line(&mr_data.title);
  println!("  Title branch:  — {}", title);
  let assignee = get_assignee_str(mr_data.assignee_id, args.value_of("assignee-name"));
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
