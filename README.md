# Fluvio Website

Fluvio website repository stores all documentation published in [fluvio.io](https://fluvio.io).


## Website Organization 

Fluvio website is an umbrella project that mixes internal documentation with content form other Fluvio projects.

The documentation is found in `content` directory and it is organized as follows:

```
main
blog
docs
...
```

While the documentation for `main` and `blog` is is generated locally, the content for `docs` is pulled from different projects.

The origin on the documentation as follows:

|        Folder         |         Origin        |
|-----------------------|-----------------------|
| main                  |  [fluvio-website](https://github.com/infinyon/fluvio-website) |
| blog                  |  [fluvio-website](https://github.com/infinyon/fluvio-website) |
| docs/architecture     |  [fluvio-website](https://github.com/infinyon/fluvio-website) |
| docs/cli              |  [fluvio](https://github.com/infinyon/fluvio) |
| docs/fluvio-cloud     |  [fluvio-cloud](https://github.com/infinyon/fluvio-cloud) |
| docs/getting-started  |  [fluvio-website](https://github.com/infinyon/fluvio-website)  |
| docs/node-api         |  [flv-client-node](https://github.com/infinyon/flv-client-node)  |
| docs/rust-api         |  [flv-client-rust](https://github.com/infinyon/flv-client-rust)  |



## Website Generator

The website is generated using [Hugo Framework](https://gohugo.io/). 


## Website Publisher

The website is published on [Netlify](https://www.netlify.com/). Netlify monitors branch `stable` and updates the website when new changes are detected.
