---
title: Troubleshoot Fluvio Installation
menu: Troubleshooting
toc: true
weight: 60
---

Fluvio is a very young project, so we're still ironing out all the wrinkles
when it comes to the first-time installation process. If you get stuck while
following the [Installing Fluvio Locally] guide, check this page to see if
you're running into one of the more common errors. If you still can't figure
it out, please feel free to [open an issue] or to drop by
[our Discord channel] to ask us for help!

[Installing Fluvio Locally]: ./fluvio-local.md
[open an issue]: https://github.com/infinyon/fluvio/issues/new
[our Discord channel]: https://discord.gg/Z6ebmmh

## Help, I'm stuck

Let's try to get you unstuck. Here are some known installation problems and
how to fix them. We're working to shrink this list over time :)

### `minikube start`: Unable to pick a default driver

If you've never used a container technology like Docker before, you might run
into trouble at the `minikube start` step that looks like this

```bash
$ minikube start
😄  minikube v1.16.0 on Ubuntu 20.04
👎  Unable to pick a default driver. Here is what was considered, in preference order:
    ▪ docker: Not installed: exec: "docker": executable file not found in $PATH
    ▪ kvm2: Not installed: exec: "virsh": executable file not found in $PATH
    ▪ none: Not installed: exec: "docker": executable file not found in $PATH
    ▪ podman: Not installed: exec: "podman": executable file not found in $PATH
    ▪ virtualbox: Not installed: unable to find VBoxManage in $PATH
    ▪ vmware: Not installed: exec: "docker-machine-driver-vmware": executable file not found in $PATH

❌  Exiting due to DRV_NOT_DETECTED: No possible driver was detected. Try specifying --driver, or see https://minikube.sigs.k8s.io/docs/start/
```

Make sure that you've [installed docker correctly] on your system. If everything
is set up right, you should be able to run `docker run hello-world` without sudo.
If you're using Linux, make sure you've followed all of the [post-install steps].

Specifically, make sure you've added your user to the `docker` group:

```bash
$ sudo usermod -aG docker $USER
```

```bash
$ groups
... docker ...
```

Once you show up in the `docker` group, try logging out and logging back into your system.

Once you've done that, try again using `minikube start --driver=docker`.

[installed docker correctly]: https://hub.docker.com/search?q=&type=edition&offering=community&sort=updated_at&order=desc
[post-install steps]: https://docs.docker.com/engine/install/linux-postinstall/

### `fluvio cluster start`: waiting for sc service up come up

Sometimes when running the install command, you might run into a loop like this:

```bash
$ fluvio cluster start
"fluvio" already exists with the same configuration, skipping
Hang tight while we grab the latest from your chart repositories...
...Successfully got an update from the "fluvio" chart repository
Update Complete. ⎈Happy Helming!⎈
NAME: fluvio
LAST DEPLOYED: Fri Oct  2 10:07:22 2020
NAMESPACE: default
STATUS: deployed
REVISION: 1
TEST SUITE: None
waiting for sc service up come up: 0
waiting for sc service up come up: 1
waiting for sc service up come up: 2
...
waiting for sc service up come up: 28
waiting for sc service up come up: 29
Error: 
   0: Fluvio cluster error
   1: An unknown error occurred: Timed out when waiting for SC service

Backtrace omitted.
Run with RUST_BACKTRACE=1 environment variable to display it.
Run with RUST_BACKTRACE=full to include source snippets.
```

This means that the minikube tunnel isn't running. This can sometimes happen if you
run `minikube tunnel` using `nohup`. If you check your processes for minikube tunnel,
you probably won't see anything:

```bash
$ ps aux | grep "minikube tunnel"
fluvio    510051  0.0  0.0   9036   728 pts/0    R+   10:26   0:00 grep --color=auto minikube tunnel
```

- **Fix**: You need to run `minikube tunnel` again. Try it this time without using `nohup`.
Open a new terminal window and run the following:

```bash
$ sudo minikube tunnel
```

