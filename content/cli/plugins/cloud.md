---
title: Cloud
weight: 10
---

The `fluvio-cloud` plugin is distributed by default when Fluvio is installed
using the one-liner installation script:

%copy first-line%
```bash
$ curl -fsS https://packages.fluvio.io/v1/install.sh | bash
```

It is used to log into a [Fluvio Cloud account] and download a profile that
allows you to connect to your account's hosted cluster. After downloading
this profile, you may use the rest of the Fluvio commands like normal and
they will interact with your Fluvio Cloud cluster.

[Fluvio Cloud account]: https://cloud.fluvio.io/

## `fluvio cloud login`

After creating a Fluvio Cloud account, run this command to log in with your
username and password and download your Fluvio Cloud profile. This will
automatically create a profile called `cloud` in your Fluvio config file
(`~/.fluvio/config`) and set it as the active profile.

```
fluvio-cloud-login
Log into Fluvio Cloud with a username and password

USAGE:
    fluvio-cloud login [OPTIONS]

FLAGS:
    -h, --help         Prints help information
    -V, --version      Prints version information
        --use-oauth2   Authenticate using OAuth 2.0 Device Code Flow

OPTIONS:
        --email <email>          Fluvio Cloud email to use for logging in
        --password <password>    Password to use when logging in (not recommended)
        --profile <profile>      The name of the Profile to save
```

Example usage with email and password:

%copy first-line%
```bash
$ fluvio cloud login
Fluvio Cloud email: batman@justiceleague.com
Password: <hidden>
```

Example usage with OAuth2:
%copy first-line%
```bash
$ fluvio cloud login --use-oauth2
A web browser has been opened at https://OAUTH_SERVER_DOMAIN/activate?user_code=<CODE>.
Please proceed with authentication.
```

After this has completed successfully, you should be able to see your new cloud
profile using [`fluvio profile list`]:

%copy first-line%
```bash
$ fluvio profile list
    PROFILE    CLUSTER    ADDRESS                          TLS
 *  cloud      cloud      router.cloud.fluvio.io:9003      Verified
```

At this point, any other `fluvio` commands (such as produce, consume, etc.) will
interact with your `cloud` cluster. To switch back to a different cluster, use
[`fluvio profile switch`].

[`fluvio profile list`]: {{< ref "/cli/installation/profile#fluvio-profile-list" >}}
[`fluvio profile switch`]: {{< ref "/cli/installation/profile#fluvio-profile-switch" >}}


## `fluvio cloud usage` (since version 0.1.7)

After logging in you are able to query the CPU and memory usage of your SPUs with this command.

Example usage:

%copy first-line%
```bash
$ fluvio cloud usage 
 SPU     CPU       Memory 
 main-0  1066037n  3168Ki 

Note: CPU usage is expressed in nanocores. 1 nanocore is equal to 1 billionth of 1 core.
```