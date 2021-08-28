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
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --email <email>          Fluvio Cloud email to use for logging in
        --password <password>    Password to use when logging in (not recommended)
        --profile <profile>      The name of the Profile to save
```

Example usage:

%copy first-line%
```bash
$ fluvio cloud login
Fluvio Cloud email: mosher@infinyon.com
Password: <hidden>
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

[`fluvio profile list`]: {{< ref "/cli/commands/profile#fluvio-profile-list" >}}
[`fluvio profile switch`]: {{< ref "/cli/commands/profile#fluvio-profile-switch" >}}
