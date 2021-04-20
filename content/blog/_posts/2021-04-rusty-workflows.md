---
title: GitHub Actions best practices for Rust projects
author:
    name: "Nick Mosher"
    github: "nicholastmosher"
description: "How I cleaned up Rust CI boilerplate using the GitHub Actions build matrix"
date: 2021-04-15
slug: github-actions-best-practices
url: /blog/2021/04/github-actions-best-practices
twitter-card: summary_large_image
code:
    height: 9000
---

[Fluvio] is a high-performance distributed streaming platform written in Rust.
As a fairly large project, we have a lot of build configurations and testing
scenarios that we automate in order to make sure we don't break things by
accident. We've been using GitHub Actions in order to run our CI/CD workflows,
but as we've grown, things have naturally gotten messy over time. This week,
I took some time to re-visit our workflow definitions to clean things up and
try to increase our team's productivity. I specifically want to talk about
two main improvements I worked on:

[Fluvio]: https://github.com/infinyon/fluvio/

- Consolidating multiple jobs using the build matrix
- Setting up `sccache` to improve our building and testing speed
  - We actually had already set up `sccache` but it was misconfigured.
    I'll talk about how to check that everything is set up properly.

The first half of this post should be generally useful for anybody who needs
to use GitHub Actions and wants to learn more about the build matrix. The
second half should be useful to Rust developers who want a good starting point
for a solid CI setup.

## Using the GitHub workflows build matrix

The reason I was working on workflows this week was because our CI build and
test time had grown to a point that it was interfering with our team's ability
to move quickly. When I started reading through our workflow definitions, what
I saw was a lot of independent jobs with a lot of duplicated steps. Most of the
jobs would install the Rust toolchain, install [`sccache`], cache the Cargo
registry and the sccache directory, and then run a single task from our project
Makefile. The boilerplate to set up each of these jobs came out to at least
60 lines of configuration. I won't post any of the "before" workflow code here,
but if you are interested in seeing it you can [look at this old commit].

[look at this old commit]: https://github.com/infinyon/fluvio/blob/6eced8a04a41552e4c5276c26a8300c00c990007/.github/workflows/ci.yml

Instead, I'm going to show you the _new_ job definition, and the matrix setup
that goes with it. One thing I learned while doing this is that GitHub's
workflow documentation does not really give the matrix feature justice because
they use such simple examples. Here's the matrix definition I came up with for
our new job definition. I'll briefly explain how the matrix feature works in
case you are unfamiliar with it:

```yaml
# .github/workflows/ci.yml
tests:
  name: ${{ matrix.make.name }} (${{ matrix.os }})
  runs-on: ${{ matrix.os }}
  strategy:
    fail-fast: false
    matrix:
      os: [ubuntu-latest, macos-latest]
      rust: [stable]
      make:
        - name: Clippy
          task: "check-clippy"
        - name: Unit tests
          task: "build-all-test run-all-unit-test"
        - name: Doc tests
          task: "run-all-doc-test"
      include:
        - os: ubuntu-latest
          sccache-path: /home/runner/.cache/sccache
        - os: macos-latest
          sccache-path: /Users/runner/Library/Caches/Mozilla.sccache
      exclude:
        - os: macos-latest
          rust: stable
          make:
            name: Clippy
```

The part of this that we're interested in is the `matrix` object. Notice that
it is a proper key-value object, it has keys `os`, `rust`, and `make`. These
keys are arbitrary, you can choose any names that you want for them. I like to
think of them as the "dimensions" of the matrix. The value at each of these keys
must be a list:

- For the key `os`, the list is `[ubuntu-latest, macos-latest]`.
- For the key `rust`, we have a list with a single element: `[stable]`. 
- And even though the value under `make` looks different, notice that it is still a list,
  it is just a list of more yaml objects.
  
When GitHub Actions reads your job definition, it performs a sort of cross-product
on each entry in your matrix, creating a list of all the combinations of items in
your lists. For this matrix definition, GitHub will generate the following list of
configurations to run the job with:

