Next you will have to add Fluvio to your `$PATH` environment variable. There are three main shells that see common usage: Bash, Zsh, and Fish. Bash and Zsh use a similar approach to modifying the `$PATH` but Fish is completely different.

If you are unsure of which shell you are using, the command `echo $0` will tell you:

%copy first-line%
```bash
$ echo$0
/bin/zsh
```

Once you confirm which shell you are using, use one of the following commands to modify your `$PATH`.

{{< h-list tabTotal="3" tabID="1" tabName1="Zsh" tabName2="Bash" tabName3="Fish">}}

{{< h-item tabNum="1">}}

%copy first-line%
```zsh
$ echo 'export PATH="$PATH:~/.fluvio/bin"' >> .zshrc
```

{{</ h-item >}}


{{< h-item tabNum="2" >}}

%copy first-line%
```bash
$ echo 'export PATH="$PATH:$HOME/.fluvio/bin"' >> .bashrc
```
{{</ h-item >}}

{{< h-item tabNum="3" >}}

%copy first-line%
```fish
$ fish_add_path ~/.fluvio/bin
```
{{</ h-item >}}
