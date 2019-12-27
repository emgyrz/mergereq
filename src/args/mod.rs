use clap::ArgMatches;
use std::str::FromStr;

use crate::api::{
  GetBranchesQuery, GetMergeRequestsQuery, GetProjectsQuery, GetUsersQuery, MRScope, MRState,
  ProjectVisibility, UserState,
};

mod defs;
mod matches;
mod names;

pub(crate) use defs::Args;
pub(crate) use matches::get_matches;
pub(crate) use names::ArgName;

pub fn parse_args<'a>(arg_matches: &'a ArgMatches) -> Args<'a> {
  if let Some(m) = arg_matches.subcommand_matches(ArgName::Ls) {
    return handle_ls(m);
  } else if let Some(m) = arg_matches.subcommand_matches(ArgName::Create) {
    return handle_create(m);
  } else if let Some(m) = arg_matches.subcommand_matches(ArgName::Config) {
    return handle_config(m);
  }

  Args::Unknown
}

fn handle_ls<'a>(mat: &'a ArgMatches) -> Args<'a> {
  if let Some(m) = mat.subcommand_matches(ArgName::Users) {
    return handle_ls_users(m);
  } else if let Some(m) = mat.subcommand_matches(ArgName::Projects) {
    return handle_ls_projects(m);
  } else if let Some(m) = mat.subcommand_matches(ArgName::Branches) {
    return handle_ls_branches(m);
  } else if let Some(m) = mat.subcommand_matches(ArgName::Mr) {
    return handle_ls_mr(m);
  }

  Args::Unknown
}

fn handle_create<'a>(mat: &'a ArgMatches) -> Args<'a> {
  if let Some(m) = mat.subcommand_matches(ArgName::Mr) {
    return Args::CreateMR(m);
  }
  Args::Unknown
}

fn handle_config<'a>(mat: &'a ArgMatches) -> Args<'a> {
  if let Some(m) = mat.subcommand_matches(ArgName::SaveToken) {
    let token = m.value_of(ArgName::PrivateToken).unwrap();
    return Args::CfgSaveToken(token);
  } else if mat.is_present(ArgName::ShowToken) {
    return Args::CfgShowToken;
  } else if mat.is_present(ArgName::ForgetToken) {
    return Args::CfgForgetToken;
  }
  Args::Unknown
}

fn handle_ls_projects<'a>(m: &'a ArgMatches) -> Args<'a> {
  let mut q = GetProjectsQuery::new();
  if let Some(s) = m.value_of(ArgName::Search) {
    q = q.search(s);
  }

  if let Some(vis) = m.value_of(ArgName::Visibility) {
    if let Ok(v) = ProjectVisibility::from_str(vis) {
      q = q.visibility(v)
    }
  }

  if m.is_present(ArgName::Archived) {
    q = q.archived(true);
  }
  if m.is_present(ArgName::Owned) {
    q = q.owned(true);
  }
  if m.is_present(ArgName::Membership) {
    q = q.membership(true);
  }
  Args::LsProjects(q)
}

fn handle_ls_users<'a>(m: &'a ArgMatches) -> Args<'a> {
  let mut q = GetUsersQuery::new();
  if let Some(un) = m.value_of(ArgName::Username) {
    q = q.username(un);
  }

  if m.is_present(ArgName::Active) {
    q = q.state(UserState::Active);
  } else if m.is_present(ArgName::Blocked) {
    q = q.state(UserState::Blocked);
  }

  Args::LsUsers(q)
}

fn handle_ls_branches<'a>(m: &'a ArgMatches) -> Args<'a> {
  let mut q = GetBranchesQuery::new();
  if let Some(un) = m.value_of(ArgName::Search) {
    q = q.search(un);
  }
  Args::LsBranches {
    query: q,
    project: m.value_of(ArgName::Project),
  }
}

fn handle_ls_mr<'a>(m: &'a ArgMatches) -> Args<'a> {
  let mut q = GetMergeRequestsQuery::new();
  if let Some(s) = m.value_of(ArgName::State) {
    if let Ok(v) = MRState::from_str(s) {
      q = q.state(v);
    }
  }
  if let Some(s) = m.value_of(ArgName::Scope) {
    if let Ok(v) = MRScope::from_str(s) {
      q = q.scope(v);
    }
  }
  if let Some(un) = m.value_of(ArgName::Search) {
    q = q.search(un);
  }
  if let Some(v) = m.value_of(ArgName::TargetBranch) {
    q = q.target_branch(v);
  }
  if let Some(v) = m.value_of(ArgName::SourceBranch) {
    q = q.source_branch(v);
  }
  if let Some(v) = m.value_of(ArgName::AuthorId) {
    if let Ok(n) = v.parse() {
      q = q.author_id(n);
    }
  }
  if let Some(v) = m.value_of(ArgName::AssigneeId) {
    if let Ok(n) = v.parse() {
      q = q.assignee_id(n);
    }
  }

  Args::LsMr {
    project: m.value_of(ArgName::Project),
    query: q,
  }
}
