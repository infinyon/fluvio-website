---
title: Install 
weight: 10
---

{{% inline-embed file="embeds/cli/help/fvm-install.md" %}}

The following command will install `fvm`, and the rest of the `fluvio` development toolchain.

{{% inline-embed file="embeds/download-cli/curl-bash-copy.md" %}}

`fvm` will be installed at `~/.fvm/bin`, and will install `fluvio` and the rest of the development tools at `~/.fluvio/bin`.

You will need to add these directories to your shell's `PATH` environment variable.

{{< h-list tabTotal="3" tabID="1" tabName1="Bash" tabName2="Zsh" tabName3="Fish">}}

{{< h-item tabNum="1">}}
%copy first-line%
```shell
$ echo 'export PATH="${HOME}/.fvm/bin:${HOME}/.fluvio/bin:${PATH}"' >> ~/.bashrc
```
{{< /h-item>}}

{{< h-item tabNum="2">}}
%copy first-line%
```shell
$ echo 'export PATH="${HOME}/.fvm/bin:${HOME}/.fluvio/bin:${PATH}"' >> ~/.zshrc
```
{{< /h-item>}}

{{< h-item tabNum="3">}}
%copy first-line%
```shell
$ fish_add_path ~/.fluvio/bin ~/.fvm/bin
```
{{< /h-item>}}

{{< /h-list>}}

Example output

%copy first-line%
```shell
$ curl -fsS https://hub.infinyon.cloud/install/install.sh | bash

☁️  Downloading fluvio version manager, fvm
   target arch aarch64-apple-darwin
⬇️ Installing fvm
done: FVM installed successfully at /Users/telant/.fvm
help: Add FVM to PATH using source $HOME/.fvm/env
fluvio install dir /Users/$USER/.fluvio
If version of fluvio is already installed, you can run 'fvm install' or 'fvm switch' to change versions
☁️ Installing fluvio
info: Downloading (1/5): fluvio@0.11.0
info: Downloading (2/5): fluvio-cloud@0.2.15
info: Downloading (3/5): fluvio-run@0.11.0
info: Downloading (4/5): cdk@0.11.0
info: Downloading (5/5): smdk@0.11.0
done: Installed fluvio version 0.11.0
done: Now using fluvio version 0.11.0
🎉 Install complete!
fluvio: 💡 You'll need to add '~/.fvm/bin' and ~/.fluvio/bin/' to your PATH variable
fluvio:     You can run the following to set your PATH on shell startup:
fluvio:       echo 'export PATH="${HOME}/.fvm/bin:${HOME}/.fluvio/bin:${PATH}"' >> ~/.zshrc
```