### `fluvio cluster install`: 0 of 1 spu ready

Sometimes when running the install command, you might run into a loop like this:

```bash
fluvio@fluvio:~/fluvio$ fluvio cluster install
"fluvio" already exists with the same configuration, skipping
Hang tight while we grab the latest from your chart repositories...
...Successfully got an update from the "fluvio" chart repository
Update Complete. ⎈Happy Helming!⎈
NAME: fluvio
LAST DEPLOYED: Fri Oct  2 11:15:21 2020
NAMESPACE: default
STATUS: deployed
REVISION: 1
TEST SUITE: None
waiting for sc service up come up: 0
waiting for sc service up come up: 1
waiting for sc service up come up: 2
0 of 1 spu ready
0 of 1 spu ready
0 of 1 spu ready
0 of 1 spu ready
0 of 1 spu ready
0 of 1 spu ready
0 of 1 spu ready
0 of 1 spu ready
^C
```

This means that Fluvio's [Streaming Controller (sc)] is online, but failed to launch a
[Streaming Processing Unit (spu)].

- **Fix**: We need to re-run the Fluvio installer. All we have to do is delete the
cluster, then re-start it with the same install command you tried before. To uninstall
Fluvio, run the following:

```bash
$ fluvio cluster delete
removing fluvio installation
removing kubernetes cluster
release "fluvio" uninstalled
checking to see if Pod is deleted, count: 0
sc Pod still exists, sleeping 10 second
checking to see if Pod is deleted, count: 1
sc Pod still exists, sleeping 10 second
checking to see if Pod is deleted, count: 2
sc Pod still exists, sleeping 10 second
checking to see if Pod is deleted, count: 3
sc Pod still exists, sleeping 10 second
checking to see if Pod is deleted, count: 4
no sc Pod found, can proceed to setup 
deleting all spugroups in: default
spugroup.fluvio.infinyon.com "main" deleted
deleting all spus in: default
spu.fluvio.infinyon.com "main-0" deleted
deleting all topics in: default
No resources found
deleting label 'app=spu' object persistentvolumeclaims in: default
persistentvolumeclaim "data-flv-spg-main-0" deleted
```

Then, when you go to re-start it, a successful install will look like this:

```bash
$ fluvio cluster start
"fluvio" already exists with the same configuration, skipping
Hang tight while we grab the latest from your chart repositories...
...Successfully got an update from the "fluvio" chart repository
Update Complete. ⎈Happy Helming!⎈
NAME: fluvio
LAST DEPLOYED: Fri Oct  2 11:19:23 2020
NAMESPACE: default
STATUS: deployed
REVISION: 1
TEST SUITE: None
waiting for sc service up come up: 0
waiting for sc service up come up: 1
waiting for sc service up come up: 2
waiting for sc service up come up: 3
0 of 1 spu ready
0 of 1 spu ready
0 of 1 spu ready
0 of 1 spu ready
0 of 1 spu ready
waiting for spu to be provisioned
1 spus provisioned
```

### `fluvio cluster start`: repository name (fluvio) already exists

Sometimes if you make it partway through a startup and encounter an error, you'll need
to run the uninstaller to reset to a fresh state. You need to do that if you encounter
the following error:

```bash
$ fluvio cluster start
Error: repository name (fluvio) already exists, please specify a different name
Exited with status code: 1
The application panicked (crashed).
Message: internal error: entered unreachable code
Location: .../.cargo/registry/src/github.com-1ecc6299db9ec823/flv-util-0.5.2/src/cmd.rs:92

Backtrace omitted.
Run with RUST_BACKTRACE=1 environment variable to display it.
Run with RUST_BACKTRACE=full to include source snippets.
```

- **Fix**: To reset before making a fresh startup, just run the following:

```bash
$ fluvio cluster delete
removing fluvio installation
removing kubernetes cluster
release "fluvio" uninstalled
```

Then try re-running the `fluvio cluster start` command again