```yaml
- os: ubuntu-latest
  rust: stable
  make:
    name: Clippy
    task: "check-clippy"
- os: macos-latest
  rust: stable
  make:
    name: Clippy
    task: "check-clippy"
- os: ubuntu-latest
  rust: stable
  make:
    name: Unit tests
    task: "build-all-test run-all-unit-test"
- os: macos-latest
  rust: stable
  make:
    name: Unit tests
    task: "build-all-test run-all-unit-test"
- os: ubuntu-latest
  rust: stable
  make:
    name: Doc tests
    task: "run-all-doc-test"
- os: macos-latest
  rust: stable
  make:
    name: Doc tests
    task: "run-all-doc-test"
```

In the rest of the job definition, you can access the fields of the
active configuration using the `${{ matrix.KEY }}` syntax. You can
see in the job definition above that this is used in the line
`runs-on: ${{ matrix.os }}`, which is how we tell the runner which
type of machine to run the job on.

You may be wondering right now, "Hey, what happened to `include` and
`exclude`? They didn't get mixed into the matrix!", and you would be
right. `include` and `exclude` are special keys that allow you to
manually edit the resulting configurations.

When writing a rule for `include`, we are essentially writing a pattern
that matches against the output configurations and may add new data to
them. Let's look at the effect of a specific `include` rule:

```yaml
      include:
        - os: ubuntu-latest
          sccache-path: /home/runner/.cache/sccache
```

This says: "for any configuration that has `os: ubuntu-latest`, add
another key that has `sccache-path: /home/runner/.cache/sccache`".
If we apply this rule to our output configuration, it would look like
this:

```diff
  - os: ubuntu-latest
    rust: stable
    make:
      name: Unit tests
      task: "build-all-test run-all-unit-test"
+   sccache-path: /home/runner/.cache/sccache
  - os: macos-latest
    rust: stable
    make:
      name: Unit tests
      task: "build-all-test run-all-unit-test"
  - os: ubuntu-latest
    rust: stable
    make:
      name: Doc tests
      task: "run-all-doc-test"
+   sccache-path: /home/runner/.cache/sccache
  - os: macos-latest
    rust: stable
    make:
      name: Doc tests
      task: "run-all-doc-test"
  - os: ubuntu-latest
    rust: stable
    make:
      name: Clippy
      task: "check-clippy"
+   sccache-path: /home/runner/.cache/sccache
  - os: macos-latest
    rust: stable
    make:
      name: Clippy
      task: "check-clippy"
```

Similarly, we can use `exclude` rules to describe objects in the output
configuration to discard. For example, we currently have two configurations
that will cause Clippy to be run, but we really only need Clippy to run
once. Let's look at the effect the following `exclude` rule from our matrix
has on the output configuration:

```yaml
exclude:
  - os: macos-latest
    rust: stable
    make:
      name: Clippy
```

When this rule gets applied, it removes the entry for Clippy on MacOS:

```diff
  - os: ubuntu-latest
    rust: stable
    make:
      name: Unit tests
      task: "build-all-test run-all-unit-test"
  - os: macos-latest
    rust: stable
    make:
      name: Unit tests
      task: "build-all-test run-all-unit-test"
  - os: ubuntu-latest
    rust: stable
    make:
      name: Doc tests
      task: "run-all-doc-test"
  - os: macos-latest
    rust: stable
    make:
      name: Doc tests
      task: "run-all-doc-test"
  - os: ubuntu-latest
    rust: stable
    make:
      name: Clippy
      task: "check-clippy"
- - os: macos-latest
-   rust: stable
-   make:
-     name: Clippy
-     task: "check-clippy"
```

Putting it all together, our matrix definition with all the include
and exclude rules applied will look like the following:

