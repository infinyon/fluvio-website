# Website Scripts

Fluvio website is an umbrella project that pulls content from other Fluvio projects, such as:
* [fluvio](https://github.com/infinyon/fluvio)
* [fluvio-cloud](https://github.com/infinyon/fluvio-cloud)
* [flv-client-node](https://github.com/infinyon/flv-client-node)
* [flv-client-rust](https://github.com/infinyon/flv-client-rust)

and generate the website.

## Scripts

```
sync-fluvio.sh
sync-fluvio-cloud.sh
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


## Convenience Scripts

For convenience, each project has an independent script. By default the script pull content from **master** branch. 

```
./sync-fluvio-cloud.sh
```

To pull content from a different branch, specify the branch name (i.e. scripts):

```
./sync-fluvio-cloud.sh scripts
```

## Advanced Script

If you'd like additional customizations, use 'sync-repo.sh'. This script comes with a cli:

```
scripts/sync-repo.sh

Usage: scripts/sync-repo.sh -b <branch> -r <repo-name> -d <docs-folder>
    -b  source branch   (master)
    -r  repo name       (fluvio-cloud)
    -d  docs folder     (fluvio-cloud)
```


## Script Details

* Script step-by-step defined in [SCRIPT.MD](./SCRIPT.MD)