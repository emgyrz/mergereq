#[derive(Debug, Fail)]
pub enum GLApiError {
  #[fail(display = "{}", 0)]
  Reqwest(reqwest::Error),
  #[fail(display = "private token is required but not specified")]
  NoPrivateToken,
  #[fail(display = "repo_url is required but not specified")]
  NoRepoUrl,
  #[fail(display = "project is required but not specified")]
  NoProject,
  #[fail(display = "cannot parse response: {:?}", resp_text)]
  CantParseResp { resp_text: String },
  #[fail(display = "API Error: {}", error)]
  APIErr { error: String },
  #[fail(display = "Parse error: {}", 0)]
  ParseError(String),
  #[fail(display = "Cannot read config file")]
  ReadCfgError,
}

impl From<reqwest::Error> for GLApiError {
  fn from(re: reqwest::Error) -> Self {
    GLApiError::Reqwest(re)
  }
}

impl From<APIErr> for GLApiError {
  fn from(ae: APIErr) -> Self {
    let error = ae.error;
    GLApiError::APIErr { error }
  }
}

#[derive(Deserialize)]
pub struct APIErr {
  error: String,
}

// impl fmt::Display for APIErr {
//   fn display(f: fmt::Formatter, )
// }
