# Fluvio Website

Fluvio website repository stores all documentation published in [fluvio.io](https://fluvio.io).

## Run Website on Local Machine

The website is generated using [Hugo Framework](https://gohugo.io/). To run the website on your local machine:

1. [Install Hugo](https://gohugo.io/getting-started/installing/)
2. Run Hugo
    ```
   ./hugo-start.sh
    ```
3. Website is rendered at
    ```
    http://localhost:1313/
    ```

Hugo watches for file changes and automatically updates website.


## Public/Nightly Websites

[Netlify](https://www.netlify.com/) watches the following branches and automatically updates websites:

* `stable` updates [fluvio.io](https://fluvio.io)
* `master` updates [nightly.fluvio.io](https://nightly.fluvio.io)
