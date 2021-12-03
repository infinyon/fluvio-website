---
title: Reading GitHub API data with a SmartModule
menu: GitHub Stars
weight: 20
---

We can use the HTTP Connector to continuously stream data from the GitHub API,
allowing us to observe and react to changes in real-time. However, the GitHub API
typically returns large JSON responses with dozens of fields, many of which are
not useful for all use-cases. Since HTTP is a Smart Connector, we can write a
custom SmartModule that runs directly inside the connector, shaping the responses
to match our exact needs.

### Counting Stars and Forks

Using the GitHub API, we can check on the number of Stars and Forks that
any repository has. We can use the HTTP Smart Connector to create a stream from this
API endpoint, where each record in our stream will be an HTTP response from the API.
Let's look at how to set up the HTTP connector _without_ a SmartModule first, then
we'll look at the fields available, decide which fields we want to keep, then finally
write a SmartModule to help us extract those fields.

We can launch the HTTP Connector as a Managed Connector (preferred for [InfinyOn Cloud][1])
or as a Local Connector. If you don't know which one to pick, we recommend sticking
with a Managed Connector.

You can find the full code for this example in [the fluvio-smartmodule-examples][2] repository.

{{< h-list tabTotal="2" tabID="1" tabName1="Managed Connector" tabName2="Local Connector">}}

{{< h-item tabNum="1">}}
#### Connect to GitHub using HTTP as a Managed Connector

To set up our use-case using a managed connector, we'll need to create a connector
configuration file, which we'll call `connect.yml`. Paste the following contents into
the file:

%copy%
```yaml
# connect.yml
version: v1
name: github-repo
type: http
topic: github-repo
create_topic: true
direction: source
parameters:
  endpoint: https://api.github.com/repos/infinyon/fluvio
  headers: "User-Agent:fluvio-http-example"
  interval: 30
```

This configuration is using the `http` connector type, will produce events to a topic
called `github-repo` (creating the topic if it does not exist), and will send HTTP
requests to `https://api.github.com/repos/infinyon/fluvio` every 30 seconds.

To use this configuration, run the following command:

%copy first-line%
```bash
$ fluvio connector create --config=./connect.yml
```

### Checking out the data

Now that we have our HTTP connector scraping GitHub, we can check our topic to see the
data we're receiving:

%copy first-line%
```bash
$ fluvio consume github-repo -B -d
```

You should see each record is a JSON object that looks something like this:

```json
{
  "id": 205473061,
  "node_id": "MDEwOlJlcG9zaXRvcnkyMDU0NzMwNjE=",
  "name": "fluvio",
  "full_name": "infinyon/fluvio",
  "private": false,
  "owner": {
    "login": "infinyon",
    "id": 52172389,
    "node_id": "MDEyOk9yZ2FuaXphdGlvbjUyMTcyMzg5",
    "avatar_url": "https://avatars.githubusercontent.com/u/52172389?v=4",
    "gravatar_id": "",
    "url": "https://api.github.com/users/infinyon",
    "html_url": "https://github.com/infinyon",
    "followers_url": "https://api.github.com/users/infinyon/followers",
    "following_url": "https://api.github.com/users/infinyon/following{/other_user}",
    "gists_url": "https://api.github.com/users/infinyon/gists{/gist_id}",
    "starred_url": "https://api.github.com/users/infinyon/starred{/owner}{/repo}",
    "subscriptions_url": "https://api.github.com/users/infinyon/subscriptions",
    "organizations_url": "https://api.github.com/users/infinyon/orgs",
    "repos_url": "https://api.github.com/users/infinyon/repos",
    "events_url": "https://api.github.com/users/infinyon/events{/privacy}",
    "received_events_url": "https://api.github.com/users/infinyon/received_events",
    "type": "Organization",
    "site_admin": false
  },
  "html_url": "https://github.com/infinyon/fluvio",
  "description": "Programmable platform for data in motion",
  "fork": false,
  "url": "https://api.github.com/repos/infinyon/fluvio",
  "forks_url": "https://api.github.com/repos/infinyon/fluvio/forks",
  "keys_url": "https://api.github.com/repos/infinyon/fluvio/keys{/key_id}",
  "collaborators_url": "https://api.github.com/repos/infinyon/fluvio/collaborators{/collaborator}",
  "teams_url": "https://api.github.com/repos/infinyon/fluvio/teams",
  "hooks_url": "https://api.github.com/repos/infinyon/fluvio/hooks",
  "issue_events_url": "https://api.github.com/repos/infinyon/fluvio/issues/events{/number}",
  "events_url": "https://api.github.com/repos/infinyon/fluvio/events",
  "assignees_url": "https://api.github.com/repos/infinyon/fluvio/assignees{/user}",
  "branches_url": "https://api.github.com/repos/infinyon/fluvio/branches{/branch}",
  "tags_url": "https://api.github.com/repos/infinyon/fluvio/tags",
  "blobs_url": "https://api.github.com/repos/infinyon/fluvio/git/blobs{/sha}",
  "git_tags_url": "https://api.github.com/repos/infinyon/fluvio/git/tags{/sha}",
  "git_refs_url": "https://api.github.com/repos/infinyon/fluvio/git/refs{/sha}",
  "trees_url": "https://api.github.com/repos/infinyon/fluvio/git/trees{/sha}",
  "statuses_url": "https://api.github.com/repos/infinyon/fluvio/statuses/{sha}",
  "languages_url": "https://api.github.com/repos/infinyon/fluvio/languages",
  "stargazers_url": "https://api.github.com/repos/infinyon/fluvio/stargazers",
  "contributors_url": "https://api.github.com/repos/infinyon/fluvio/contributors",
  "subscribers_url": "https://api.github.com/repos/infinyon/fluvio/subscribers",
  "subscription_url": "https://api.github.com/repos/infinyon/fluvio/subscription",
  "commits_url": "https://api.github.com/repos/infinyon/fluvio/commits{/sha}",
  "git_commits_url": "https://api.github.com/repos/infinyon/fluvio/git/commits{/sha}",
  "comments_url": "https://api.github.com/repos/infinyon/fluvio/comments{/number}",
  "issue_comment_url": "https://api.github.com/repos/infinyon/fluvio/issues/comments{/number}",
  "contents_url": "https://api.github.com/repos/infinyon/fluvio/contents/{+path}",
  "compare_url": "https://api.github.com/repos/infinyon/fluvio/compare/{base}...{head}",
  "merges_url": "https://api.github.com/repos/infinyon/fluvio/merges",
  "archive_url": "https://api.github.com/repos/infinyon/fluvio/{archive_format}{/ref}",
  "downloads_url": "https://api.github.com/repos/infinyon/fluvio/downloads",
  "issues_url": "https://api.github.com/repos/infinyon/fluvio/issues{/number}",
  "pulls_url": "https://api.github.com/repos/infinyon/fluvio/pulls{/number}",
  "milestones_url": "https://api.github.com/repos/infinyon/fluvio/milestones{/number}",
  "notifications_url": "https://api.github.com/repos/infinyon/fluvio/notifications{?since,all,participating}",
  "labels_url": "https://api.github.com/repos/infinyon/fluvio/labels{/name}",
  "releases_url": "https://api.github.com/repos/infinyon/fluvio/releases{/id}",
  "deployments_url": "https://api.github.com/repos/infinyon/fluvio/deployments",
  "created_at": "2019-08-31T00:11:58Z",
  "updated_at": "2021-12-02T07:14:17Z",
  "pushed_at": "2021-12-02T07:53:14Z",
  "git_url": "git://github.com/infinyon/fluvio.git",
  "ssh_url": "git@github.com:infinyon/fluvio.git",
  "clone_url": "https://github.com/infinyon/fluvio.git",
  "svn_url": "https://github.com/infinyon/fluvio",
  "homepage": "https://www.fluvio.io/",
  "size": 14667,
  "stargazers_count": 790,
  "watchers_count": 790,
  "language": "Rust",
  "has_issues": true,
  "has_projects": true,
  "has_downloads": true,
  "has_wiki": true,
  "has_pages": false,
  "forks_count": 50,
  "mirror_url": null,
  "archived": false,
  "disabled": false,
  "open_issues_count": 103,
  "license": {
    "key": "apache-2.0",
    "name": "Apache License 2.0",
    "spdx_id": "Apache-2.0",
    "url": "https://api.github.com/licenses/apache-2.0",
    "node_id": "MDc6TGljZW5zZTI="
  },
  "allow_forking": true,
  "is_template": false,
  "topics": [
    "cloud-native",
    "rust",
    "rust-lang",
    "streaming"
  ],
  "visibility": "public",
  "forks": 50,
  "open_issues": 103,
  "watchers": 790,
  "default_branch": "master",
  "temp_clone_token": null,
  "organization": {
    "login": "infinyon",
    "id": 52172389,
    "node_id": "MDEyOk9yZ2FuaXphdGlvbjUyMTcyMzg5",
    "avatar_url": "https://avatars.githubusercontent.com/u/52172389?v=4",
    "gravatar_id": "",
    "url": "https://api.github.com/users/infinyon",
    "html_url": "https://github.com/infinyon",
    "followers_url": "https://api.github.com/users/infinyon/followers",
    "following_url": "https://api.github.com/users/infinyon/following{/other_user}",
    "gists_url": "https://api.github.com/users/infinyon/gists{/gist_id}",
    "starred_url": "https://api.github.com/users/infinyon/starred{/owner}{/repo}",
    "subscriptions_url": "https://api.github.com/users/infinyon/subscriptions",
    "organizations_url": "https://api.github.com/users/infinyon/orgs",
    "repos_url": "https://api.github.com/users/infinyon/repos",
    "events_url": "https://api.github.com/users/infinyon/events{/privacy}",
    "received_events_url": "https://api.github.com/users/infinyon/received_events",
    "type": "Organization",
    "site_admin": false
  },
  "network_count": 50,
  "subscribers_count": 21
}
```