```diff
  - os: ubuntu-latest
    rust: stable
    make:
      name: Unit tests
      task: "build-all-test run-all-unit-test"
+   sccache-path: /home/runner/.cache/sccache
  - os: macos-latest
    rust: stable
    make:
      name: Unit tests
      task: "build-all-test run-all-unit-test"
+   sccache-path: /Users/runner/Library/Caches/Mozilla.sccache
  - os: ubuntu-latest
    rust: stable
    make:
      name: Doc tests
      task: "run-all-doc-test"
+   sccache-path: /home/runner/.cache/sccache
  - os: macos-latest
    rust: stable
    make:
      name: Doc tests
      task: "run-all-doc-test"
+   sccache-path: /Users/runner/Library/Caches/Mozilla.sccache
  - os: ubuntu-latest
    rust: stable
    make:
      name: Clippy
      task: "check-clippy"
+   sccache-path: /home/runner/.cache/sccache
- - os: macos-latest
-   rust: stable
-   make:
-     name: Clippy
-     task: "check-clippy"
```

Ok, awesome. So now we have a strategy for adding new configurations
as well as for tweaking options on specific configurations. If you're trying
to consolidate a bunch of duplicate jobs, a good strategy is to start identifying
the small pieces of each job that are different from the others, and put those
as options in the matrix. Next I'll walk through the rest of the job definition
and talk about how we set it up to meet our Rust project needs.

## Optimizing Rust's build speed with `sccache`

For the rest of this post I'll just be talking about how I set up the rest
of this job definition to build and test our Rust binaries using `sccache`.
`sccache` is a tool built by Mozilla that can cache the output of `rustc` and
re-use those build artifacts if nothing has changed. The basic setup we'll
need in our job is:

- Install `sccache` for the OS we're running on
- Use the [GitHub actions cache] to save the `sccache` cache directory

[GitHub actions cache]: https://github.com/actions/cache

Let me start by just posting the job definition up front, including the matrix
definition we've already seen:

```yaml
# .github/workflows/ci.yml
name: CI

on:
  pull_request:
  workflow_dispatch:
  
jobs:
  tests:
    name: ${{ matrix.make.name }} (${{ matrix.os }})
    runs-on: ${{ matrix.os }}
    strategy:
      fail-fast: false
      matrix:
        os: [ubuntu-latest, macos-latest]
        rust: [stable]
        make:
          - name: Clippy
            task: "check-clippy"
          - name: Unit tests
            task: "build-all-test run-all-unit-test"
          - name: Doc tests
            task: "run-all-doc-test"
        include:
          - os: ubuntu-latest
            sccache-path: /home/runner/.cache/sccache
          - os: macos-latest
            sccache-path: /Users/runner/Library/Caches/Mozilla.sccache
        exclude:
          - os: macos-latest
            rust: stable
            make:
              name: Clippy
    env:
      RUST_BACKTRACE: full
      RUSTC_WRAPPER: sccache
      RUSTV: ${{ matrix.rust }}
      SCCACHE_CACHE_SIZE: 2G
      SCCACHE_DIR: ${{ matrix.sccache-path }}
      # SCCACHE_RECACHE: 1 # Uncomment this to clear cache, then comment it back out
    steps:
      - uses: actions/checkout@v2
      - name: Install sccache (ubuntu-latest)
        if: matrix.os == 'ubuntu-latest'
        env:
          LINK: https://github.com/mozilla/sccache/releases/download
          SCCACHE_VERSION: 0.2.13
        run: |
          SCCACHE_FILE=sccache-$SCCACHE_VERSION-x86_64-unknown-linux-musl
          mkdir -p $HOME/.local/bin
          curl -L "$LINK/$SCCACHE_VERSION/$SCCACHE_FILE.tar.gz" | tar xz
          mv -f $SCCACHE_FILE/sccache $HOME/.local/bin/sccache
          echo "$HOME/.local/bin" >> $GITHUB_PATH
      - name: Install sccache (macos-latest)
        if: matrix.os == 'macos-latest'
        run: |
          brew update
          brew install sccache
      - name: Install Rust ${{ matrix.rust }}
        uses: actions-rs/toolchain@v1
        with:
          toolchain: ${{ matrix.rust }}
          profile: minimal
          override: true
      - name: Cache cargo registry
        uses: actions/cache@v2
        continue-on-error: false
        with:
          path: |
            ~/.cargo/registry
            ~/.cargo/git
          key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
          restore-keys: |
            ${{ runner.os }}-cargo-
      - name: Save sccache
        uses: actions/cache@v2
        continue-on-error: false
        with:
          path: ${{ matrix.sccache-path }}
          key: ${{ runner.os }}-sccache-${{ hashFiles('**/Cargo.lock') }}
          restore-keys: |
            ${{ runner.os }}-sccache-
      - name: Start sccache server
        run: sccache --start-server
      - name: ${{ matrix.make.name }}
        run: make ${{ matrix.make.task }}
      - name: Print sccache stats
        run: sccache --show-stats
      - name: Stop sccache server
        run: sccache --stop-server || true
```

