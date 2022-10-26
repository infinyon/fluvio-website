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

Run this command to log into InfinyOn Cloud.

You log in with either username and password or Oauth2.

After login, a CLI profile called `cloud` will be created and set as active. For more about profiles, see [`fluvio profile`]({{<ref "/cli/client/profile.md">}})

%copy first-line%
```bash
$ fluvio cloud login -h
```

{{% inline-embed file="embeds/cli/help/fluvio-cloud-login.md" %}}

On login, a profile called `cloud` will be created and set as active. For more about profiles, see [`fluvio profile`]({{<ref "/cli/client/profile.md">}})

%copy first-line%
```bash
$ fluvio profile list
    PROFILE    CLUSTER    ADDRESS                          TLS
 *  cloud      cloud      router.infinyon.cloud:9003      Verified
```

### Examples
#### Email and password method

%copy first-line%
```bash
$ fluvio cloud login
InfinyOn Cloud email: example@infinyon.com
Password: <hidden>
```

#### OAuth2 method

%copy first-line%
```bash
$ fluvio cloud login --use-oauth2
A web browser has been opened at https://<OAUTH2_SERVER_DOMAIN>/activate?user_code=<CODE>.
Please proceed with authentication.
```

[ Download Fluvio CLI ]: {{< ref "/cli/client/overview" >}}
[Provision InfinyOn Cloud Account]: https://infinyon.cloud/
[`Fluvio CLI`]: {{< ref "/cli/client/overview" >}}
[`Cloud account`]: https://infinyon.cloud/
[`fluvio profile list`]: {{< ref "/cli/client/profile#fluvio-profile-list" >}}
[`fluvio profile switch`]: {{< ref "/cli/client/profile#fluvio-profile-switch" >}}