As we can see, the data we want is in here (`stargazers_count` and `forks_count`),
but it's buried under the slew of other fields that we don't need for our use-case. If we kept a
long-lived stream of this data running, we'd be using hundreds of times the bandwidth necessary.

Let's see how we can use a custom SmartModule to select just these three fields we're interested
in to create a lean and lightweight stream that meets our needs.

### Using a SmartModule to shape GitHub data

Now we're going to take a look at how a SmartModule can help us slim down our HTTP responses.
We can use a `cargo-generate` template to get a starter SmartModule project going to work with.
If you want to see the full code for this example, you can check it out
[in the fluvio-smartmodule-examples repo][2].

If you don't have it installed, let's install `cargo-generate` with the following command:

%copy first-line%
```bash
$ cargo install cargo-generate
```

Then, let's create a new project using the `map` template:

%copy first-line%
```bash
$ cargo generate --git=https://github.com/infinyon/fluvio-smartmodule-template
⚠️   Unable to load config file: ~/.cargo/cargo-generate.toml
🤷   Project Name : github-stars
🔧   Generating template ...
✔ 🤷   Which type of SmartModule would you like? · map
[1/7]   Done: .cargo/config.toml
[2/7]   Done: .cargo
[3/7]   Done: .gitignore
[4/7]   Done: Cargo.toml
[5/7]   Done: README.md
[6/7]   Done: src/lib.rs
[7/7]   Done: src
🔧   Moving generated files into: `github-stars`...
✨   Done! New project created github-stars
```

The Map type of SmartModule is used to define a transformation from the
input record to a new output record. We'll use the following code to select
the `stargazers_count` and `forks_count` fields from the input JSON record.
Copy and paste this code into the `src/lib.rs` file of the project.

