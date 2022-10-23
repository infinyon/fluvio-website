---
title: InfinyOn Cloud Overview
menu: Overview
weight: 10
---

InfinyOn Cloud is managed through the [`Fluvio CLI`] via `fluvio-cloud` plugin which is distributed by default by the installation script.

##### Prerequisites
* [Download Fluvio CLI]
* [Provision InfinyOn Cloud Account]

Use [`Fluvio CLI`] to login into your [`Cloud account`] and retrieve a unique profile that allows you to communicate with your environment securely.

## `fluvio cloud login`

After creating a Fluvio Cloud account, run this command to log in with your
username and password and download your Fluvio Cloud profile. This will
automatically create a profile called `cloud` in your Fluvio config file
(`~/.fluvio/config`) and set it as the active profile.

%copy first-line%
```bash
$ fluvio cloud login -h
```

```
fluvio-cloud-login 
Log into Infinyon Cloud with a username and password

USAGE:
    fluvio-cloud login [OPTIONS]

OPTIONS:
        --email <EMAIL>          Infinyon Cloud email to use for logging in
    -h, --help                   Print help information
        --password <PASSWORD>    Password to use when logging in (not recommended)
        --profile <PROFILE>      The name of the Profile to save
        --remote <REMOTE>        Infinyon Cloud remote address to use
        --use-oauth2             Authenticate using OAuth 2.0 Device Code Flow. CLI will try to
                                 launch a web browser to log in interactively. If a web browser is
                                 not available, CLI will print URL for device code login
```

#### Example login with email and password

%copy first-line%
```bash
$ fluvio cloud login
InfinyOn Cloud email: example@infinyon.com
Password: <hidden>
```

#### Example login with OAuth2

%copy first-line%
```bash
$ fluvio cloud login --use-oauth2
A web browser has been opened at https://<OAUTH2_SERVER_DOMAIN>/activate?user_code=<CODE>.
Please proceed with authentication.
```


## ```fluvio profile```

After this has completed successfully, you should be able to see your new cloud
profile using [`fluvio profile list`]:

%copy first-line%
```bash
$ fluvio profile list
    PROFILE    CLUSTER    ADDRESS                          TLS
 *  cloud      cloud      router.infinyon.cloud:9003      Verified
```

At this point, any other `fluvio` commands (such as produce, consume, etc.) will
interact with your `cloud` cluster. To switch back to a different cluster, use
[`fluvio profile switch`].

[ Download Fluvio CLI ]: {{< ref "/cli/client/overview" >}}
[Provision InfinyOn Cloud Account]: https://infinyon.cloud/
[`Fluvio CLI`]: {{< ref "/cli/client/overview" >}}
[`Cloud account`]: https://infinyon.cloud/
[`fluvio profile list`]: {{< ref "/cli/client/profile#fluvio-profile-list" >}}
[`fluvio profile switch`]: {{< ref "/cli/client/profile#fluvio-profile-switch" >}}

