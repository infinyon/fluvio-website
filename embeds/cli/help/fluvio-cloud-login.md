```
fluvio-cloud-login 
Log into Infinyon Cloud with Oauth2 or username/password

USAGE:
    fluvio-cloud login [OPTIONS]

OPTIONS:
        --email <EMAIL>          Infinyon Cloud email to use for logging in
    -h, --help                   Print help information
        --password <PASSWORD>    Password to use when logging in (not recommended)
        --profile <PROFILE>      The name of the Profile to save
        --use-oauth2             Authenticate using OAuth 2.0 Device Code Flow. CLI will try to
                                 launch a web browser to log in interactively. If a web browser is
                                 not available, CLI will print URL for device code login
```