%copy%
```rust
use fluvio_smartmodule::{smartmodule, Record, RecordData, Result};
use serde_json::{json, Value as JsonValue};

#[smartmodule(map)]
fn shaper(record: &Record) -> Result<(Option<RecordData>, RecordData)> {
    let repo: JsonValue = serde_json::from_slice(record.value.as_ref())?;

    let shaped = json!({
        "stars": repo["stargazers_count"],
        "forks": repo["forks_count"],
    });

    let output = serde_json::to_vec(&shaped)?;
    Ok((record.key.clone(), RecordData::from(output)))
}
```

Here we're using a [SmartModule Map][3], which is called once for each record in the input stream,
transforms the record, and returns it to the output stream. In this example, we're parsing the
record value as a JSON value, then selecting just the `stargazers_count` and `forks_count`
fields to include in our output object. Finally, we re-serialize the JSON and return it!

Let's take this for a spin and use the SmartModule directly inside the HTTP connector.

### Registering and using the SmartModule

Since we're using a SmartModule from source, the next thing we need to do is compile it!

%copy first-line%
```bash
$ cargo build --release
```

Next, we need to register this SmartModule using the `fluvio smartmodule` command, giving it a
name that we can use to refer to it later.

%copy first-line%
```bash
$ fluvio smartmodule create github-smartmodule --wasm-file=target/wasm32-unknown-unknown/release/github_stars.wasm
```

At this point, our SmartModule has been registered and named `github-smartmodule`. Now, we can
return to our connector setup and re-launch the HTTP Connector with our SmartModule!

#### With Managed Connector

If you're following along with a Managed Connector, the first thing we need to do is stop the
connector we started previously, and delete the topic since it contains old data.

%copy first-line%
```bash
$ fluvio connector delete github-repo
```

%copy first-line%
```bash
$ fluvio topic delete github-repo
```

Then, we can edit the `connect.yml` file and tell it to use our SmartModule as a Map:

%copy%
{{< highlight yaml "hl_lines=12" >}}
# connect.yml
version: v1
name: github-repo
type: http
topic: github-repo
create_topic: true
direction: source
parameters:
  endpoint: https://api.github.com/repos/infinyon/fluvio
  headers: "User-Agent:fluvio-http-example"
  interval: 30
  map: github-smartmodule
{{</ highlight >}}

Finally, let's restart our connector, and we should see that our data arrives in our topic
transformed!

%copy first-line%
```bash
$ fluvio connector create --config=./connect.yml
```


{{</ h-item >}}

{{< h-item tabNum="2">}}
#### Connect to GitHub using HTTP as a Local Connector

Local connectors are run using `docker`. Unlike Managed Connectors, they do not
create topics if they don't exist, so the first thing we want to do is create our topic:

%copy first-line%
```bash
$ fluvio topic create github-repo
```

Now to run HTTP as a local connector with the same configuration options as above,
we can simply run the following command:

%copy%
```bash
docker run -d --name="github-repo" \
    -v"$HOME/.fluvio/config:/home/fluvio/.fluvio/config" \
    -t infinyon/fluvio-connect-http \
    -- \
    --endpoint="https://api.github.com/repos/infinyon/fluvio" \
    --fluvio-topic="github-repo" \
    --interval=30 \
    --header="User-Agent:fluvio-http-example"
```

### Checking out the data

Now that we have our HTTP connector scraping GitHub, we can check our topic to see the
data we're receiving:

%copy first-line%
```bash
$ fluvio consume github-repo -B -d
```

You should see each record is a JSON object that looks something like this:

