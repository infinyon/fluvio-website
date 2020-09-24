# Documentation for Writers

Fluvio Website was written in Hugo. Checkout [Hugo Documentation](https://gohugo.io/documentation/) for the directory layout and other introductory information. This document describes the formatting and the customizations created for Fluvio website.

## Formatting and Short Codes

The following document has a list of formatting and short code samples: [(https://fluvio.io/docs/samples](https://fluvio.io/docs/samples). The content for the page is generated from the following markdown: [https://github.com/infinyon/fluvio-website/blob/stable/content/docs/samples.md](https://github.com/infinyon/fluvio-website/blob/stable/content/docs/samples.md).


## Tutorial Section

Tutorial is a custom-built section that uses tiles for listing and tags for filtering. The tags are driven the `front matter`, where tag is automatically generated for filtering criteria.

For example, _hello-world.md_ file has 3 tags:

```
---
title: '"Hello World" ...'
desc: ...
group: hello-world
tags:
  - node
  - rust
  - swift
githubAuthors:
  - ...
difficulty: low
weight: 10
```

The `front matter` parameters for the **root** file are defined as follows:

* **title**: header of the tile
* **desc**: description of the tile
* **group**: label that links all programming language related files to this tile
* **tags**: programming language supported by this tile - each programming language must have a separate labeled with the same group name
* **githubAuthors**: github usernames of the authors this tile displayed as github avatars
* **difficulty**: [low, medium, high] controls the difficulty icon of the tile
* **weight**: controls the tile position in the list

Each tile must have at least two files joined by the `group` name:

* **root file**: an empty file such as `hello-world.md` that controls the tile content and defines the group label.
* **programming languages files**: files such as `hello-world-node.md`, `hello-world-rust.md`, etc. - one file per tag. The file name does as the content is joined by the group label.

Each `programming language file` is controlled by `front matter` and an optional `short code`. 

For example, _hello-world-node.md_ has the following `front matter`:

```
---
title: '"Hello World" ...'
hidden: true
group: hello-world
tag: node
weight: 10
toc: true
---
```

The `front matter` parameters for the **programming language** file are defined as follows:

* **title**: header of the file
* **hidden**: all programming language files **must be hidden** otherwise they are displayed as tiles
* **group**: the label that links all programming language to the root file
* **tag**: the programming language associated to this file
* **weight**: the order in which this file should be displayed in the `short-code` described below
* **toc**: if true, the TOC is automatically computed and displayed

The `short code` is a script that generates the the language buttons. In general the following short code should be placed right below teh front matter:

```
{{< lang-selector >}}
```

If there is only one programming language, the short code may be omitted.

**NOTE**: Each programming language has a corresponding icon that must be added to the following directory:

```
<project>/static/img/tiles/
```

and mapped in the following `partial`:

```
layouts/partials/tutorial/print-languages.html
```