This job definition is 95% setup and about 5% task-running. In fact, the entirety
of our project-specific building and testing specifications are defined inside our
`Makefile`, so the step where we call `make ${{ matrix.make.task }}` is the only
step that is unique to our project, the rest could be re-used as boilerplate for
other Rust projects ;)

Let's start with the `env` section. First, we set `RUST_BACKTRACE: full` because
if any of our tests panic we want to know why. `RUSTV` is an environment variable
that we use inside our `Makefile` to tell cargo commands which release channel to use.

The rest of the env has to do with [`sccache`]. As I mentioned before, we want to
use [`sccache`] to reduce the number of times we need to re-build crates when possible.
Here is a quick summary of the other variables:

- `RUSTC_WRAPPER: sccache` is read by `cargo` itself and tells it to run compiler
  commands using `sccache rustc ...` rather than just `rustc ...`. This is the easiest
  way to use `sccache` with Rust.
  
- `SCCACHE_CACHE_SIZE` tells sccache the maximum size you want to allow your cache
  to grow to. This is one of the most important things to get right: if you set this
  too small and the cache runs out of space, `sccache` will start evicting artifacts
  from the cache and you will end up recompiling more than you need to.
  
- `SCCACHE_DIR` tells sccache where to store build artifacts. This path is different
  on different OS's, which is why we assign it using a matrix parameter.
  
Now let's run through the step definitions:

```yaml
      - uses: actions/checkout@v2
```

This is a pretty standard GH Actions step that just checks out your git repository
in the current directory. This gives further steps the ability to build or interact
with your code.

Next up is installing `sccache`:

```yaml
      - name: Install sccache (ubuntu-latest)
        if: matrix.os == 'ubuntu-latest'
        env:
          LINK: https://github.com/mozilla/sccache/releases/download
          SCCACHE_VERSION: 0.2.13
        run: |
          SCCACHE_FILE=sccache-$SCCACHE_VERSION-x86_64-unknown-linux-musl
          mkdir -p $HOME/.local/bin
          curl -L "$LINK/$SCCACHE_VERSION/$SCCACHE_FILE.tar.gz" | tar xz
          mv -f $SCCACHE_FILE/sccache $HOME/.local/bin/sccache
          echo "$HOME/.local/bin" >> $GITHUB_PATH
      - name: Install sccache (macos-latest)
        if: matrix.os == 'macos-latest'
        run: |
          brew update
          brew install sccache
```

This is a good example of adapting steps according to a matrix. Depending on whether
we are running on `ubuntu-latest` or `macos-latest`, there is a different procedure for
installing `sccache`. Notice that only one of these two steps will ever run in a given
execution of a job: the `if:` conditional checks which `matrix.os` value is specified
in this job instance.

This is also a great example of a good opportunity for creating
a reusable GitHub Action definition. If there's an action definition out there for
more easily installing `sccache` (or if one of you readers decides to put one together),
let me know!

The next step just installs the Rust toolchain. If you have used Rust on GitHub actions
before you have probably seen this one already:

```yaml
      - name: Install Rust ${{ matrix.rust }}
        uses: actions-rs/toolchain@v1
        with:
          toolchain: ${{ matrix.rust }}
          profile: minimal
          override: true
```

