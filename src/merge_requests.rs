use std::fmt;
use std::str::FromStr;

use percent_encoding::{percent_encode, NON_ALPHANUMERIC};

use crate::{users::User, GLApiError};

use serde::{Serialize, Serializer};

const SCOPE_CREATED_BY_ME: &str = "created_by_me";
const SCOPE_ASSIGNED_TO_ME: &str = "assigned_by_me";
const SCOPE_ALL: &str = "all";

const STATE_OPENED: &str = "opened";
const STATE_CLOSED: &str = "closed";
const STATE_LOCKED: &str = "locked";
const STATE_MERGED: &str = "merged";

pub fn url_all() -> &'static str {
  "/merge_requests"
}

pub fn url_project_mr(project_id: &str) -> String {
  let mut u = "/projects/".to_owned();
  let id = percent_encode(project_id.as_bytes(), NON_ALPHANUMERIC);
  u.push_str(&id.to_string());
  u.push_str("/merge_requests");
  u
}

// {
//     "id": 1,
//     "iid": 1,
//     "project_id": 3,
//     "title": "test1",
//     "description": "fixed login page css paddings",
//     "state": "merged",
//     "merged_by": {
//       "id": 87854,
//       "name": "Douwe Maan",
//       "username": "DouweM",
//       "state": "active",
//       "avatar_url": "https://gitlab.example.com/uploads/-/system/user/avatar/87854/avatar.png",
//       "web_url": "https://gitlab.com/DouweM"
//     },
//     "merged_at": "2018-09-07T11:16:17.520Z",
//     "closed_by": null,
//     "closed_at": null,
//     "created_at": "2017-04-29T08:46:00Z",
//     "updated_at": "2017-04-29T08:46:00Z",
//     "target_branch": "master",
//     "source_branch": "test1",
//     "upvotes": 0,
//     "downvotes": 0,
//     "author": {
//       "id": 1,
//       "name": "Administrator",
//       "username": "admin",
//       "state": "active",
//       "avatar_url": null,
//       "web_url" : "https://gitlab.example.com/admin"
//     },
//     "assignee": {
//       "id": 1,
//       "name": "Administrator",
//       "username": "admin",
//       "state": "active",
//       "avatar_url": null,
//       "web_url" : "https://gitlab.example.com/admin"
//     },
//     "assignees": [{
//       "name": "Miss Monserrate Beier",
//       "username": "axel.block",
//       "id": 12,
//       "state": "active",
//       "avatar_url": "http://www.gravatar.com/avatar/46f6f7dc858ada7be1853f7fb96e81da?s=80&d=identicon",
//       "web_url": "https://gitlab.example.com/axel.block"
//     }],
//     "source_project_id": 2,
//     "target_project_id": 3,
//     "labels": [
//       "Community contribution",
//       "Manage"
//     ],
//     "work_in_progress": false,
//     "milestone": {
//       "id": 5,
//       "iid": 1,
//       "project_id": 3,
//       "title": "v2.0",
//       "description": "Assumenda aut placeat expedita exercitationem labore sunt enim earum.",
//       "state": "closed",
//       "created_at": "2015-02-02T19:49:26.013Z",
//       "updated_at": "2015-02-02T19:49:26.013Z",
//       "due_date": "2018-09-22",
//       "start_date": "2018-08-08",
//       "web_url": "https://gitlab.example.com/my-group/my-project/milestones/1"
//     },
//     "merge_when_pipeline_succeeds": true,
//     "merge_status": "can_be_merged",
//     "sha": "8888888888888888888888888888888888888888",
//     "merge_commit_sha": null,
//     "user_notes_count": 1,
//     "discussion_locked": null,
//     "should_remove_source_branch": true,
//     "force_remove_source_branch": false,
//     "allow_collaboration": false,
//     "allow_maintainer_to_push": false,
//     "web_url": "http://gitlab.example.com/my-group/my-project/merge_requests/1",
//     "time_stats": {
//       "time_estimate": 0,
//       "total_time_spent": 0,
//       "human_time_estimate": null,
//       "human_total_time_spent": null
//     },
//     "squash": false,
//     "task_completion_status":{
//       "count":0,
//       "completed_count":0
//     }
//   }

