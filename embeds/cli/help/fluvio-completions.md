```
Generate command-line completions for Fluvio

Run the following two commands to enable fluvio command completions.

Open a new terminal for the changes to take effect.

$ fluvio completions bash > ~/fluvio_completions.sh 
$ echo "source ~/fluvio_completions.sh" >> ~/.bashrc

Usage: fluvio completions <COMMAND>

Commands:
  bash  Generate CLI completions for bash
  zsh   Generate CLI completions for zsh
  fish  Generate CLI completions for fish

Options:
  -h, --help
          Print help (see a summary with '-h')
```