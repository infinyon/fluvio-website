# Fluvio Website

Fluvio website repository stores all documentation published in [fluvio.io](https://fluvio.io).


## Website Organization 

Fluvio website is an **umbrella project** that mixes internal documentation with content form other Fluvio projects. The documentation is stored in `content` directory and it is organized as follows:

```
content/
  main
  blog
  docs
  ...
```

While documentation for `main` and `blog` directory is generated locally, the content for `docs` is a mix of local and external content. The external content is pulled from other projects and placed in the content directory for rendering. The **origin** on the documentation as follows:

|           Folder              |   Type   |         Origin        |
|-------------------------------|----------|-----------------------|
| content/main                  | local    | [fluvio-website](https://github.com/infinyon/fluvio-website) |
| content/blog                  | local    | [fluvio-website](https://github.com/infinyon/fluvio-website) |
| content/docs/architecture     | local    | [fluvio-website](https://github.com/infinyon/fluvio-website) |
| content/docs/cli              | external | [fluvio](https://github.com/infinyon/fluvio) |
| content/docs/fluvio-cloud     | external | [fluvio-cloud](https://github.com/infinyon/fluvio-cloud) |
| content/docs/getting-started  | local    | [fluvio-website](https://github.com/infinyon/fluvio-website)  |
| content/docs/node-api         | external | [flv-client-node](https://github.com/infinyon/flv-client-node)  |
| content/docs/rust-api         | external | [flv-client-rust](https://github.com/infinyon/flv-client-rust)  |
| content/docs/demo-apps/simple-chat-demo-node         | external | [flv-simple-chat-node](https://github.com/infinyon/flv-simple-chat-demo-node)  |
| content/docs/demo-apps/cdc-demo-node         | external | [flv-cdc-demo-node](https://github.com/infinyon/flv-cdc-demo-node)  |
| content/docs/demo-apps/cdc-demo-rust         | external | [flv-cdc-demo-rust](https://github.com/infinyon/flv-cdc-demo-rust)  |

Clone the content from the **origin** project into of **fluvio-website** to test rendering. There is a script available for this purpose at [scripts/README.MD](./scripts/README.MD).

## Website Generator

The website is generated using [Hugo Framework](https://gohugo.io/). 

### Run Website on Local Machine

To run the website on your local machine:

1. [Install Hugo](https://gohugo.io/getting-started/installing/)
2. Run Hugo
    ```
    hugo server --watch --verbose --buildDrafts --cleanDestinationDir --disableFastRender
    ```
3. Website is rendered at
    ```
    http://localhost:1313/
    ```

Hugo watches for file changes and automatically updates website.


## Website Publisher

The website is published on [Netlify](https://www.netlify.com/). Netlify monitors branch `stable` and updates the website when new changes are detected.
