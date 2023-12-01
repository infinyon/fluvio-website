# Fluvio Website

## Public/Nightly Websites

[Cloudflare Pages](https://pages.cloudflare.com/) watches the default branch, and updates the live site automatically after merge

* master branch - [preview environment](https://master.fluvio-website-preview.pages.dev/)
* stable branch - [prod environment (fluvio.io)](https://fluvio.io)

If you create branches in this repo (i.e., not on a fork), Cloudflare will build and provide a url to the Hugo site for each PR. Their URLS are dynamic, based on the name of your branch + `.fluvio-website-preview.pages.dev`

For more info read: https://developers.cloudflare.com/pages/platform/preview-deployments/#preview-aliases

## Run Website on Local Machine

The website is generated using [Hugo Framework](https://gohugo.io/). To run the website on your local machine:

1. [Install Hugo](https://gohugo.io/getting-started/installing/)
2. Run Hugo
    ```shell
    cargo run -- hugo 
    ```
3. Website is rendered at
    ```
    http://localhost:1313/
    ```

4. Checkout [WRITERS.md](./WRITERS.md) for formatting and Fluvio customizations.

Hugo watches for file changes and automatically updates website.


## Connector reference docs

A preview of the `fluvio-cms` CLI can be used to update the reference docs for connectors in the Hub.

Connectors are organized in`scripts/fluvio-cms/src/connectors/mod.rs`, and define:
- Protocol used by connector
- Direction of the data flow with respect to a Fluvio cluster.
- Location in Hugo repo where connector README is stored 
- Location in Hugo repo where the content template is stored

At the time of this writing, the latest connector docs are collected by parsing the output of `fluvio hub connector list`

### Add new connector docs

2 areas need to be updated before `fluvio-cms` will support a new connector's docs

1. For new protocols, add to the `DataService` enum.
2. For new connectors, add to the `OfficialConnector` enum

### Update existing connector docs

Downloading from the public git repo:

Ex. Download MQTT docs from github repo
```shell
$ cargo run -- connector --service mqtt --direction inbound
```

Using README from local git clone:

Ex. Copy SQL docs
```shell
$ cargo run -- connector --service sql --direction outbound --file /path/to/README
```

## CLI docs

### Update help text

The list of CLI commands is located at `/data/cli-commands.yml`

Running this command will iterate through the list of CLI commands and capture the output of the `--help` flag

```shell
$ cargo run -- cli
```

This creates files in `/embed/cli/help` with the filename pattern `<cmd>[-<subcmd1>][-<subcmd2][-subcmd3etc].md`. The help output for the command `fluvio cluster list` would be located at `/embed/cli/help/fluvio-cluster-list.md`

The templates for the CLI pages in https://www.fluvio.io/cli/ are not generated. Any commands that are added or removed may need to modify additional files.