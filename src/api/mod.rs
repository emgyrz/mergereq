
#[cfg(test)]
mod tests;

mod branches;
mod error;
mod merge_requests;
mod projects;
mod users;

pub use branches::{Branch, Commit, GetBranchesQuery};
pub use merge_requests::{CreateMRBody, GetMergeRequestsQuery, MRScope, MRState, MergeRequest};
pub use projects::{GetProjectsQuery, Project, ProjectVisibility};
pub use users::{GetUsersQuery, User, UserState};

use serde::{de::DeserializeOwned, Serialize};

pub use error::GLApiError;
pub type GLApiResult<T> = Result<T, failure::Error>;

pub struct GLApi<'a> {
  pub req_params: ReqParams<'a>,
}

#[derive(Default, Debug)]
pub struct ReqParams<'a> {
  pub private_token: Option<&'a str>,
  pub repo_url: Option<&'a str>,
  pub default_project: Option<&'a str>,
}

impl<'a> ReqParams<'a> {
  // pub fn new() -> Self {
  //   ReqParams::default()
  // }

  // pub fn set_private_token(&mut self, val: &'a str) {
  //   self.private_token = Some(val);
  // }
  // pub fn set_repo_url(&mut self, val: &'a str) {
  //   self.repo_url = Some(val);
  // }
  // pub fn set_default_project(&mut self, val: &'a str) {
  //   self.default_project = Some(val);
  // }

  pub fn get_private_token_checked(&self) -> GLApiResult<&str> {
    self
      .private_token
      .ok_or_else(|| GLApiError::NoPrivateToken.into())
  }
  pub fn get_repo_url_checked(&self) -> GLApiResult<&str> {
    self.repo_url.ok_or_else(|| GLApiError::NoRepoUrl.into())
  }
  pub fn get_default_project_checked(&self) -> GLApiResult<&str> {
    self
      .default_project
      .ok_or_else(|| GLApiError::NoProject.into())
  }
}

impl<'a> GLApi<'a> {
  pub fn init(req_params: ReqParams<'a>) -> Self {
    GLApi { req_params }
  }

  fn gen_url(&self, url_part: &str) -> GLApiResult<String> {
    let mut s = self.req_params.get_repo_url_checked()?.to_owned();
    s.push_str("/api/v4");
    s.push_str(url_part);
    Ok(s)
  }

  pub fn get<T, Q>(&self, url: &str, query: Option<&Q>) -> GLApiResult<T>
  where
    T: DeserializeOwned,
    Q: Serialize + ?Sized,
  {
    let token = self.req_params.get_private_token_checked()?;
    let full_url = self.gen_url(url)?;

    // println!("full_url {:?}", full_url);
    // println!("query {:?}", query.serialize());

    let mut req = reqwest::Client::new()
      .get(&full_url)
      .header("PRIVATE-TOKEN", token);
    if let Some(q) = query {
      req = req.query(q);
    }
    let mut req = req.send()?;

    let resp = req.text()?;

    if let Ok(result) = serde_json::from_str(&resp) {
      return Ok(result);
    }

    println!("ERR>>>{:?}", resp);
    if let Ok(api_err) = serde_json::from_str::<error::APIErr>(&resp) {
      let e: GLApiError = api_err.into();
      return Err(e.into());
    }

    Err(
      GLApiError::CantParseResp {
        resp_text: req.text()?,
      }
      .into(),
    )
  }

  pub fn post<T, B>(&self, url: &str, body: Option<&B>) -> GLApiResult<T>
  where
    T: DeserializeOwned,
    B: Serialize + ?Sized,
  {
    let token = self.req_params.get_private_token_checked()?;
    let full_url = self.gen_url(url)?;
    // println!("full_url {:?}", full_url);

    let mut req = reqwest::Client::new()
      .post(&full_url)
      .header("PRIVATE-TOKEN", token);

    if let Some(b) = body {
      req = req.json(b);
    }

    let mut req = req.send()?;

    let resp = req.text()?;

    match serde_json::from_str(&resp) {
      Ok(r) => return Ok(r),
      Err(e) => println!("{:?}", e),
    }

    println!("ERR>>>{:?}", resp);
    if let Ok(api_err) = serde_json::from_str::<error::APIErr>(&resp) {
      let e: GLApiError = api_err.into();
      return Err(e.into());
    }

    Err(
      GLApiError::CantParseResp {
        resp_text: req.text()?,
      }
      .into(),
    )
  }

  pub fn get_project(&self, project: &str) -> GLApiResult<projects::Project> {
    let q: Option<&projects::GetProjectsQuery> = None;
    self.get(&projects::url_one(project), q)
  }

  pub fn get_projects(
    &self,
    query: &projects::GetProjectsQuery,
  ) -> GLApiResult<Vec<projects::Project>> {
    self.get(projects::url_all(), Some(query))
  }
  pub fn get_users(&self, query: &users::GetUsersQuery) -> GLApiResult<Vec<users::User>> {
    self.get(users::url_all(), Some(query))
  }

  // pub fn get_merge_requests(
  //   &self,
  //   query: &merge_requests::GetMergeRequestsQuery,
  // ) -> GLApiResult<Vec<merge_requests::MergeRequest>> {
  //   self.get(merge_requests::url_all(), Some(query))
  // }
  pub fn get_project_merge_requests(
    &self,
    project_id: &str,
    query: &merge_requests::GetMergeRequestsQuery,
  ) -> GLApiResult<Vec<merge_requests::MergeRequest>> {
    self.get(&merge_requests::url_project_mr(project_id), Some(query))
  }
  pub fn create_merge_request(
    &self,
    project_id: &str,
    body: &merge_requests::CreateMRBody,
  ) -> GLApiResult<merge_requests::MergeRequest> {
    self.post(&merge_requests::url_project_mr(project_id), Some(body))
  }
  pub fn get_project_branches(
    &self,
    project_id: &str,
    query: &branches::GetBranchesQuery,
  ) -> GLApiResult<Vec<branches::Branch>> {
    self.get(&branches::url_all(project_id), Some(query))
  }
}
