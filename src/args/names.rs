use std::convert::AsRef;

#[derive(Debug, Clone)]
pub enum ArgName {
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
  Ls,
  Users,
  Username,
  Active,
  Blocked,
  Branches,
  Search,
  State,
  Scope,
  AuthorId,
  Projects,
  Visibility,
  Archived,
  Owned,
  Membership,
}

impl From<ArgName> for &str {
  fn from(arg_name: ArgName) -> &'static str {
    match arg_name {
      ArgName::PrivateToken => "private-token",
      ArgName::GlobalConfig => "global-config",
      ArgName::LocalConfig => "local-config",
      ArgName::Project => "project",
      ArgName::RepoUrl => "repo-url",
      ArgName::Config => "config",
      ArgName::SaveToken => "save-token",
      ArgName::ShowToken => "show-token",
      ArgName::ForgetToken => "forget-token",
      ArgName::Create => "create",
      ArgName::Mr => "mr",
      ArgName::Title => "title",
      ArgName::SourceBranch => "source-branch",
      ArgName::TargetBranch => "target-branch",
      ArgName::AssigneeId => "assignee-id",
      ArgName::AssigneeName => "assignee-name",
      ArgName::Description => "description",
      ArgName::RemoveSourceBranch => "remove-source-branch",
      ArgName::Squash => "squash",
      ArgName::Ls => "ls",
      ArgName::Users => "users",
      ArgName::Username => "username",
      ArgName::Active => "active",
      ArgName::Blocked => "blocked",
      ArgName::Branches => "branches",
      ArgName::Search => "search",
      ArgName::State => "state",
      ArgName::Scope => "scope",
      ArgName::AuthorId => "authorId",
      ArgName::Projects => "projects",
      ArgName::Visibility => "visibility",
      ArgName::Archived => "archived",
      ArgName::Owned => "owned",
      ArgName::Membership => "membership",
    }
  }
}

impl AsRef<str> for ArgName {
  fn as_ref(&self) -> &str {
    self.clone().into()
  }
}
