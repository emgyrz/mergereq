use crate::api::GLApiError;
use std::fmt;
use std::str::FromStr;

use serde::ser::{Serialize, SerializeStruct, Serializer};

pub fn url_all() -> &'static str {
  "/users"
}

const STATE_ACTIVE: &str = "active";
const STATE_BLOCKED: &str = "blocked";

#[derive(Debug, Deserialize)]
pub struct User {
  // {
  //     "id": 1,
  //     "username": "john_smith",
  //     "name": "John Smith",
  //     "state": "active",
  //     "avatar_url": "http://localhost:3000/uploads/user/avatar/1/cd8.jpeg",
  //     "web_url": "http://localhost:3000/john_smith"
  //   },
  pub id: u32,
  pub username: String,
  pub name: String,
  pub state: String,
}

#[derive(Debug, Deserialize)]
pub enum UserState {
  Active,
  Blocked,
}

impl UserState {
  pub fn to_str(&self) -> &'static str {
    match self {
      UserState::Active => STATE_ACTIVE,
      UserState::Blocked => STATE_BLOCKED,
    }
  }
}

impl FromStr for UserState {
  type Err = GLApiError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      STATE_ACTIVE => Ok(UserState::Active),
      STATE_BLOCKED => Ok(UserState::Blocked),
      _ => Err(GLApiError::ParseError(s.to_owned())),
    }
  }
}

impl fmt::Display for UserState {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.to_str())
  }
}

impl Serialize for UserState {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_str(&self.to_string())
  }
}

#[derive(Debug, Default)]
pub struct GetUsersQuery<'a> {
  username: Option<&'a str>,
  state: Option<UserState>,
  per_page: u8,
}

impl<'a> GetUsersQuery<'a> {
  pub fn new() -> Self {
    let mut q = GetUsersQuery::default();
    q.per_page = 100;
    q
  }

  pub fn username(mut self, val: &'a str) -> Self {
    self.username = Some(val);
    self
  }
  pub fn state(mut self, val: UserState) -> Self {
    self.state = Some(val);
    self
  }
}

impl<'a> Serialize for GetUsersQuery<'a> {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let mut state = serializer.serialize_struct("GetUsersQuery", 2)?;
    state.serialize_field("per_page", &self.per_page.to_string())?;
    if let Some(un) = self.username {
      state.serialize_field("username", un)?;
    }
    if let Some(st) = &self.state {
      state.serialize_field(st.to_str(), "true")?;
    }

    state.end()
  }
}

// struct User {
//   "id": 1,
//   "username": "john_smith",
//   "name": "John Smith",
//   "state": "active",
//   "avatar_url": "http://localhost:3000/uploads/user/avatar/1/cd8.jpeg",
//   "web_url": "http://localhost:3000/john_smith",
//   "created_at": "2012-05-23T08:00:58Z",
//   "bio": null,
//   "location": null,
//   "public_email": "john@example.com",
//   "skype": "",
//   "linkedin": "",
//   "twitter": "",
//   "website_url": "",
//   "organization": ""
// }
