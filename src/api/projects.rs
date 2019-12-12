use crate::api::{utils, GLApiError};
use std::fmt;
use std::str::FromStr;

use serde::ser::{Serialize, Serializer};

const VIS_PUBLIC: &str = "public";
const VIS_INTERNAL: &str = "internal";
const VIS_PRIVATE: &str = "private";

pub fn url_all() -> &'static str {
  "/projects"
}

pub fn url_one(project: &str) -> String {
  let mut u = "/projects/".to_owned();
  let id = utils::encode(project).to_string();
  u.push_str(&id);
  u
}

#[derive(Debug, Deserialize)]
pub struct Project {
  pub id: u32,
  pub description: Option<String>,
  pub default_branch: String,
  pub ssh_url_to_repo: String,
  pub http_url_to_repo: String,
  pub web_url: String,
  pub readme_url: Option<String>,
  pub name: String,
  pub path: String,
  // "id": 4,
  // "description": null,
  // "default_branch": "master",
  // "ssh_url_to_repo": "git@example.com:diaspora/diaspora-client.git",
  // "http_url_to_repo": "http://example.com/diaspora/diaspora-client.git",
  // "web_url": "http://example.com/diaspora/diaspora-client",
  // "readme_url": "http://example.com/diaspora/diaspora-client/blob/master/README.md",
  // "tag_list": [
  //   "example",
  //   "disapora client"
  // ],
  // "name": "Diaspora Client",
  // "name_with_namespace": "Diaspora / Diaspora Client",
  // "path": "diaspora-client",
  // "path_with_namespace": "diaspora/diaspora-client",
  // "created_at": "2013-09-30T13:46:02Z",
  // "last_activity_at": "2013-09-30T13:46:02Z",
  // "forks_count": 0,
  // "avatar_url": "http://example.com/uploads/project/avatar/4/uploads/avatar.png",
  // "star_count": 0,
}

#[derive(Copy, Clone, Debug)]
pub enum ProjectVisibility {
  Public,
  Internal,
  Private,
}

impl FromStr for ProjectVisibility {
  type Err = GLApiError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      VIS_PUBLIC => Ok(ProjectVisibility::Public),
      VIS_INTERNAL => Ok(ProjectVisibility::Internal),
      VIS_PRIVATE => Ok(ProjectVisibility::Private),
      _ => Err(GLApiError::ParseError(s.to_owned())),
    }
  }
}

impl fmt::Display for ProjectVisibility {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let s = match self {
      ProjectVisibility::Public => VIS_PUBLIC,
      ProjectVisibility::Internal => VIS_INTERNAL,
      ProjectVisibility::Private => VIS_PRIVATE,
    };
    write!(f, "{}", s)
  }
}

impl Serialize for ProjectVisibility {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_str(&self.to_string())
  }
}

#[derive(Default, Serialize, Debug)]
pub struct GetProjectsQuery<'a> {
  archived: Option<bool>,
  visibility: Option<ProjectVisibility>,
  search: Option<&'a str>,
  owned: Option<bool>,
  membership: Option<bool>,
  // archived  boolean   no  Limit by archived status
  // visibility  string  no  Limit by visibility public, internal, or private
  // order_by  string  no  Return projects ordered by id, name, path, created_at, updated_at, or last_activity_at fields. Default is created_at
  // sort  string  no  Return projects sorted in asc or desc order. Default is desc
  // search  string  no  Return list of projects matching the search criteria
  // simple  boolean   no  Return only limited fields for each project. This is a no-op without authentication as then only simple fields are returned.
  // owned   boolean   no  Limit by projects explicitly owned by the current user
  // membership  boolean   no  Limit by projects that the current user is a member of
  // starred   boolean   no  Limit by projects starred by the current user
  // statistics  boolean   no  Include project statistics
  // with_custom_attributes  boolean   no  Include custom attributes in response (admins only)
  // with_issues_enabled   boolean   no  Limit by enabled issues feature
  // with_merge_requests_enabled   boolean   no  Limit by enabled merge requests feature
  // with_programming_language   string  no  Limit by projects which use the given programming language
  // wiki_checksum_failed  boolean   no  Limit projects where the wiki checksum calculation has failed (Introduced in GitLab Premium 11.2)
  // repository_checksum_failed  boolean   no  Limit projects where the repository checksum calculation has failed (Introduced in GitLab Premium 11.2)
  // min_access_level  integer   no  Limit by current user minimal access level
}

impl<'a> GetProjectsQuery<'a> {
  pub fn new() -> Self {
    GetProjectsQuery::default()
  }
  pub fn archived(mut self, val: bool) -> Self {
    self.archived = Some(val);
    self
  }
  pub fn visibility(mut self, val: ProjectVisibility) -> Self {
    self.visibility = Some(val);
    self
  }
  pub fn search(mut self, val: &'a str) -> Self {
    self.search = Some(val);
    self
  }
  pub fn owned(mut self, val: bool) -> Self {
    self.owned = Some(val);
    self
  }
  pub fn membership(mut self, val: bool) -> Self {
    self.membership = Some(val);
    self
  }
}
