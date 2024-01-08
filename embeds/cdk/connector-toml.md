%copy first-line%
```bash
$ cat Connector.toml
[package]
name = "my-connector"
group = "acme"
version = "0.1.0"
apiVersion = "0.1.0"
fluvio = "0.10.0"
description = ""
license = "Apache-2.0"
visibility = "private"

[direction]
source = true

[deployment]
binary = "my-connector"

[custom.properties.foo]
title = "Foo"
description = "Foo"
type = "string"
```