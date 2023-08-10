# Fluvio Website

Fluvio website repository stores all documentation published in [fluvio.io](https://fluvio.io).

## Run Website on Local Machine

The website is generated using [Hugo Framework](https://gohugo.io/). To run the website on your local machine:

1. [Install Hugo](https://gohugo.io/getting-started/installing/)
2. Run Hugo
    ```
   cargo run -- hugo
    ```
3. Website is rendered at
    ```
    http://localhost:1313/
    ```

4. Checkout [WRITERS.md](./WRITERS.md) for formatting and Fluvio customizations.

Hugo watches for file changes and automatically updates website.


## Public/Nightly Websites

[Cloudflare Pages](https://pages.cloudflare.com/) watches the default branch, and updates the live site automatically after merge

* `master` updates [fluvio.io](https://fluvio.io)

Preview pages are created for each PR. Their URLS are dynamic, based on the name of your branch + `.fluvio-website-preview.pages.dev`

For more info read: https://developers.cloudflare.com/pages/platform/preview-deployments/#preview-aliases

Or ask someone with access to the Pages dashboard to retrieve your preview URL

## Connector-beta reference docs

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