```json
{
  "id": 205473061,
  "node_id": "MDEwOlJlcG9zaXRvcnkyMDU0NzMwNjE=",
  "name": "fluvio",
  "full_name": "infinyon/fluvio",
  "private": false,
  "owner": {
    "login": "infinyon",
    "id": 52172389,
    "node_id": "MDEyOk9yZ2FuaXphdGlvbjUyMTcyMzg5",
    "avatar_url": "https://avatars.githubusercontent.com/u/52172389?v=4",
    "gravatar_id": "",
    "url": "https://api.github.com/users/infinyon",
    "html_url": "https://github.com/infinyon",
    "followers_url": "https://api.github.com/users/infinyon/followers",
    "following_url": "https://api.github.com/users/infinyon/following{/other_user}",
    "gists_url": "https://api.github.com/users/infinyon/gists{/gist_id}",
    "starred_url": "https://api.github.com/users/infinyon/starred{/owner}{/repo}",
    "subscriptions_url": "https://api.github.com/users/infinyon/subscriptions",
    "organizations_url": "https://api.github.com/users/infinyon/orgs",
    "repos_url": "https://api.github.com/users/infinyon/repos",
    "events_url": "https://api.github.com/users/infinyon/events{/privacy}",
    "received_events_url": "https://api.github.com/users/infinyon/received_events",
    "type": "Organization",
    "site_admin": false
  },
  "html_url": "https://github.com/infinyon/fluvio",
  "description": "Programmable platform for data in motion",
  "fork": false,
  "url": "https://api.github.com/repos/infinyon/fluvio",
  "forks_url": "https://api.github.com/repos/infinyon/fluvio/forks",
  "keys_url": "https://api.github.com/repos/infinyon/fluvio/keys{/key_id}",
  "collaborators_url": "https://api.github.com/repos/infinyon/fluvio/collaborators{/collaborator}",
  "teams_url": "https://api.github.com/repos/infinyon/fluvio/teams",
  "hooks_url": "https://api.github.com/repos/infinyon/fluvio/hooks",
  "issue_events_url": "https://api.github.com/repos/infinyon/fluvio/issues/events{/number}",
  "events_url": "https://api.github.com/repos/infinyon/fluvio/events",
  "assignees_url": "https://api.github.com/repos/infinyon/fluvio/assignees{/user}",
  "branches_url": "https://api.github.com/repos/infinyon/fluvio/branches{/branch}",
  "tags_url": "https://api.github.com/repos/infinyon/fluvio/tags",
  "blobs_url": "https://api.github.com/repos/infinyon/fluvio/git/blobs{/sha}",
  "git_tags_url": "https://api.github.com/repos/infinyon/fluvio/git/tags{/sha}",
  "git_refs_url": "https://api.github.com/repos/infinyon/fluvio/git/refs{/sha}",
  "trees_url": "https://api.github.com/repos/infinyon/fluvio/git/trees{/sha}",
  "statuses_url": "https://api.github.com/repos/infinyon/fluvio/statuses/{sha}",
  "languages_url": "https://api.github.com/repos/infinyon/fluvio/languages",
  "stargazers_url": "https://api.github.com/repos/infinyon/fluvio/stargazers",
  "contributors_url": "https://api.github.com/repos/infinyon/fluvio/contributors",
  "subscribers_url": "https://api.github.com/repos/infinyon/fluvio/subscribers",
  "subscription_url": "https://api.github.com/repos/infinyon/fluvio/subscription",
  "commits_url": "https://api.github.com/repos/infinyon/fluvio/commits{/sha}",
  "git_commits_url": "https://api.github.com/repos/infinyon/fluvio/git/commits{/sha}",
  "comments_url": "https://api.github.com/repos/infinyon/fluvio/comments{/number}",
  "issue_comment_url": "https://api.github.com/repos/infinyon/fluvio/issues/comments{/number}",
  "contents_url": "https://api.github.com/repos/infinyon/fluvio/contents/{+path}",
  "compare_url": "https://api.github.com/repos/infinyon/fluvio/compare/{base}...{head}",
  "merges_url": "https://api.github.com/repos/infinyon/fluvio/merges",
  "archive_url": "https://api.github.com/repos/infinyon/fluvio/{archive_format}{/ref}",
  "downloads_url": "https://api.github.com/repos/infinyon/fluvio/downloads",
  "issues_url": "https://api.github.com/repos/infinyon/fluvio/issues{/number}",
  "pulls_url": "https://api.github.com/repos/infinyon/fluvio/pulls{/number}",
  "milestones_url": "https://api.github.com/repos/infinyon/fluvio/milestones{/number}",
  "notifications_url": "https://api.github.com/repos/infinyon/fluvio/notifications{?since,all,participating}",
  "labels_url": "https://api.github.com/repos/infinyon/fluvio/labels{/name}",
  "releases_url": "https://api.github.com/repos/infinyon/fluvio/releases{/id}",
  "deployments_url": "https://api.github.com/repos/infinyon/fluvio/deployments",
  "created_at": "2019-08-31T00:11:58Z",
  "updated_at": "2021-12-02T07:14:17Z",
  "pushed_at": "2021-12-02T07:53:14Z",
  "git_url": "git://github.com/infinyon/fluvio.git",
  "ssh_url": "git@github.com:infinyon/fluvio.git",
  "clone_url": "https://github.com/infinyon/fluvio.git",
  "svn_url": "https://github.com/infinyon/fluvio",
  "homepage": "https://www.fluvio.io/",
  "size": 14667,
  "stargazers_count": 790,
  "watchers_count": 790,
  "language": "Rust",
  "has_issues": true,
  "has_projects": true,
  "has_downloads": true,
  "has_wiki": true,
  "has_pages": false,
  "forks_count": 50,
  "mirror_url": null,
  "archived": false,
  "disabled": false,
  "open_issues_count": 103,
  "license": {
    "key": "apache-2.0",
    "name": "Apache License 2.0",
    "spdx_id": "Apache-2.0",
    "url": "https://api.github.com/licenses/apache-2.0",
    "node_id": "MDc6TGljZW5zZTI="
  },
  "allow_forking": true,
  "is_template": false,
  "topics": [
    "cloud-native",
    "rust",
    "rust-lang",
    "streaming"
  ],
  "visibility": "public",
  "forks": 50,
  "open_issues": 103,
  "watchers": 790,
  "default_branch": "master",
  "temp_clone_token": null,
  "organization": {
    "login": "infinyon",
    "id": 52172389,
    "node_id": "MDEyOk9yZ2FuaXphdGlvbjUyMTcyMzg5",
    "avatar_url": "https://avatars.githubusercontent.com/u/52172389?v=4",
    "gravatar_id": "",
    "url": "https://api.github.com/users/infinyon",
    "html_url": "https://github.com/infinyon",
    "followers_url": "https://api.github.com/users/infinyon/followers",
    "following_url": "https://api.github.com/users/infinyon/following{/other_user}",
    "gists_url": "https://api.github.com/users/infinyon/gists{/gist_id}",
    "starred_url": "https://api.github.com/users/infinyon/starred{/owner}{/repo}",
    "subscriptions_url": "https://api.github.com/users/infinyon/subscriptions",
    "organizations_url": "https://api.github.com/users/infinyon/orgs",
    "repos_url": "https://api.github.com/users/infinyon/repos",
    "events_url": "https://api.github.com/users/infinyon/events{/privacy}",
    "received_events_url": "https://api.github.com/users/infinyon/received_events",
    "type": "Organization",
    "site_admin": false
  },
  "network_count": 50,
  "subscribers_count": 21
}
```

