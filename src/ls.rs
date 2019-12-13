use crate::api::{Branch, MergeRequest, Project, User};
use prettytable::Table;

pub fn users(users: &[User]) {
  let mut table = Table::new();

  table.add_row(row!["ID", "USERNAME", "NAME", "STATE"]);
  for u in users {
    table.add_row(row![u.id, u.username, u.name, u.state]);
  }
  println!("Users ({}):", users.len());
  table.printstd();
}

pub fn projects(projects: &[Project]) {
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

pub fn branches(branches: &[Branch]) {
  let mut table = Table::new();

  table.add_row(row!["SHA", "NAME", "AUTHOR"]);
  for b in branches {
    table.add_row(row![b.commit.short_id, b.name, b.commit.author_name]);
  }
  println!("Branches ({}):", branches.len());
  table.printstd();
}

pub fn mrs(mrs: &[MergeRequest]) {
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


pub fn save_token(config_path: &str) {
  println!("GitLab private token saved to `{}`:", config_path);
}
