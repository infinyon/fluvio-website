```
Generates a new SmartModule Project

Usage: smdk generate [OPTIONS] [NAME]

Arguments:
  [NAME]  SmartModule Project Name

Options:
      --project-group <GROUP>  SmartModule Project Group Name. Default to Hub ID, if set. Overrides
                               Hub ID if provided [env: SMDK_PROJECT_GROUP=]
      --destination <PATH>     Local path to generate the SmartModule project. Default to directory
                               with project name, created in current directory [env:
                               SMDK_DESTINATION=]
      --sm-type <TYPE>         Type of SmartModule project to generate. Skip prompt if value given
                               [env: SMDK_SM_TYPE=] [possible values: filter, map, array-map,
                               aggregate, filter-map]
      --with-params            Include SmartModule input parameters in generated SmartModule
                               project. Skip prompt if value given [env: SMDK_WITH_PARAMS=]
      --no-params              No SmartModule input parameters in generated SmartModule project.
                               Skip prompt if value given [env: SMDK_NO_PARAMS=]
  -h, --help                   Print help information (use `--help` for more detail)
```