As we can see, the data we want is in here (`stargazers_count` and `forks_count`),
but it's buried under the slew of other fields that we don't need for our use-case. If we kept a
long-lived stream of this data running, we'd be using hundreds of times the bandwidth necessary.

Let's see how we can use a custom SmartModule to select just these three fields we're interested
in to create a lean and lightweight stream that meets our needs.

### Using a SmartModule to shape GitHub data

Now we're going to take a look at how a SmartModule can help us slim down our HTTP responses.
We can use a `cargo-generate` template to get a starter SmartModule project going to work with.
If you want to see the full code for this example, you can check it out
[in the fluvio-smartmodule-examples repo][2].

If you don't have it installed, let's install `cargo-generate` with the following command:

%copy first-line%
```bash
$ cargo install cargo-generate
```

Then, let's create a new project using the `map` template:

%copy first-line%
```bash
$ cargo generate --git=https://github.com/infinyon/fluvio-smartmodule-template
⚠️   Unable to load config file: ~/.cargo/cargo-generate.toml
🤷   Project Name : github-stars
🔧   Generating template ...
✔ 🤷   Which type of SmartModule would you like? · map
[1/7]   Done: .cargo/config.toml
[2/7]   Done: .cargo
[3/7]   Done: .gitignore
[4/7]   Done: Cargo.toml
[5/7]   Done: README.md
[6/7]   Done: src/lib.rs
[7/7]   Done: src
🔧   Moving generated files into: `github-stars`...
✨   Done! New project created github-stars
```

