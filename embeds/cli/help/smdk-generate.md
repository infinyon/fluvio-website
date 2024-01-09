```
Generates a new SmartModule Project

Usage: smdk generate [OPTIONS] [NAME]

Arguments:
  [NAME]
          SmartModule Project Name

Options:
      --project-group <GROUP>
          SmartModule Project Group Name. Default to Hub ID, if set. Overrides Hub ID if provided
          
          [env: SMDK_PROJECT_GROUP=]

      --destination <PATH>
          Local path to generate the SmartModule project. Default to directory with project name,
          created in current directory
          
          [env: SMDK_DESTINATION=]

      --silent
          Disable interactive prompt. Take all values from CLI flags. Fail if a value is missing

      --template-repo <GIT_URL>
          URL to git repo containing templates for generating SmartModule projects. Using this
          option is discouraged. The default value is recommended
          
          [env: SMDK_TEMPLATE_REPO=]

      --template-repo-branch <BRANCH>
          An optional git branch to use with `--template-repo`
          
          [env: SMDK_TEMPLATE_REPO_BRANCH=]

      --template-repo-tag <TAG>
          An optional git tag to use with `--template-repo`
          
          [env: SMDK_TEMPLATE_REPO_TAG=]

      --template-path <PATH>
          Local filepath containing templates for generating SmartModule projects. Using this option
          is discouraged. The default value is recommended
          
          [env: SMDK_TEMPLATE_PATH=]

      --sm-crate-repo <GIT_URL>
          URL of git repo to include in generated Cargo.toml. Repo used for `fluvio-smartmodule`
          dependency. Using this option is discouraged. The default value is recommended
          
          [env: SMDK_SM_CRATE_REPO=]

      --sm-repo-branch <BRANCH>
          An optional git branch to use with `--sm-crate-repo`
          
          [env: SMDK_SM_REPO_BRANCH=]

      --sm-repo-tag <TAG>
          An optional git tag to use with `--sm-crate-repo`
          
          [env: SMDK_SM_REPO_TAG=]

      --sm-repo-rev <GIT_SHA>
          An optional git rev to use with `--sm-crate-repo`
          
          [env: SMDK_SM_REPO_REV=]

      --sm-crate-path <PATH>
          Local filepath to include in generated Cargo.toml. Path used for fluvio-smartmodule
          dependency. Using this option is discouraged. The default value is recommended
          
          [env: SMDK_SM_CRATE_PATH=]

      --sm-crate-version <X.Y.Z>
          Public version of `fluvio-smartmodule` from crates.io. Defaults to latest. Using this
          option is discouraged. The default value is recommended
          
          [env: SMDK_SM_CRATE_VERSION=]

      --sm-type <TYPE>
          Type of SmartModule project to generate. Skip prompt if value given
          
          [env: SMDK_SM_TYPE=]
          [possible values: filter, map, array-map, aggregate, filter-map]

      --sm-public <PUBLIC>
          Visibility of SmartModule project to generate. Skip prompt if value given
          
          [env: SMDK_SM_PUBLIC=]
          [possible values: true, false]

      --with-params
          Include SmartModule input parameters in generated SmartModule project. Skip prompt if
          value given
          
          [env: SMDK_WITH_PARAMS=]

      --no-params
          No SmartModule input parameters in generated SmartModule project. Skip prompt if value
          given
          
          [env: SMDK_NO_PARAMS=]

      --hub-remote <HUB_REMOTE>
          Set the remote URL for the hub
          
          [env: SMDK_HUB_REMOTE=]

      --develop
          Using this option will always choose the Fluvio repo as source for templates and
          dependencies
          
          [env: SMDK_DEVELOP=]

  -h, --help
          Print help (see a summary with '-h')
```