---
title: How to Build a Custom Test Harness in Rust
author:
    name: "T.J. Telan"
    github: "tjtelan"
description: "Building a custom test harness in Rust is less complicated following these patterns"
date: 2021-04-27
slug: rust-custom-test-harness 
url: /blog/2021/04/rust-custom-test-harness
img: blog/images/rust-custom-harness/banner-title.png
tile: blog/images/rust-custom-harness/banner-notitle.png
twitter-card: summary_large_image
code:
    height: 9000
---

I ran into a problem effectively using `cargo test` in [Fluvio](https://github.com/infinyon/fluvio) for integration testing.

## Let’s talk about integration testing in Rust

While creating integration testing for Fluvio, I ran into a problem. Organizing and executing integration tests with `cargo test` was becoming inefficient. We needed to standardize the setup of a test environment. 

As a lone developer, you can apply one-off customizations when running tests locally. But if you try to extend that strategy to continuous integration, you’ll quickly find that making changes manually becomes burdensome. CI encourages testing many different configurations, which means a successful CI plan requires easy management of test harness variables (a.k.a. Not manually updating variables for every test you need to run).

`cargo test` is just not equipped to handle this specialized focus on environment setup, or the cleanup/teardown needed after a test is run. When using `cargo test`, these crucial tasks could only occur outside of the harness or within the logic of a test. Neither of these are good choices. Outside of the harness is not ideal because these processes end up too disconnected and hard to maintain. Likewise, including setup/teardown within the logic of a test is inappropriate because it creates mental overhead for a test writer, and may obscure the results of tests.

I needed to find a way around the limited functionality of `cargo test` -- keep reading to find out how I did it by creating a standardized setup and teardown as part of our testing harness.

### How does `cargo test` work by default?

There is a distinction between unit tests and integration tests in [the Rust book]. The distinction is less about testing strategy and more about defining Rust’s conventions for test organization.

[the Rust book]: https://doc.rust-lang.org/book/ch11-03-test-organization.html

The main points are that:
  * Your tests are annotated with `#[test]`
  * [libtest] harness enumerates through all of your tests (a point we’ll revisit later in more detail)
  * libtest returns the pass/fail status of the execution

[libtest]: https://github.com/rust-lang/libtest

### What do I need from integration testing?

Libtest doesn't specifically offer anything to support integration testing patterns.

Setup of a standard test environment – especially in a complex system – is essential for managing expected behavior when making code changes.

Unfortunately libtest does not assist with setup or teardown. I needed the ability to abstract away the setup and teardown of my test environment from test code. 

This task will be performed either way. Without harness support, setup/teardown will be performed via external shell scripts or padding the setup/teardown process within every single integration test... (no one's idea of fun).

It isn’t convenient to manage setup and teardown in a different context than the integration test. This kind of testing overhead leads to hard-to-reproduce and time consuming mistakes.

### Where do we get started with a custom test harness?

By default, libtest will compile each of your `#[test]` labeled functions into their own binary crates (with its own `main()`) and executes it as part of the test. But we’re going to build all our integration tests into a single crate. This is recommended in order to speed up compile time ([[1]], [[2]])

[1]: https://endler.dev/2020/rust-compile-times/#combine-all-integration-tests-in-a-single-binary
[2]: https://matklad.github.io/2021/02/27/delete-cargo-integration-tests.html

First we’re going to create an integration test directory at the root of the crate where we’re going to build our integration test focused binary.

```shell
$ mkdir integration
$ touch integration/main.rs

# Then create a main() function in main.rs
```

In your Cargo.toml, you want to add 

```toml
# Cargo.toml

# We'll revisit the `inventory` crate later in the post
[dev-dependencies]
inventory = "0.1"

[[test]]
name = "integration"
path = "integration/main.rs"
harness = false
```

This tells cargo test to not use libtest when running the `integration` test.

When we run `cargo test integration`, what cargo will compile `integration/main.rs` and execute it in the same manner as `cargo run`. This is all a harness is from `cargo`’s perspective. 

### Add Setup and teardown steps

Next we’ll lay the foundation for our testing pattern. We’ll create 2 functions, `setup()` and `teardown()`, and add them to our `main()` (with reserved space in between for our future tests to be called).

```rust
// main.rs

fn setup() {
   println!("Setup")
}

fn teardown() {
   println!("Teardown")
}
fn main() {
   // Setup test environment
   setup();

   // TODO: Run the test

   // Teardown test environment
   teardown();
}
```

### Collect all integration tests

To do its job, our test runner needs to create a list of all the test functions. Initially, I thought there would be an easy way to do this by leveraging [libtest]'s `#[test]` attribute. 

I dug around in [relevant areas of libtest] and [Cargo test] and [Rustc macros] code, but long (sad) story short, there is no straightforward way to reuse libtest for the purpose of test collection.

[relevant areas of libtest]: https://github.com/rust-lang/libtest/blob/master/libtest/lib.rs
[Cargo test]: https://github.com/rust-lang/cargo/blob/master/src/cargo/ops/cargo_test.rs
[Rustc macros]: https://github.com/rust-lang/rust/blob/master/compiler/rustc_builtin_macros/src/test.rs

If that surprises you, then you're like me. I had hoped to use the test collection functionality from `#[test]`, but it wasn’t clear how I could accomplish this. My mental model for how `cargo test` works needed a refresh.

Now that we’ve removed the option of using libtest, so that gives you 2 practical options for collecting tests:

1. Manually modify `integration/main.rs` and add your test in between the setup and teardown
    * A quick and straightforward solution if you have a small set of tests
    * This option requires us to add new tests to this list, which can be error-prone and tedious as we grow.
2. Build a test collector. We generate an external test catalog, and modify `integration/main.rs` to execute tests from the catalog.
    * This is a long term solution, which we’ll be covering for the rest of the post.

### Building the test collector

For this test collector, we'll be utilizing a crate. The [inventory] crate is a plugin registry system. We'll be using it for all the heavy lifting in our test framework, which means we'll be treating our tests as plugins.

[inventory]: https://crates.io/crates/inventory

In our `main.rs`, let’s declare a new module `tests`, where we can organize all the integration tests.


```diff
// main.rs

+ pub mod tests;

fn setup() {
   println!("Setup")
}

fn teardown() {
   println!("Teardown")
}
fn main() {
   // Setup test environment
   setup();

   // TODO: Run the test

   // Teardown test environment
   teardown();
}
```

In our new module, we’ll start by creating a struct to represent a single test for the plugin registry.

```rust
// tests/mod.rs

#[derive(Debug)]
pub struct IntegrationTest {
   pub name: &'static str,
   pub test_fn: fn(),
}

inventory::collect!(IntegrationTest);
```

In this example, our struct `IntegrationTest` has 2 fields. 

*   `name` is a human-readable name, which can be used as a key for test selection.
*   `test_fn` is a pointer to a function whose signature is non-async, takes no args, and does not return anything.

Note:  You can use functions that take args, and return things.

For example:

```rust
pub test_fn: fn(String) -> Result<(), ()>,
```

Then we call the `inventory::collect!()`macro to instantiate a plugin registry. When we write our tests, we’ll add to the plugin registry. More on this next.


### Adding new tests to plugin registry

We’re going to add a new basic test to the plugin registry. Start by adding a new submodule called `basic` in the tests module. 

```rust
// tests/mod.rs

pub mod basic;
```

In the `basic` module, we write our basic test `basic_test()`


```rust
// tests/basic.rs

use super::IntegrationTest;

fn basic_test() {
   println!("Running basic test")
}

inventory::submit!(IntegrationTest {
   name: "basic",
   test_fn: basic_test
});
```

We'll use `inventory::submit!()` to register our new test with the `IntegrationTest` struct we defined earlier.

`name` is a friendly, human-readable name. We can use this name as a key to search through the plugin registry.

`test_fn` takes the name of our test function. It has the same function signature as we defined. 


### Running tests from registry

We’ll finish this example up by running all of our registered tests

```diff
// main.rs

pub mod tests;
+ use tests::IntegrationTest;

fn setup() {
   println!("Setup")
}

fn teardown() {
   println!("Teardown")
}
fn main() {
   setup();

-   // TODO: Run the test
+   // Run the tests
+   for t in inventory::iter::<IntegrationTest> {
+       (t.test_fn)()
+   }

   teardown();
}
```

```shell
$ cargo test integration
   Compiling blog-post-example v0.1.0 (/home/telant/Documents/blog-post-example)
	Finished test [unoptimized + debuginfo] target(s) in 0.21s
 	Running target/debug/deps/blog_post_example-e042d787684bb333

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

 	Running target/debug/deps/integration-7ed2452642c6f3b6

Setup
Running basic test
Teardown
```

### Tips for extending the example

The example runs all of the registered tests. But here are some useful impls if you want to extend even further. For example, adding a CLI, if you want to select individual tests. Or provide options to customize setup or teardown behavior.

```rust
impl IntegrationTest {
   pub fn all_test_names() -> Vec<&'static str> {
       inventory::iter::<IntegrationTest>
           .into_iter()
           .map(|x| x.name)
           .collect::<Vec<&str>>()
   }

   pub fn from_name<S: AsRef<str>>(test_name: S) -> Option<&'static IntegrationTest> {
       inventory::iter::<IntegrationTest>
           .into_iter()
           .find(|t| t.name == test_name.as_ref())
   }
}
```

{{<idea>}}

If you want to see more of these ideas extended even further, check out [Fluvio’s integration test runner](https://github.com/infinyon/fluvio/tree/master/tests/runner/src).

We use the CLI to customize setup, handle async testing, and we use an [attribute macro] to collect tests.

[attribute macro]: https://github.com/infinyon/fluvio/blob/master/tests/runner/src/fluvio-integration-derive/src/lib.rs 
{{</idea>}}

### Conclusion

Rust’s testing ecosystem in the 2018 edition is great for unit testing. But for integration testing it still has room for improvement. Custom harnesses will become more necessary as Rust gains new developers. 

If we want to avoid reinventing the wheel, we need stable support from [libtest](https://docs.rs/libtest/0.0.1/libtest/) or more examples of how to perform test collection and patterns for setup, test, and teardown workflows.

If you made it this far, thank you for following along with me! I wrote this because I could not find a guide to do this before trying to do this myself, and knowing these things beforehand would have made it much faster. Hopefully others find my experience helpful.