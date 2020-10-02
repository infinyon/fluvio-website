---
title: Troubleshoot Fluvio Installation
menu: Troubleshooting
toc: true
weight: 40
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
😄  minikube v1.13.1 on Ubuntu 20.04
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

### `fluvio cluster install`: Fluvio system chart is not installed

If you see an error like the one below, it's likely that you forgot to run
the install command with the `--sys` option first.

```bash
$ fluvio cluster install --chart-version=0.6.0-latest --image-version=latest
Error: 
   0: Fluvio cluster error
   1: An unknown error occurred: Fluvio system chart is not installed, please install fluvio-sys first

Backtrace omitted.
Run with RUST_BACKTRACE=1 environment variable to display it.
Run with RUST_BACKTRACE=full to include source snippets.
```

- **Fix**: Run the following command to install the system chart
(note the `--sys` flag at the end)

```bash
$ fluvio cluster install --chart-version=0.6.0-latest --image-version=latest --sys
```

### `fluvio cluster install`: Cluster in kube context cannot use IP address

If you see an error like the one below, something went wrong when Fluvio tried to
integrate with Minikube.

```bash
$ fluvio cluster install --chart-version=0.6.0-latest --image-version=latest
Error: 
   0: Fluvio cluster error
   1: An unknown error occurred: Cluster in kube context cannot use IP address, please use minikube context: 172.17.0.3

Backtrace omitted.
Run with RUST_BACKTRACE=1 environment variable to display it.
Run with RUST_BACKTRACE=full to include source snippets.
```

- **Fix**: We have a command that should help resolve this. It will prompt for `sudo`
because Fluvio needs to add an entry to a special file called `/etc/hosts` in order
to connect to Minikube. Run the following command:

```bash
$ fluvio cluster set-minikube-context
```

### `fluvio cluster install`: waiting for sc service up come up

Sometimes when running the install command, you might run into a loop like this:

```bash
$ fluvio cluster install --chart-version=0.6.0-latest --image-version=latest
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
fluvio@fluvio:~/fluvio$ fluvio cluster install --chart-version=0.6.0-latest --image-version=latest
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

- **Fix**: We need to re-run the Fluvio installer. All we have to do is uninstall the
cluster, then re-install it with the same install command you tried before. To uninstall
Fluvio, run the following:

```bash
$ fluvio cluster uninstall
fluvio@fluvio:~/fluvio$ fluvio cluster uninstall
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

Then, when you go to re-install it, a successful install will look like this:

```bash
$ fluvio cluster install --chart-version=0.6.0-latest --image-version=latest
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

### `fluvio cluster set-minikube-context`: Kubernetes config error

On some systems, running the `set-minikube-context` command doesn't work correctly yet.
When you run it, you might see an error like the following:

```bash
$ fluvio cluster set-minikube-context
Error: 
   0: Kubernetes config error
   1: IO error: Exec format error (os error 8)
   2: Exec format error (os error 8)

Backtrace omitted.
Run with RUST_BACKTRACE=1 environment variable to display it.
Run with RUST_BACKTRACE=full to include source snippets.
```

In this case, you'll need to make an edit to your `/etc/hosts` file manually. The reason
we need to do this is to set a hostname that points to minikube's IP address. Think of it
like a road sign that tells programs where minikube lives. The first thing we need to do
is make sure we can actually see minikube's address.

```bash
$ minikube ip
172.17.0.3
```

-> The IP address you see for minikube might be different, but that's ok!

The next thing we'll want to do is make a backup of the hosts file so that we can restore
it if something goes wrong.

```bash
$ cp /etc/hosts ~/Desktop/
```

-> Now, if you want to reset things back to how they were, you can just run `sudo cp ~/Desktop/hosts /etc/hosts`

Then, we want to add a line to the end of `/etc/hosts` that says `172.17.0.3 minikubeCA`.
We can do that with the following command:

```bash
$ echo "$(minikube ip) minikubeCA" | sudo tee -a /etc/hosts
```

Just so you understand what's going on here, here's what that line does:

- `$(minikube ip)`: This is running the `minikube ip` command inline, and then substituting
the output into the string. After this step runs, the shell will continue executing the rest
of the command like this:

```bash
$ echo "172.17.0.3 minikubeCA" | sudo tee -a /etc/hosts
```

-> Here, `minikubeCA` is being used as the new hostname for minikube, and `172.17.0.3` is the address

- `sudo tee -a /etc/hosts`: This is the part of the command that writes the line to the
`/etc/hosts` file. It's important that you don't forget the `-a`, otherwise it will erase
the file instead of appending to it! But don't worry, you made a backup :)

To make sure everything worked correctly, let's take a look at the `/etc/hosts` file. You
should see something similar to this:

```bash
$ cat /etc/hosts
127.0.0.1	localhost
127.0.1.1	your-hostname
...
172.17.0.3 minikubeCA
```

If everything worked correctly, you should now be able to run `set-minikube-context`
successfully:

```bash
$ fluvio cluster set-minikube-context
Cluster "flvkube" set.
Context "flvkube" created.
Switched to context "flvkube".
```