#[derive(Debug, Deserialize)]
pub struct MergeRequest {
  pub id: u32,
  pub iid: u32,
  pub project_id: u32,
  pub title: String,
  pub description: Option<String>,
  pub state: String,
  pub merged_by: Option<User>,
  pub merged_at: Option<String>,
  pub closed_by: Option<User>,
  pub closed_at: Option<String>,
  pub created_at: String,
  pub updated_at: Option<String>,
  pub target_branch: String,
  pub source_branch: String,
  pub author: User,
  pub assignee: Option<User>,
  pub source_project_id: u32,
  pub target_project_id: u32,
  pub work_in_progress: bool,
  pub merge_when_pipeline_succeeds: bool,
  pub merge_status: String,
  pub sha: Option<String>,
  pub merge_commit_sha: Option<String>,
  // pub should_remove_source_branch: bool,
  // pub force_remove_source_branch: bool,
  // pub allow_collaboration: bool,
  // pub allow_maintainer_to_push: bool,
  pub web_url: String,
}

#[derive(Debug)]
pub enum MRState {
  Opened,
  Closed,
  Locked,
  Merged,
}

impl FromStr for MRState {
  type Err = GLApiError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      STATE_OPENED => Ok(MRState::Opened),
      STATE_CLOSED => Ok(MRState::Closed),
      STATE_LOCKED => Ok(MRState::Locked),
      STATE_MERGED => Ok(MRState::Merged),
      _ => Err(GLApiError::ParseError(s.to_owned())),
    }
  }
}

impl fmt::Display for MRState {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let s = format!("{:?}", self);
    write!(f, "{}", s.to_lowercase())
  }
}

impl Serialize for MRState {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_str(&self.to_string())
  }
}

#[derive(Debug)]
pub enum MRScope {
  CreatedByMe,
  AssignedToMe,
  All,
}

impl FromStr for MRScope {
  type Err = GLApiError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      SCOPE_CREATED_BY_ME => Ok(MRScope::CreatedByMe),
      SCOPE_ASSIGNED_TO_ME => Ok(MRScope::AssignedToMe),
      SCOPE_ALL => Ok(MRScope::All),
      _ => Err(GLApiError::ParseError(s.to_owned())),
    }
  }
}

impl fmt::Display for MRScope {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let s = match self {
      MRScope::CreatedByMe => SCOPE_CREATED_BY_ME,
      MRScope::AssignedToMe => SCOPE_ASSIGNED_TO_ME,
      MRScope::All => SCOPE_ALL,
    };
    write!(f, "{}", s)
  }
}

impl Serialize for MRScope {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_str(&self.to_string())
  }
}

