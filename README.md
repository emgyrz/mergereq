# mergereq
CLI for Gitlab merge requests & more


[![Crates.io](https://img.shields.io/crates/v/mergereq)](https://crates.io/crates/mergereq)
[![npm](https://img.shields.io/npm/v/mergereq-bin)](https://www.npmjs.com/package/mergereq-bin)

### Install
```sh
cargo install mergereq
# or
npm i -g mergereq-bin
```


### Setup
First of all, to request data requires authentication you need to save you Gitlab private token.
( Find it [here](https://docs.gitlab.com/ee/user/profile/personal_access_tokens.html) ).
```sh
mergereq config save-token "$YOUR_PRIVATE_TOKEN"
```
Token will be saved (by default) at system config dir, e.g. for Linux it would be `~/.config/.mergereq-config`.
Path to global config can be overwritten with `--global-config` option.


Also create the local configuration file for mergereq in directory when you want to use it
(default name is `./.mergereqrc.toml`). Path can be overwritten with `--local-config` option.
```toml
# Gitlab API endpoint
repo_url = "https://example.com"

# The ID or path of the project
default_project = "web/my_best_project"
```

You may overwrite all this parameters when run command with `--private-token`, `--repo-url` and `-P, --project` options.


### Docs

Available subcommands:
* `config save-token` - Stores token to config file
* `config show-token` - Shows GitLab private token if exists
* `config forget-token` - Removes global config file where private token is
* `create mr` - Creates merge request
* `ls branches` - Shows list of branches
* `ls mr` - Shows list of merge requests
* `ls projects` - Shows list of projects
* `ls users` - Shows list of users

#### Aboute merge request creating
Some options has default values

| Option                                | Desc                                                                                                                           | Default                                 |
| ------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------ | --------------------------------------- |
| `-S, --src <source-branch>`           | The source branch                                                                                                              | Current git branch                      |
| `-T, --trg <target-branch>`           | The target branch                                                                                                              | Project default branch                  |
| `-I, --title <title>`                 | The target branch                                                                                                              | Message of last commit in source branch |
| `--assignee-id <assignee-id>`         | Assignee user ID                                                                                                               | Unassigned                              |
| `-A, --assignee-name <assignee-name>` | Assignee user name. `mergereq` will fetch all active users, then search one with specified name, if not available throws error | Unassigned                              |
| `-R, --remove-source-branch`          | Flag indicating if a merge request should remove the source branch when merging                                                | false                                   |
| `-Q, --squash`                        | Squash commits into a single commit when merging                                                                               | false                                   |


##### Example
```sh
mergereq create mr -A team.lead

#  You creating merge requests with this parameters:
#    Source branch: — feature/move_btn
#    Target branch: — develop
#    Title branch:  — Move button by 1px to the right
#    Assignee:    —   team.lead (ID: 2)
#  Do you want to continue? [Y/n]
#
#  Your merge request is created. You can see it here:
#  https://example.com/web/my_best_project/merge_requests/23
#  Status: can_be_merged

```

#### Create merge request
```
mergereq-create-mr
Creates merge request

USAGE:
    mergereq create mr [FLAGS] [OPTIONS]

FLAGS:
    -h, --help                    Prints help information
    -R, --remove-source-branch    Flag indicating if a merge request should remove the source branch when merging
    -Q, --squash                  Squash commits into a single commit when merging

OPTIONS:
        --assignee-id <assignee-id>        Assignee user ID
    -A, --assignee-name <assignee-name>    Assignee user name
    -D, --desc <description>               Description of MR. Limited to 1 000 000 characters
        --global-config <global-config>    Path of global config file. e.g. for Linux it would be `~/.config/.mergereq-config`
        --local-config <local-config>      Path of local config file. Default is `.mergereqrc.toml` in the current
                                           directory.
        --private-token <private-token>    Sets the Gitlab private token for requests
    -P, --project <project>                The ID or path of the project owned by the authenticated user
        --repo-url <repo-url>              URL of your Gitlab domain
    -S, --src <source-branch>              The source branch
    -T, --trg <target-branch>              The target branch
    -I, --title <title>                    Title of MR

```

All documentation available in mergereq CLI with `--help` or `-h` flag.

##### Enjoy using!

### License

This module is [MIT licensed](./LICENSE).
