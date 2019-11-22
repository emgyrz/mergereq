use gitlabapi::{GetBranchesQuery, GetMergeRequestsQuery, GetProjectsQuery, GetUsersQuery};

use super::create_mr::CreateMRArgsData;

#[derive(Debug)]
pub enum Args<'a> {
  LsUsers(GetUsersQuery<'a>),
  LsProjects(GetProjectsQuery<'a>),
  LsBranches {
    query: GetBranchesQuery<'a>,
    project: Option<&'a str>,
  },
  LsMr(GetMergeRequestsQuery<'a>),
  CreateMR(CreateMRArgsData<'a>),
  CfgSaveToken {
    token: &'a str,
    file_path: Option<&'a str>,
  },
  CfgShowToken {
    file_path: Option<&'a str>,
  },
  Unknown,
}
