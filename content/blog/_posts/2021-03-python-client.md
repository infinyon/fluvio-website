---
title: How we built our Python Client that's mostly Rust
author:
    name: "Sebastian Imlay"
    github: "simlay"
description: "Learn how to wrap your rust crate in Python"
date: 2021-03-28
slug: key-value-records
url: /blog/2021/03/python-client
img: blog/images/key-value/social/florian-berger-keys.jpg
img-credit:
    link: https://unsplash.com/photos/SzG0ncGBOeo
    author: Florian Berger
    site: Upsplash
twitter-card: summary_large_image

---

This week, we're happy to announce the addition of a [Python client library for
for Fluvio](https://github.com/infinyon/fluvio-client-python).
Using Python client s just as easy as using our other client. Checkout the [hello world in Python
tutorial](/tutorials/python/hello-world/) or
[documentation](https://infinyon.github.io/fluvio-client-python/fluvio.html)
for usage.

In this post, we'll talk about how we were able to leverage some of the great
rust tooling to build a python client without writing much Python itself.

# Overview

In short, we will:
* use [flapigen](https://github.com/Dushistov/flapigen-rs) to
define how our rust structs will go across the FFI.
* Use the [rust-cpython](https://github.com/dgrunwald/rust-cpython) extension
in our python project and call it.

## Setup

Starting from scratch, to get started you need to do:
rust project via:
```bash
cargo new --lib my-python-lib
cd my-python-lib
python -m venv venv
source venv/bin/activate
```

The above creates a rust project named `my-python-lib`, then creates a python
[virtual environment](https://docs.python.org/3/tutorial/venv.html).

Note: you'll need to have the [rust toolchain](https://rustup.rs/) and [python 3.6](https://www.python.org/downloads/) or above.


## Rust glue

You'll need to add this to your `Cargo.toml`:

```toml
[lib]
crate-type = ["cdylib"]

[dependencies]
cpython = { version = "0.5", features = ["extension-module"] }

[build-dependencies]
flapigen = "0.6.0-pre7"
```

Now inside a `build.rs` you need:
```rust
use flapigen::{LanguageConfig, PythonConfig};
use std::{env, path::Path};

fn main() {
    let in_src = Path::new("src").join("glue.rs.in");
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_src = Path::new(&out_dir).join("glue.rs");

    let python_cfg = PythonConfig::new("my_python_lib".to_owned());
    let flap_gen =
        flapigen::Generator::new(LanguageConfig::PythonConfig(python_cfg)).rustfmt_bindings(true);
    flap_gen.expand("python bindings", &in_src, &out_src);
    println!("cargo:rerun-if-changed={}", in_src.display());
}
```

This takes in `src/glue.rs.in` and write it to a file in `${OUT_DIR}/glue.rs`.

Now add a `src/glue.rs.in` file with something like the following:
```rust
pub struct Foo {
    val: i32
}
impl Foo {
    pub fn new(val: i32) -> Self {
        Self {
            val
        }
    }
    pub fn set_field(&mut self, new_val: i32) {
        self.val = new_val;
    }
    pub fn val(&self) -> i32 {
        self.val
    }
}
foreign_class!(class Foo {
    self_type Foo;
    constructor Foo::new(_: i32) -> Foo;
    fn Foo::set_field(&mut self, _: i32);
    fn Foo::val(&self) -> i32;
});
```

This is a simple example the [flapigen
book](https://dushistov.github.io/flapigen-rs/foreign-class.html) for you to
nicely copy and paste.

Your `src/lib.rs` should currently have some basic tests. You should change it to the following:
```rust
#![allow(non_snake_case, unused)]

include!(concat!(env!("OUT_DIR"), "/glue.rs"));
```

This is a very common rust pattern with build generation. What this does is
take the file in `${OUT_DIR}/glue.rs` and include the contents `src/lib.rs`.

This section uses flapigen to expand the
[`foreign_class`](https://dushistov.github.io/flapigen-rs/foreign-class.html)
macro into a bunch of [cpython](https://github.com/dgrunwald/rust-cpython)
functions as an [extension
module](https://docs.python.org/3/extending/extending.html) and cargo compiles
it as a
[`cdylib`](https://doc.rust-lang.org/cargo/reference/cargo-targets.html#library).
If you wanna see what that actually looks like install
[`cargo-expand`](https://crates.io/crates/cargo-expand) and run `cargo expand`.
You'll get a lot of generated rust code.

## Python Glue

In the [`setup`](#setup), we created a virtual environment, now we'll need to
install some python tools.

Create a file called `setup.py` with:
```python
from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
    name="my-python-lib",
    version="1.0",
    rust_extensions=[RustExtension("my_python_lib", binding=Binding.RustCPython)],
    # rust extensions are not zip safe, just like C-extensions.
    zip_safe=False,
)
```

This is the most basic
[setuptools-rust](https://github.com/PyO3/setuptools-rust#setuppy) setup with
the exception of using
[`RustCPython`](https://setuptools-rust.readthedocs.io/en/latest/reference.html#setuptools_rust.Binding)
as the binding.

You'll need to install `setuptools-rust` in your virtual env via:
```sh
source venv/bin/activate && pip install setuptools-rust
```

Now to build the rust and python you just do `python setup.py develop`. This will call `cargo` and move it into your local directory.

Now in a `simple.py` script to use said library:

```python
from my_python_lib import Foo
foo = Foo(1)
print(foo.val())
foo.set_field(11)
print(foo.val())
```

Running said script via `python simple.py` should result in:
```
$ python simple.py
1
11
```

And there you go, you've called Rust from python!

## Conclusion

You can get the source for this post in our [fluvio-demo-apps-rust.](https://github.com/infinyon/fluvio-demo-apps-rust/).

This is just the basics for setting up a python wrapper.  For our Fluvio Python
Client, our rust crate is
[`_fluvio_python`](https://github.com/infinyon/fluvio-client-python/blob/c6d82d63a001376325d8583c75319c41fd5bfcd5/build.rs#L10)
so that it's not exported and wrapped the rust structs with python classes so
we can have nice [documentation
generation](https://infinyon.github.io/fluvio-client-python/fluvio.html).

Packaging this up as python package, testing and putting it on pypi is another
key part for productionalizing such a project. You can see how we did it in our
[`Makefile`](https://github.com/infinyon/fluvio-client-python/blob/main/Makefile)
and [github publishing
workflow](https://github.com/infinyon/fluvio-client-python/blob/main/.github/workflows/publish.yml).