Even though in our matrix we only have one value, `rust: [stable]`, it is nice to use
`toolchain: ${{ matrix.rust }}` so that in the future if you (or a coworker of yours)
decides to run your workflow against other release channels, the only edit that will
need to be made is in the matrix.

This next one has to do with `sccache` again, and it has to do with saving the cache
directory itself. When using the free GitHub Action runners, jobs are always run on a
fresh instance of a container or virtual machine somewhere. This means that our
sccache would have to start from scratch on every job and we wouldn't get any benefit
out of it. To fix this, we can use the `actions/cache@v2` action to preserve certain
directories between job runs.

```yaml
      - name: Cache cargo registry
        uses: actions/cache@v2
        continue-on-error: false
        with:
          path: |
            ~/.cargo/registry
            ~/.cargo/git
          key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
          restore-keys: |
            ${{ runner.os }}-cargo-
      - name: Save sccache
        uses: actions/cache@v2
        continue-on-error: false
        with:
          path: ${{ matrix.sccache-path }}
          key: ${{ runner.os }}-sccache-${{ hashFiles('**/Cargo.lock') }}
          restore-keys: |
            ${{ runner.os }}-sccache-
```

The `actions/cache@v2` action lets us specify directories that the GitHub runner will save at the
end of each job, and attempt to restore at the beginning of the next job. You can have
multiple different types of things you want to cache, so you have to choose a `key` for
each cache to be labeled with when it gets saved. You also need to give it `restore-keys`
that tell it how to choose an existing saved cache that you want to restore from. You can
think of `restore-keys` as a pattern for matching some prefix of an actual key: the more
specifically a key matches your restore-key, the higher precedence it will have.

{{<idea>}}

On a side note about GitHub caches, does anybody know whether it makes more sense to
keep separate caches separate or to combine them? Here I have separate caches for the
Cargo registry and for the Sccache directory, but does it make any practical difference?
If you know one way or another, [let me know]!

[let me know]: https://twitter.com/RazzleRustacean

{{</idea>}}

Finally, we have the last steps of our jobs. Here we start the sccache server, run our
Make task, then print statistics and stop the sccache server.

```yaml
      - name: Start sccache server
        run: sccache --start-server
      - name: ${{ matrix.make.name }}
        run: make ${{ matrix.make.task }}
      - name: Print sccache stats
        run: sccache --show-stats
      - name: Stop sccache server
        run: sccache --stop-server || true
```

There are a couple of things you will want to check on when you first
set this up to make sure you are actually getting benefit from the cache:

- Make sure your cargo commands are actually running sccache
- Make sure that sccache is actually hitting the cache rather than always rebuilding

Here's an example of output when the `sccache` server has just started up and has
not processed any compile requests:

```bash
$ sccache --show-stats
Compile requests                      8
Compile requests executed             0
Cache hits                            0
Cache misses                          0
Cache timeouts                        0
Cache read errors                     0
Forced recaches                       0
Cache write errors                    0
Compilation failures                  0
Cache errors                          0
Non-cacheable compilations            0
Non-cacheable calls                   8
Non-compilation calls                 0
Unsupported compiler calls            0
Average cache write               0.000 s
Average cache read miss           0.000 s
Average cache read hit            0.000 s
Failed distributed compilations       0

Non-cacheable reasons:
incremental                           7
crate-type                            1

Cache location                  Local disk: "/Users/runner/Library/Caches/Mozilla.sccache"
Cache size                          545 MiB
Max cache size                        2 GiB
```

If you build your project and see this immediately afterwards, something went wrong. I
would recommend running your build with `--verbose` and double-checking that the commands
cargo is running all begin with `sccache rustc` rather than just `rustc`

If your verbose cargo output looks like this, your sccache setup is working:

```bash
   Compiling fluvio-spu v0.5.0 (/Users/nick/infinyon/fluvio/src/spu)
     Running `/Users/nick/.cargo/bin/sccache rustc --crate-name fluvio_spu --edition=2018 ...
