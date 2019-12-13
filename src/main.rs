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

use args::{parse_args, Args};
use clap::{App, ArgMatches};
use configs::Configs;
use prettytable::Table;

use api::{Branch, GLApi, MergeRequest, Project, ReqParams, User};

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
      ls_users(&users);
    }
    Args::LsProjects(q) => {
      let projects = gl.get_projects(&q)?;
      ls_projects(&projects);
    }
    Args::LsBranches { project, query } => {
      let branches = gl.get_project_branches(project, &query)?;
      ls_branches(&branches);
    }
    Args::LsMr{ project, query } => {
      let mrs = gl.get_project_merge_requests(project, &query)?;
      ls_mrs(&mrs);
    }
    Args::CreateMR(args_matches) => {
      let project = gl.req_params.get_default_project_checked()?;
      let create_mr_data = create_mr::fill_mr_create_data(&gl, project, &args_matches);
      create_mr::confirm_mr(&create_mr_data, &args_matches);
      let mr = gl.create_merge_request(project, &create_mr_data)?;
      create_mr::log_new_mr(&mr);
    }

    Args::CfgSaveToken { token, .. /* file_path */ } => {
      configs.save_new_token(token)?;
    }
    Args::CfgShowToken => {
      let msg = "Private token is";
      if let Some(glob) = configs.global {
        println!("{} `{}`", msg, glob.private_token);
      } else {
        println!("{} None", msg);
      }
    }
    Args::CfgForgetToken => {
      configs.remove_global_cfg()?;
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

fn ls_users(users: &[User]) {
  let mut table = Table::new();

  table.add_row(row!["ID", "USERNAME", "NAME", "STATE"]);
  for u in users {
    table.add_row(row![u.id, u.username, u.name, u.state]);
  }
  println!("Users ({}):", users.len());
  table.printstd();
}

fn ls_projects(projects: &[Project]) {
  let mut table = Table::new();

  table.add_row(row!["ID", "NAME", "DESC", "DEF_BRANCH"]);
  for p in projects {
    let desc: &str = p
      .description
      .as_ref()
      .map(|s| s.as_str())
      .unwrap_or_default();
    table.add_row(row![p.id, p.name, desc, p.default_branch]);
  }
  println!("Projects ({}):", projects.len());
  table.printstd();
}

fn ls_branches(branches: &[Branch]) {
  let mut table = Table::new();

  table.add_row(row!["SHA", "NAME", "AUTHOR"]);
  for b in branches {
    table.add_row(row![b.commit.short_id, b.name, b.commit.author_name]);
  }
  println!("Branches ({}):", branches.len());
  table.printstd();
}

fn ls_mrs(mrs: &[MergeRequest]) {
  let mut table = Table::new();

  table.add_row(row![
    "ID",
    "AUTHOR",
    "ASSIGNEE",
    "STATE",
    "SOURCE_BRANCH",
    "TARGET_BRANCH"
  ]);
  for mr in mrs {
    let assignee = if let Some(u) = &mr.assignee {
      &u.username
    } else {
      ""
    };

    table.add_row(row![
      mr.id,
      mr.author.username,
      assignee,
      mr.state,
      mr.source_branch,
      mr.target_branch
    ]);
  }
  println!("Merge requests ({}):", mrs.len());
  table.printstd();
}
