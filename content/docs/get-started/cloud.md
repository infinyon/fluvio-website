---
title: Fluvio Cloud
menu: Cloud
weight: 60
---

Fluvio Cloud is the fastest and easiest way to get started with Fluvio. We'll walk you through the steps of creating a free account and connecting to it with a Fluvio client.


You can reach us on <a href="https://discordapp.com/invite/bBG2dTz" target="_blank">Discord</a> or in <a href="https://github.com/infinyon/fluvio/issues" target="_blank">Github</a>.
## Install Fluvio CLI

The Fluvio CLI (_command-line interface_) is an all-in-one tool for setting up, managing, and interacting with Fluvio clusters.

Install the Fluvio CLI by running the following command:

%copy first-line%
```bash
curl -fsS https://packages.fluvio.io/v1/install.sh | bash
```

## Creating an InfinyOn Cloud account

Head on over to the <a href="https://infinyon.cloud" target="_blank">InfinyOn Cloud signup page</a> to create an account.

<img src="../images/cloud-signup.png"
     alt="A screenshot of the InfinyOn new account form, with Name, Organization, Email, and Password fields"
     style="justify: center; max-width: 400px" />

After filling out the form, you'll be greeted with a success message telling you to verify your email. You'll need to complete this step in order to continue.

<img src="../images/cloud-verification.png"
     alt="A screenshot of the verification email received after completing the signup form, including a verification link"
     style="justify: center; max-width: 600px" />

You should get a confirmation that your account is ready to use

<img src="../images/cloud-confirmation.png"
     alt="A screenshot of the prompt received after clicking the verification link, saying the account is ready to use"
     style="justify: center; max-width: 600px" />


At this point, we can log in via the Fluvio CLI and start sending and receiving messages to your Fluvio cluster. To log in with the CLI, you'll need to run the `fluvio cloud login` command, then type in your email and password when prompted.

%copy first-line%
```bash
$ fluvio cloud login
Fluvio Cloud email: batman@justiceleague.com
Password:
```

You'll be able to tell that everything worked if your current profile is set to `cloud`. You can check with this command:

%copy first-line%
```bash
$ fluvio profile current
cloud
```

If you installed fluvio locally it will be listed alongside `cloud`:

%copy first-line%
```bash
$ fluvio profile view
    PROFILE       CLUSTER       ADDRESS                          TLS 
    local         local         localhost:9003                   Disabled 
 *  cloud         cloud         router.cloud.fluvio.io:9003      Verified
```

-> Use `fluvio profile switch` command to switch between clusters.