```

But if you see output like this, sccache is not being invoked at all, and you should probably
double-check that your `RUSTC_WRAPPER` environment variable is set properly.

```bash
   Compiling fluvio-spu v0.5.0 (/Users/nick/infinyon/fluvio/src/spu)
     Running `rustc --crate-name fluvio_spu --edition=2018
```

### Verifying the `sccache` results

When you have set things up so that sccache is properly running, you will see stats
that have actual numbers in them rather than zeros. The next step is to make sure that
those numbers are telling you that you hit the cache rather than rebuilding (missing the cache).

You might miss the cache for a couple of reasons:

- This is the first time you have built using sccache and so you are populating the cache for the first time
  - In this case, just run your job again to test if you hit the cache on the second go-round
- Something went wrong when re-loading the cache directory (e.g. from the GitHub cache).
- You have added a ton of dependencies to your project that you haven't built before, so they're not in the cache

Here is an example of what your stats might look like if you have missed the cache.
Notice that the number of hits is actually not zero, it's just a very low number.
I think this is probably because of caching duplicate dependencies within your dependency tree.
However, if you see these results after a build, then your build did not really benefit from
`sccache`.

```bash
Compile requests                    484
Compile requests executed           343
Cache hits                           13
Cache hits (Rust)                    13
Cache misses                        330
Cache misses (Rust)                 330
Cache timeouts                        0
Cache read errors                     0
Forced recaches                       0
Cache write errors                    0
Compilation failures                  0
Cache errors                        313
Cache errors (Rust)                 313
Non-cacheable compilations            0
Non-cacheable calls                 141
Non-compilation calls                 0
Unsupported compiler calls            0
Average cache write               0.000 s
Average cache read miss           2.249 s
Average cache read hit            0.001 s
Failed distributed compilations       0

Non-cacheable reasons:
crate-type                          111
incremental                          28
-                                     2

Cache location                  Local disk: "/Users/runner/Library/Caches/Mozilla.sccache"
Cache size                            4 GiB
Max cache size                       10 GiB
```

Usually if you get these results, all you need to do is run the build again, and
the second build will be able to leverage the pre-compiled artifacts from the previous run.
The stats from the second build might look more like the following:

```bash
Compile requests                    481
Compile requests executed           330
Cache hits                          318
Cache hits (Rust)                   318
Cache misses                         12
Cache misses (Rust)                  12
Cache timeouts                        0
Cache read errors                     0
Forced recaches                       0
Cache write errors                    0
Compilation failures                  0
Cache errors                          0
Non-cacheable compilations            0
Non-cacheable calls                 151
Non-compilation calls                 0
Unsupported compiler calls            0
Average cache write               0.001 s
Average cache read miss           1.242 s
Average cache read hit            0.001 s
Failed distributed compilations       0

Non-cacheable reasons:
crate-type                          108
incremental                          41
-                                     2

Cache location                  Local disk: "/Users/runner/Library/Caches/Mozilla.sccache"
Cache size                          545 MiB
Max cache size                        2 GiB
```

Notice that we have many more cache hits than cache misses, that's the indicator that
we are getting good value out of `sccache`.

## The end result

When you push this workflow and trigger it, you'll see a job start running for each
combination of matrix parameters you provided:

<img 
  style="width: 600px"
  src="/blog/images/rust-workflows/workflow-jobs.png" 
  alt="Workflow jobs as shown by the GitHub Actions dashboard" />

I took this screenshot from our full workflow file which includes a job for Rustfmt. The
rest of the jobs in this list were generated from the single job definition I showed above.

# Conclusion

Thanks for reading, I hope this can be helpful to others who are also setting up
GitHub actions with Rust projects. If you want to learn more about the Fluvio project,
feel free to [check out our GitHub] or come [join our community Discord].

[`sccache`]: https://github.com/mozilla/sccache
[check out our GitHub]: https://github.com/infinyon/fluvio/
[join our community Discord]: https://discordapp.com/invite/bBG2dTz

#### Quick links:

- [Getting started with Fluvio](/docs/getting-started/)
- [Fluvio CLI reference](/docs/cli-reference/)
- [Fluvio Architecture](/docs/architecture/)
