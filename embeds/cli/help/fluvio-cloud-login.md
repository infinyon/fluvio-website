```
Log into Infinyon Cloud with Oauth2 or username/password

Usage: fluvio-cloud login [OPTIONS]

Options:
      --email <EMAIL>
          Infinyon Cloud email to use for logging in

      --password <PASSWORD>
          Password to use when logging in (not recommended).
          
          If not provided, fluvio-cloud will prompt on stdin unless using third party
          authentication.

      --profile <PROFILE>
          The name of the Profile to save

      --use-oauth2
          Authenticate using OAuth 2.0 Device Code Flow. CLI will try to launch a web browser to log
          in interactively. If a web browser is not available, CLI will print URL for device code
          login

  -h, --help
          Print help (see a summary with '-h')
```