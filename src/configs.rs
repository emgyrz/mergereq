use directories::BaseDirs;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::fs;

use crate::api::{GLApiError, GLApiResult};

static GLOBAL_FILE_NAME: &str = ".mergereq-config";
static LOCAL_FILE_NAME: &str = ".mergereqrc.toml";

pub enum CfgVariant {
  Global,
  #[allow(dead_code)]
  Local,
}

fn default_global_file_path() -> GLApiResult<String> {
  let path = if let Some(bd) = BaseDirs::new() {
    bd.config_dir()
      .join(GLOBAL_FILE_NAME)
      .to_string_lossy()
      .to_string()
  } else {
    return Err(GLApiError::ReadCfgError.into());
  };
  Ok(path)
}

fn default_local_file_path() -> GLApiResult<String> {
  let path = if let Ok(pwd) = std::env::current_dir() {
    pwd.join(LOCAL_FILE_NAME).to_string_lossy().to_string()
  } else {
    return Err(GLApiError::ReadCfgError.into());
  };
  Ok(path)
}

#[derive(Serialize, Deserialize)]
pub struct GlobalData {
  pub private_token: String,
}

#[derive(Serialize, Deserialize)]
pub struct LocalData {
  pub repo_url: String,
  pub default_project: Option<String>,
}

pub struct Configs {
  pub global: Option<GlobalData>,
  pub local: Option<LocalData>,

  global_file_path: String,
  local_file_path: String,
}

impl Configs {
  pub fn read(global_cfg_path: Option<&str>, local_cfg_path: Option<&str>) -> GLApiResult<Self> {
    let (global, gpath) = if let Some(g) = global_cfg_path {
      let data: GlobalData = Configs::parse_from_file(g)?;
      (Some(data), g.to_owned())
    } else {
      let path = default_global_file_path()?;
      (Configs::parse_from_file(&path).ok(), path)
    };

    let (local, lpath) = if let Some(l) = local_cfg_path {
      let data: LocalData = Configs::parse_from_file(l)?;
      (Some(data), l.to_owned())
    } else {
      let path = default_local_file_path()?;
      (Configs::parse_from_file(&path).ok(), path)
    };

    Ok(Configs {
      global_file_path: gpath,
      local_file_path: lpath,
      global,
      local,
    })
  }

  fn parse_from_file<T: DeserializeOwned>(path: &str) -> GLApiResult<T> {
    let cfg_str = fs::read_to_string(&path)?;
    let data = toml::from_str(&cfg_str)?;
    Ok(data)
  }

  pub fn get_file_path(&self, cfg_variant: CfgVariant) -> &str {
    match cfg_variant {
      CfgVariant::Global => &self.global_file_path,
      CfgVariant::Local => &self.local_file_path,
    }
  }

  fn store(&self, variant: CfgVariant) -> GLApiResult<()> {
    let data_bytes = match variant {
      CfgVariant::Global => toml::ser::to_vec(&self.global)?,
      CfgVariant::Local => toml::ser::to_vec(&self.local)?,
    };
    fs::write(self.get_file_path(variant), data_bytes)?;
    Ok(())
  }

  pub fn save_new_token(&mut self, token: &str) -> GLApiResult<()> {
    if self.global.is_none() {
      self.global = Some(GlobalData {
        private_token: String::new(),
      });
    }

    let global = self.global.as_mut().unwrap();
    global.set_token(token);
    self.store(CfgVariant::Global)
  }

  pub fn remove_global_cfg(&mut self) -> GLApiResult<()> {
    let path = self.get_file_path(CfgVariant::Global);
    fs::remove_file(path)?;
    Ok(())
  }
}

impl GlobalData {
  fn set_token(&mut self, token: &str) {
    self.private_token = token.to_owned();
  }
}

// #[test]
// fn read_cfg() -> GLApiResult<()> {
//   let mut cfg = Configs::new(None, None)?;
//   cfg.read_global();
//   cfg.read_local();
//   println!("{:#?}", cfg);
//   Ok(())
// }
