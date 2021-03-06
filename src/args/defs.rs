use crate::api::{GetBranchesQuery, GetMergeRequestsQuery, GetProjectsQuery, GetUsersQuery};
use clap::ArgMatches;

pub enum Args<'a> {
  LsUsers(GetUsersQuery<'a>),
  LsProjects(GetProjectsQuery<'a>),
  LsBranches {
    query: GetBranchesQuery<'a>,
    project: Option<&'a str>,
  },
  LsMr {
    project: Option<&'a str>,
    query: GetMergeRequestsQuery<'a>,
  },
  CreateMR(&'a ArgMatches<'a>),
  CfgSaveToken(&'a str),
  CfgShowToken,
  CfgForgetToken,
  Unknown,
}
