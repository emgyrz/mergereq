use clap::{App, Arg};

enum ArgsName {
  PrivateToken,
  GlobalConfig,
  LocalConfig,
  Project,
  RepoUrl,
  Config,
  SaveToken,
  ShowToken,
  ForgetToken,
  Create,
  Mr,
  Title,
  SourceBranch,
  TargetBranch,
  AssigneeId,
  AssigneeName,
  Description,
  RemoveSourceBranch,
  Squash,
}

impl<'a> From<ArgsName> for &'a str {
  fn from(arg_name: ArgsName) -> &'a str {
    match arg_name {
      ArgsName::PrivateToken => "private-token",
      ArgsName::GlobalConfig => "global-config",
      ArgsName::LocalConfig => "local-config",
      ArgsName::Project => "project",
      ArgsName::RepoUrl => "repo-url",
      ArgsName::Config => "config",
      ArgsName::SaveToken => "save-token",
      ArgsName::ShowToken => "show-token",
      ArgsName::ForgetToken => "forget-token",
      ArgsName::Create => "create",
      ArgsName::Mr => "mr",
      ArgsName::Title => "title",
      ArgsName::SourceBranch => "source-branch",
      ArgsName::TargetBranch => "target-branch",
      ArgsName::AssigneeId => "assignee-id",
      ArgsName::AssigneeName => "assignee-name",
      ArgsName::Description => "description",
      ArgsName::RemoveSourceBranch => "remove-source-branch",
      ArgsName::Squash => "squash",
    }
  }
}

fn get_global_args<'a, 'b>() -> Vec<Arg<'a, 'b>> {
  vec![
    Arg::with_name(ArgsName::GlobalConfig.into())
      .help("Path of global config file. e.g. for Linux it would be `~/.config/.mergereq-config`")
      .global(true)
      .takes_value(true),
    Arg::with_name(ArgsName::LocalConfig.into())
      .help("Path of local config file. Default is `.mergereqrc.toml` in the current directory.")
      .global(true)
      .takes_value(true),
    Arg::with_name(ArgsName::Project.into())
      .short("P")
      .help("The ID or path of the project owned by the authenticated user")
      .global(true)
      .takes_value(true),
    Arg::with_name(ArgsName::RepoUrl.into())
      .help("URL of your Gitlab domain")
      .global(true)
      .takes_value(true),
  ]
}

pub fn get_matches() {
  let matches = App::new("mergereq")
    .version(crate_version!())
    .author(crate_authors!())
    .args(&get_global_args());

  // .subcommands(vec![

  // ]);
}
