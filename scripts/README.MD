# Website Scripts

Fluvio website is an umbrella project that pulls content from other Fluvio projects, such as:
* [fluvio](https://github.com/infinyon/fluvio)
* [fluvio-cloud](https://github.com/infinyon/fluvio-cloud)
* [flv-client-node](https://github.com/infinyon/flv-client-node)
* [flv-client-rust](https://github.com/infinyon/flv-client-rust)

and generate the website.

## Test Website

The following script copies the content from project into Fluvio Website (but does not check-in)

```
./test-repo.sh

Usage: test-repo.sh -b <branch> -r <repo-name> -d <docs-folder>
    -b  source branch   (master)
    -r  repo name       (fluvio-cloud)
    -d  docs folder     (fluvio-cloud)
```

For example use the following command to copy content form 'fluvio-cloud':

```
./test-repo.sh -b scripts -r fluvio-cloud -d fluvio-cloud
```

## Script Assumptions

The script looks for a 'website' directory located at the top of the **project** directory. The 'website' folder must have 2 sub-directories:

* docs 
* img (optional)

The script copies the content of these directories to **fluvio-website**.

### docs content

Content from 'docs' folder is placed inside 'content/docs/**project-name**' in fluvio-website. The website is rendered using [Hugo](https://gohugo.io/) framework, and the content must be Hugo compatible.

### img content

Content from 'img' folder is placed inside 'static/img/**project-name**' in fluvio-website. To reference an image, use the following shortcode. For example:

```
{{< image src="fluvio-cloud/fluvio-cloud.svg" alt="Fluvio Cloud" justify="center" width="560" type="scaled-90">}}
```

## Script Details

* [TEST-REPO.MD](./TEST-REPO.MD)