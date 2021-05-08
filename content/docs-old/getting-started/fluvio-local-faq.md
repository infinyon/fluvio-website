---
title: Troubleshoot Fluvio Installation
menu: Troubleshooting
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