#[derive(Default, Debug, Serialize)]
pub struct GetMergeRequestsQuery<'a> {
  // state   string  no  Return all merge requests or just those that are opened, closed, locked, or merged
  // order_by  string  no  Return requests ordered by created_at or updated_at fields. Default is created_at
  // sort  string  no  Return requests sorted in asc or desc order. Default is desc
  // milestone   string  no  Return merge requests for a specific milestone. None returns merge requests with no milestone. Any returns merge requests that have an assigned milestone.
  // view  string  no  If simple, returns the iid, URL, title, description, and basic state of merge request
  // labels  string  no  Return merge requests matching a comma separated list of labels. None lists all merge requests with no labels. Any lists all merge requests with at least one label. No+Label (Deprecated) lists all merge requests with no labels. Predefined names are case-insensitive.
  // created_after   datetime  no  Return merge requests created on or after the given time
  // created_before  datetime  no  Return merge requests created on or before the given time
  // updated_after   datetime  no  Return merge requests updated on or after the given time
  // updated_before  datetime  no  Return merge requests updated on or before the given time
  // scope   string  no  Return merge requests for the given scope: created_by_me, assigned_to_me or all. Defaults to created_by_me
  // For versions before 11.0, use the now deprecated created-by-me or assigned-to-me scopes instead.
  // author_id   integer   no  Returns merge requests created by the given user id. Combine with scope=all or scope=assigned_to_me
  // assignee_id   integer   no  Returns merge requests assigned to the given user id. None returns unassigned merge requests. Any returns merge requests with an assignee.
  // approver_ids  integer array   no  Returns merge requests which have specified all the users with the given ids as individual approvers. None returns merge requests without approvers. Any returns merge requests with an approver.
  // my_reaction_emoji   string  no  Return merge requests reacted by the authenticated user by the given emoji. None returns issues not given a reaction. Any returns issues given at least one reaction. (Introduced in GitLab 10.0)
  // source_branch   string  no  Return merge requests with the given source branch
  // target_branch   string  no  Return merge requests with the given target branch
  // search  string  no  Search merge requests against their title and description
  // in  string  no  Modify the scope of the search attribute. title, description, or a string joining them with comma. Default is title,description
  // wip   string  no  Filter merge requests against their wip status. yes to return only WIP merge requests, no to return non WIP merge requests
  state: Option<MRState>,
  scope: Option<MRScope>,
  author_id: Option<u32>,
  assignee_id: Option<u32>,
  source_branch: Option<&'a str>,
  target_branch: Option<&'a str>,
  search: Option<&'a str>,
  // wip: Option<bool>,
}

impl<'a> GetMergeRequestsQuery<'a> {
  pub fn new() -> Self {
    GetMergeRequestsQuery::default()
  }

  pub fn state(mut self, val: MRState) -> Self {
    self.state = Some(val);
    self
  }
  pub fn scope(mut self, val: MRScope) -> Self {
    self.scope = Some(val);
    self
  }
  pub fn author_id(mut self, val: u32) -> Self {
    self.author_id = Some(val);
    self
  }
  pub fn assignee_id(mut self, val: u32) -> Self {
    self.assignee_id = Some(val);
    self
  }
  pub fn source_branch(mut self, val: &'a str) -> Self {
    self.source_branch = Some(val);
    self
  }
  pub fn target_branch(mut self, val: &'a str) -> Self {
    self.target_branch = Some(val);
    self
  }
  pub fn search(mut self, val: &'a str) -> Self {
    self.search = Some(val);
    self
  }
}

#[derive(Debug, Default, Serialize)]
pub struct CreateMRBody<'a> {
  pub project: &'a str,
  pub source_branch: String,
  pub target_branch: String,
  pub title: String,
  pub assignee_id: Option<u32>,
  // pub description: Option<&'a str>,
  // pub remove_source_branch: Option<bool>,
  // pub squash: Option<bool>,
  // id  integer/string  yes   The ID or URL-encoded path of the project owned by the authenticated user
  // source_branch   string  yes   The source branch
  // target_branch   string  yes   The target branch
  // title   string  yes   Title of MR
  // assignee_id   integer   no  Assignee user ID
  // assignee_ids  integer array   no  The ID of the user(s) to assign the MR to. Set to 0 or provide an empty value to unassign all assignees.
  // description   string  no  Description of MR. Limited to 1 000 000 characters.
  // target_project_id   integer   no  The target project (numeric id)
  // labels  string  no  Labels for MR as a comma-separated list
  // milestone_id  integer   no  The global ID of a milestone
  // remove_source_branch  boolean   no  Flag indicating if a merge request should remove the source branch when merging
  // allow_collaboration   boolean   no  Allow commits from members who can merge to the target branch
  // allow_maintainer_to_push  boolean   no  Deprecated, see allow_collaboration
  // squash  boolean   no  Squash commits into a single commit when merging
}
