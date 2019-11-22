use percent_encoding::{percent_encode, NON_ALPHANUMERIC};

pub fn url_all(project_id: &str) -> String {
  let id = percent_encode(project_id.as_bytes(), NON_ALPHANUMERIC);
  format!("/projects/{}/repository/branches", id)
}

#[derive(Debug, Deserialize)]
pub struct Commit {
  pub author_email: String,
  pub author_name: String,
  pub authored_date: String,
  pub id: String,
  pub short_id: String,
  pub title: String,
  pub message: String,
  pub parent_ids: Option<Vec<String>>,
  //   "author_email": "john@example.com",
  //   "author_name": "John Smith",
  //   "authored_date": "2012-06-27T05:51:39-07:00",
  //   "committed_date": "2012-06-28T03:44:20-07:00",
  //   "committer_email": "john@example.com",
  //   "committer_name": "John Smith",
  //   "id": "7b5c3cc8be40ee161ae89a06bba6229da1032a0c",
  //   "short_id": "7b5c3cc",
  //   "title": "add projects API",
  //   "message": "add projects API",
  //   "parent_ids": [
  //     "4ad91d3c1144c406e50c7b33bae684bd6837faf8"
  //   ]
}

#[derive(Debug, Deserialize)]
pub struct Branch {
  pub name: String,
  pub merge: Option<bool>,
  pub protected: bool,
  pub default: bool,
  pub developers_can_push: bool,
  pub developers_can_merge: bool,
  pub can_push: bool,
  pub commit: Commit,
  // "name": "master",
  // "merged": false,
  // "protected": true,
  // "default": true,
  // "developers_can_push": false,
  // "developers_can_merge": false,
  // "can_push": true,
  // "commit": {
  //   ...
  // }
}

#[derive(Debug, Default, Serialize)]
pub struct GetBranchesQuery<'a> {
  search: Option<&'a str>,
  per_page: u8,
}

impl<'a> GetBranchesQuery<'a> {
  pub fn new() -> Self {
    let mut q = GetBranchesQuery::default();
    q.per_page = 100;
    q
  }
  pub fn search(mut self, val: &'a str) -> Self {
    self.search = Some(val);
    self
  }
}
