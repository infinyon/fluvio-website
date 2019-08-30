---
title: Version
weight: 100
---

As Fluvio evolves, the Command Line Interface (CLI) will also change to support additional features. We intend to keep Fluvio backward compatible with several release of the Command Line tool. To get the most of the product, the Fluvio Command Line and Fluvio deployment should run the same version.

## Check Version

Each component of the system, Fluvio CLI, Streaming Controller (SC), and Streaming Processing Unit (SPU) have a command line interface. The system ensures that SC and SPU will run the same version and it is sufficient to check the CLI and the SC to ensure the version numbers match.  

Check Fluvio CLI __version__ number:

{{< cli yaml >}}
$ fluvio --version
fluvio 0.1.0-alpha.1
{{< /cli >}}

Match against SC server __version__ number:

{{< cli yaml >}}
$ sc-server --version
sc-server 0.1.0-alpha.1
{{< /cli >}}

The same command is also available on the SPU.

