use super::ArgName;
use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};

const SUB_CMD_SETTINGS: [AppSettings; 4] = [
  AppSettings::SubcommandRequiredElseHelp,
  AppSettings::VersionlessSubcommands,
  AppSettings::ColoredHelp,
  AppSettings::DisableHelpSubcommand,
];

fn arg_name<'a, 'b, T: Into<&'static str>>(variant: T) -> Arg<'a, 'b> {
  Arg::with_name(variant.into())
}
fn sub_name<'a, 'b, T: Into<&'static str>>(variant: T) -> App<'a, 'b> {
  SubCommand::with_name(variant.into())
}

fn def_arg<'a, 'b, T: Into<&'static str>>(
  variant: T,
  short: &'b str,
  help: &'a str,
) -> Arg<'a, 'b> {
  arg_name(variant).help(help).short(short).takes_value(true)
}

fn def_flag<'a, 'b, T: Into<&'static str>>(
  variant: T,
  short: &'b str,
  help: &'a str,
) -> Arg<'a, 'b> {
  arg_name(variant).help(help).short(short)
}

fn get_global_args<'a, 'b>() -> Vec<Arg<'a, 'b>> {
  vec![
    arg_name(ArgName::GlobalConfig)
      .help("Path of global config file. e.g. for Linux it would be `~/.config/.mergereq-config`")
      .global(true)
      .takes_value(true),
    arg_name(ArgName::LocalConfig)
      .help("Path of local config file. Default is `.mergereqrc.toml` in the current directory.")
      .global(true)
      .takes_value(true),
    arg_name(ArgName::Project)
      .short("P")
      .help("The ID or path of the project owned by the authenticated user")
      .global(true)
      .takes_value(true),
    arg_name(ArgName::RepoUrl)
      .help("URL of your Gitlab domain")
      .global(true)
      .takes_value(true),
  ]
}

fn get_config_subcmd<'a, 'b>() -> App<'a, 'b> {
  sub_name(ArgName::Config)
    .about("Command to work with config")
    .settings(&SUB_CMD_SETTINGS)
    .subcommands(vec![
      sub_name(ArgName::SaveToken)
        .about("Stores token to config file")
        .arg(
          arg_name(ArgName::PrivateToken)
            .help("New GitLab private token to store")
            .required(true)
            .index(1),
        ),
      sub_name(ArgName::ShowToken).about("Shows GitLab private token if exists"),
      sub_name(ArgName::ForgetToken).about("Removes global config file where private token is"),
    ])
}

fn get_create_subcm<'a, 'b>() -> App<'a, 'b> {
  sub_name(ArgName::Create)
    .about("Created new merge requests")
    .settings(&SUB_CMD_SETTINGS)
    .arg(
      arg_name(ArgName::PrivateToken)
        .help("Sets the GitLab private token for requests")
        .takes_value(true)
        .global(true),
    )
    .subcommands(vec![sub_name(ArgName::Mr)
      .about("Creates merge request")
      .args(&[
        def_arg(ArgName::Title, "I", "Title of MR"),
        def_arg(ArgName::SourceBranch, "S", "The source branch"),
        def_arg(ArgName::TargetBranch, "T", "The target branch"),
        def_arg(ArgName::AssigneeId, "G", "Assignee user ID"),
        def_arg(ArgName::AssigneeName, "A", "Assignee user name")
          .conflicts_with(ArgName::AssigneeId.into()),
        def_arg(
          ArgName::Description,
          "D",
          "Description of MR. Limited to 1 000 000 characters",
        ),
        def_flag(
          ArgName::RemoveSourceBranch,
          "R",
          "Flag indicating if a merge request should remove the source branch when merging",
        ),
        def_flag(
          ArgName::Squash,
          "Q",
          "Squash commits into a single commit when merging",
        ),
      ])])
}

fn get_lsmr_subsubcmd<'a, 'b>() -> App<'a, 'b> {
  let state_possible_vals = ["opened", "closed", "locked", "merged"];
  let scope_possible_vals = ["created_by_me", "assigned_to_me", "all"];

  sub_name(ArgName::Mr)
  .about("Shows list of merge requests")
  .args(&[
    def_arg(ArgName::Search, "S", "Search merge requests against their title and description"),
    def_arg(
      ArgName::State,
      "E",
      "Return all merge requests or just those that are opened, closed, locked, or merged")
        .possible_values(&state_possible_vals),
    def_arg(
      ArgName::Scope,
      "C",
      "Return merge requests for the given scope: created_by_me, assigned_to_me or all. Defaults to created_by_me")
      .possible_values(&scope_possible_vals),
    def_arg(ArgName::AuthorId, "U", "Returns merge requests created by the given user id. Combine with scope=all or scope=assigned_to_me"),
    def_arg(ArgName::AssigneeId, "A", "Returns merge requests assigned to the given user id. None returns unassigned merge requests. Any returns merge requests with an assignee."),
    def_arg(ArgName::SourceBranch, "R", "Return merge requests with the given source branch"),
    def_arg(ArgName::TargetBranch, "T", "Return merge requests with the given target branch"),
  ])
}

fn get_lsprojects_subsubcmd<'a, 'b>() -> App<'a, 'b> {
  let vis_possible_vals = ["public", "internal", "private"];

  sub_name(ArgName::Projects)
    .about("Shows list of projects")
    .args(&[
      def_arg(
        ArgName::Search,
        "S",
        "Return list of projects matching the search criteria",
      ),
      def_arg(ArgName::Visibility, "V", "Limit by visibility").possible_values(&vis_possible_vals),
      def_flag(ArgName::Archived, "A", "Limit by archived status"),
      def_flag(
        ArgName::Owned,
        "O",
        "Limit by projects explicitly owned by the current user",
      ),
      def_flag(
        ArgName::Membership,
        "M",
        "Limit by projects that the current user is a member of",
      ),
    ])
}

fn get_ls_subcmd<'a, 'b>() -> App<'a, 'b> {
  sub_name(ArgName::Ls)
    .about("Prints info about everything")
    .settings(&SUB_CMD_SETTINGS)
    .arg(
      arg_name(ArgName::PrivateToken)
        .help("Sets the GitLab private token for requests")
        .takes_value(true)
        .global(true),
    )
    .subcommands(vec![
      get_lsmr_subsubcmd(),
      get_lsprojects_subsubcmd(),
      sub_name(ArgName::Users)
        .about("Shows list of users")
        .args(&[
          def_arg(ArgName::Username, "N", "Search for users by name or primary email"),
          def_flag(ArgName::Active, "A", "Show only active users"),
          def_flag(ArgName::Blocked, "B", "Show only blocked users")
            .conflicts_with(ArgName::Active.into()),
        ]),
      sub_name(ArgName::Branches)
        .about("Shows list of branches")
        .args(&[
          def_arg(ArgName::Search, "S", "Return list of branches containing the search string. You can use ^term and term$ to find branches that begin and end with term respectively.")
        ]),

    ])
}

pub fn get_matches<'a>() -> ArgMatches<'a> {
  App::new("mergereq")
    .version(crate_version!())
    .author(crate_authors!())
    .settings(&SUB_CMD_SETTINGS)
    .args(&get_global_args())
    .subcommands(vec![
      get_config_subcmd(),
      get_create_subcm(),
      get_ls_subcmd(),
    ])
    .get_matches()
}