The Map type of SmartModule is used to define a transformation from the
input record to a new output record. We'll use the following code to select
the `stargazers_count` and `forks_count` fields from the input JSON record.
Copy and paste this code into the `src/lib.rs` file of the project.

%copy%
```rust
use fluvio_smartmodule::{smartmodule, Record, RecordData, Result};
use serde_json::{json, Value as JsonValue};

#[smartmodule(map)]
fn shaper(record: &Record) -> Result<(Option<RecordData>, RecordData)> {
    let repo: JsonValue = serde_json::from_slice(record.value.as_ref())?;

    let shaped = json!({
        "stars": repo["stargazers_count"],
        "forks": repo["forks_count"],
    });

    let output = serde_json::to_vec(&shaped)?;
    Ok((record.key.clone(), RecordData::from(output)))
}
```

Here we're using a [SmartModule Map][3], which is called once for each record in the input stream,
transforms the record, and returns it to the output stream. In this example, we're parsing the
record value as a JSON value, then selecting just the `stargazers_count` and `forks_count`
fields to include in our output object. Finally, we re-serialize the JSON and return it!

Let's take this for a spin and use the SmartModule directly inside the HTTP connector.

### Registering and using the SmartModule

Since we're using a SmartModule from source, the next thing we need to do is compile it!

%copy first-line%
```bash
$ cargo build --release
```

Next, we need to register this SmartModule using the `fluvio smartmodule` command, giving it a
name that we can use to refer to it later.

%copy first-line%
```bash
$ fluvio smartmodule create github-smartmodule --wasm-file=target/wasm32-unknown-unknown/release/github_stars.wasm
```

At this point, our SmartModule has been registered and named `github-smartmodule`. Now, we can
return to our connector setup and re-launch the HTTP Connector with our SmartModule!

#### With Local Connector

Using a Local Connector, we'll need to stop and restart the connector as well.
To stop the connector, run the following command:

%copy first-line%
```bash
$ docker kill github-repo; docker rm github-repo
```

Then, we can re-launch it with a new command that also specifies the SmartModule we'd like
to use. Here, we'll add the `--map` flag with the name of our SmartModule:

%copy%
{{< highlight bash "hl_lines=9" >}}
docker run -d --name="github-repo" \
    -v"$HOME/.fluvio/config:/home/fluvio/.fluvio/config" \
    -t infinyon/fluvio-connect-http \
    -- \
    --endpoint="https://api.github.com/repos/infinyon/fluvio" \
    --fluvio-topic="github-repo" \
    --interval=30 \
    --header="User-Agent:fluvio-http-example" \
    --map="github-smartmodule"
{{</ highlight >}}

{{</ h-item >}}

{{</ h-list >}}

### Checking the Topic Data

Now that we have our Connector running with our SmartModule loaded up, we should be able
to see that our topic has the transformed data we expect, JSON records with only the two
fields we're interested in.

%copy first-line%
```bash
$ fluvio consume github-repo
Consuming records from the end of topic 'github-repo'. This will wait for new records
{"forks":50,"stars":790}
{"forks":50,"stars":790}
...
```

Now we can see that our topic contains just the data we selected!

[1]: https://infinyon.cloud/signup
[2]: https://github.com/infinyon/fluvio-smartmodule-examples/blob/master/github-stars/src/lib.rs
[3]: /docs/smartmodules/map
