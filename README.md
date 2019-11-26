# mergereq

### Create merge request
```sh
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
        --private-token <private-token>    Sets the GitLab private token for requests
    -P, --project <project>                The ID or path of the project owned by the authenticated user
        --repo-url <repo-url>              URL of your Gitlab domain
    -S, --src <source-branch>              The source branch
    -T, --trg <target-branch>              The target branch
    -I, --title <title>                    Title of MR

```

### List
```sh
mergereq-ls
Print info about everything

USAGE:
    mergereq ls [OPTIONS] <SUBCOMMAND>

FLAGS:
    -h, --help    Prints help information

OPTIONS:
        --global-config <global-config>    Path of global config file. e.g. for Linux it would be `~/.config/.mergereq
                                           -config`
        --local-config <local-config>      Path of local config file. Default is `.mergereqrc.toml` in the current
                                           directory.
        --private-token <private-token>    Sets the GitLab private token for requests
    -P, --project <project>                The ID or path of the project owned by the authenticated user
        --repo-url <repo-url>              URL of your Gitlab domain

SUBCOMMANDS:
    branches    List branches
    mr          List merge requests
    projects    List projects
    users       List users
```



### List merge requests
```sh
mergereq-ls-mr
List merge requests

USAGE:
    mergereq ls mr [OPTIONS]

FLAGS:
    -h, --help    Prints help information

OPTIONS:
    -A, --assignee-id <assignee-id>        Returns merge requests assigned to the given user id. None returns unassigned
                                           merge requests. Any returns merge requests with an assignee.
    -U, --author-id <author-id>            Returns merge requests created by the given user id. Combine with scope=all
                                           or scope=assigned_to_me
        --global-config <global-config>    Path of global config file. e.g. for Linux it would be `~/.config/.mergereq
                                           -config`
        --local-config <local-config>      Path of local config file. Default is `.mergereqrc.toml` in the current
                                           directory.
        --private-token <private-token>    Sets the GitLab private token for requests
    -P, --project <project>                The ID or path of the project owned by the authenticated user
        --repo-url <repo-url>              URL of your Gitlab domain
    -C, --scope, <scope>                   Return merge requests for the given scope: created_by_me, assigned_to_me or
                                           all. Defaults to created_by_me [values: created_by_me, assigned_to_me, all]
    -S, --search <search>                  Search merge requests against their title and description
    -R, --source-branch <source-branch>    Return merge requests with the given source branch
    -E, --state, <state>                   Return all merge requests or just those that are opened, closed, locked, or
                                           merged [values: opened, closed, locked, merged]
    -T, --target-branch <target-branch>    Return merge requests with the given target branch
```
