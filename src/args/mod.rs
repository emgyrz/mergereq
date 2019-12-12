use clap::ArgMatches;
use std::str::FromStr;

use crate::api::{
  GetBranchesQuery, GetMergeRequestsQuery, GetProjectsQuery, GetUsersQuery, MRScope, MRState,
  ProjectVisibility, UserState,
};

mod defs;
pub(crate) use defs::Args;

pub fn parse_args<'a>(arg_matches: &'a ArgMatches) -> Args<'a> {
  if let Some(m) = arg_matches.subcommand_matches("ls") {
    return handle_ls(m);
  } else if let Some(m) = arg_matches.subcommand_matches("create") {
    return handle_create(m);
  } else if let Some(m) = arg_matches.subcommand_matches("config") {
    return handle_config(m);
  }

  Args::Unknown
}

fn handle_ls<'a>(mat: &'a ArgMatches) -> Args<'a> {
  if let Some(m) = mat.subcommand_matches("users") {
    return handle_ls_users(m);
  } else if let Some(m) = mat.subcommand_matches("projects") {
    return handle_ls_projects(m);
  } else if let Some(m) = mat.subcommand_matches("branches") {
    return handle_ls_branches(m);
  } else if let Some(m) = mat.subcommand_matches("mr") {
    return handle_ls_mr(m);
  }

  Args::Unknown
}

fn handle_create<'a>(mat: &'a ArgMatches) -> Args<'a> {
  if let Some(m) = mat.subcommand_matches("mr") {
    return Args::CreateMR(m);
  }
  Args::Unknown
}

fn handle_config<'a>(mat: &'a ArgMatches) -> Args<'a> {
  if let Some(m) = mat.subcommand_matches("save-token") {
    let token = m.value_of("private-token").unwrap();
    return Args::CfgSaveToken {
      token,
      file_path: m.value_of("file-path"),
    };
  } else if let Some(m) = mat.subcommand_matches("show-token") {
    return Args::CfgShowToken {
      file_path: m.value_of("file-path"),
    };
  }
  Args::Unknown
}

fn handle_ls_projects<'a>(m: &'a ArgMatches) -> Args<'a> {
  let mut q = GetProjectsQuery::new();
  if let Some(s) = m.value_of("search") {
    q = q.search(s);
  }

  if let Some(vis) = m.value_of("visibility") {
    if let Ok(v) = ProjectVisibility::from_str(vis) {
      q = q.visibility(v)
    }
  }

  if m.is_present("archived") {
    q = q.archived(true);
  }
  if m.is_present("owned") {
    q = q.owned(true);
  }
  if m.is_present("membership") {
    q = q.membership(true);
  }
  Args::LsProjects(q)
}

fn handle_ls_users<'a>(m: &'a ArgMatches) -> Args<'a> {
  let mut q = GetUsersQuery::new();
  if let Some(un) = m.value_of("username") {
    q = q.username(un);
  }

  if m.is_present("active") {
    q = q.state(UserState::Active);
  } else if m.is_present("blocked") {
    q = q.state(UserState::Blocked);
  }

  Args::LsUsers(q)
}

fn handle_ls_branches<'a>(m: &'a ArgMatches) -> Args<'a> {
  let mut q = GetBranchesQuery::new();
  if let Some(un) = m.value_of("search") {
    q = q.search(un);
  }
  Args::LsBranches {
    query: q,
    project: m.value_of("project"),
  }
}

fn handle_ls_mr<'a>(m: &'a ArgMatches) -> Args<'a> {
  let mut q = GetMergeRequestsQuery::new();
  if let Some(s) = m.value_of("state") {
    if let Ok(v) = MRState::from_str(s) {
      q = q.state(v);
    }
  }
  if let Some(s) = m.value_of("scope") {
    if let Ok(v) = MRScope::from_str(s) {
      q = q.scope(v);
    }
  }
  if let Some(un) = m.value_of("search") {
    q = q.search(un);
  }
  if let Some(v) = m.value_of("target-branch") {
    q = q.target_branch(v);
  }
  if let Some(v) = m.value_of("source-branch") {
    q = q.source_branch(v);
  }
  if let Some(v) = m.value_of("author-id") {
    if let Ok(n) = v.parse() {
      q = q.author_id(n);
    }
  }
  if let Some(v) = m.value_of("assignee-id") {
    if let Ok(n) = v.parse() {
      q = q.assignee_id(n);
    }
  }

  Args::LsMr {
    project: m.value_of("project"),
    query: q,
  }
}
