#[macro_use]
extern crate failure;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate prettytable;

mod api;
mod args;
mod configs;
mod create_mr;
mod helpers;
mod ls;

use args::{parse_args, Args};
use clap::{App, ArgMatches};
use configs::{CfgVariant, Configs};

use api::{GLApi, ReqParams};

fn main() {
  if let Err(err) = run() {
    eprintln!("[ERROR] {}", err);
    std::process::exit(1);
  }
  // println!("Done");
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
  let yaml = load_yaml!("args/cfg.yml");
  let matches = App::from_yaml(yaml).get_matches();

  let mut configs = Configs::read(
    matches.value_of("global-config"),
    matches.value_of("local-config"),
  )?;

  let arg = parse_args(&matches);

  let req_params = collect_req_params(&matches, &configs);

  let gl = GLApi::init(req_params);

  match arg {
    Args::LsUsers(q) => {
      let users = gl.get_users(&q)?;
      ls::users(&users);
    }
    Args::LsProjects(q) => {
      let projects = gl.get_projects(&q)?;
      ls::projects(&projects);
    }
    Args::LsBranches { project, query } => {
      let branches = gl.get_project_branches(project, &query)?;
      ls::branches(&branches);
    }
    Args::LsMr { project, query } => {
      let mrs = gl.get_project_merge_requests(project, &query)?;
      ls::mrs(&mrs);
    }
    Args::CreateMR(args_matches) => {
      let project = gl.req_params.get_default_project_checked()?;
      let create_mr_data = create_mr::fill_mr_create_data(&gl, project, &args_matches);
      create_mr::confirm_mr(&create_mr_data, &args_matches);
      let mr = gl.create_merge_request(project, &create_mr_data)?;
      create_mr::log_new_mr(&mr);
    }

    Args::CfgSaveToken(token) => {
      configs.save_new_token(token)?;
      ls::save_token(configs.get_file_path(CfgVariant::Global));
    }
    Args::CfgShowToken => {
      let mut tok = None;
      if let Some(glob) = &configs.global {
        tok = Some(&glob.private_token);
      }
      ls::show_token(tok);
    }
    Args::CfgForgetToken => {
      configs.remove_global_cfg()?;
      let glob_cfg_path = configs.get_file_path(CfgVariant::Global);
      ls::forget_token(glob_cfg_path);
    }

    Args::Unknown => {
      eprintln!("arguments is unknown");
    }
  }

  Ok(())
}

fn collect_req_params<'a>(matches: &'a ArgMatches, cfg: &'a Configs) -> ReqParams<'a> {
  let global_ref = cfg.global.as_ref();
  let local_ref = cfg.local.as_ref();

  let token = matches
    .value_of("private-token")
    .or_else(|| global_ref.map(|glob| glob.private_token.as_str()));

  let project = matches
    .value_of("project")
    .or_else(|| local_ref.and_then(|loc| loc.default_project.as_ref().map(String::as_str)));

  let repo_url = matches
    .value_of("repo-url")
    .or_else(|| local_ref.map(|loc| loc.repo_url.as_str()));

  ReqParams {
    private_token: token,
    repo_url,
    default_project: project,
  }
}
