---
title: Completion
weight: 80
toc: false
---

This command generates CLI completions for bash and fish, by specifying either bash or fish as a subcommand.

```
fluvio completions
Generate command-line completions for Fluvio

USAGE:
    fluvio completions <SUBCOMMAND>

FLAGS:
    -h, --help    Prints help information

SUBCOMMANDS:
    bash    Generate CLI completions for bash
    fish    Generate CLI completions for fish
    help    Prints this message or the help of the given subcommand(s)
```

Example usage for bash shell. Note that we are piping the output of the command to a shell script. We then add the script to the ~/.bashrc file. One must open a new terminal for the changes to take effect.

%copy first-line%
```bash
$ fluvio completions bash > ~/fluvio_completions.sh

$ echo "source ~/fluvio_completions.sh" >> ~/.bashrc
```