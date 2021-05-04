---
title: Wrapping your rust in a Java Jar for distribution
author:
    name: "Sebastian Imlay"
    github: "simlay"
description: "Learn how to wrap your Rust crate in a Java Jar for the desktop/server"
date: 2021-05-03
slug: java-client
url: /blog/2021/05/java-client
img: blog/images/java-client/crab-in-jar.png
img-credit:
    link: https://www.flickr.com/photos/jenny-pics/4821968234/
    author: Jenny Downing
    site: Flikr
twitter-card: summary_large_image
code:
    height: 1200
---

This week, we're happy to announce the addition of a [Java client library for Fluvio].
Using the Java client is just as easy as using our other clients. Check out the
[hello world in Java tutorial] or [documentation] for usage.

This post will talk about how we bundled and distributed our rust code into a [Java Jar] using [Gradle] to target a desktop enviroment.  To do
this for android, we recommend the [Flapigen android example].

[Java client library for Fluvio]: https://github.com/infinyon/fluvio-client-java
[hello world in Java tutorial]: /tutorials/java/hello-world/
[documentation]: https://infinyon.github.io/fluvio-client-java/com/infinyon/fluvio/package-summary.html
[Java Jar]: https://docs.oracle.com/javase/tutorial/deployment/jar/basicsindex.html
[Gradle]: https://gradle.org/
[Flapigen android example]: https://dushistov.github.io/flapigen-rs/java-android-example.html

# Overview

Similar to [our python post], we used [flapigen] for packaging, but then it took a bit of research and many cups of coffee to figure out how to bundle rust into a jar for publishing. Let's get started.

[flapigen]: https://github.com/Dushistov/flapigen-rs
[our python post]: /blog/2021/03/python-client/

## Setup

Before doing anything, make sure you've got the [rust toolchain] and [Gradle installed].

To get started, we'll create a new project folder set up for both
Rust and Gradle.

```bash
cargo new --lib my-java-lib
cd my-java-lib
gradle init
```

Gradle will guide you through the installation. Follow the [Building Java Libraries Sample]. It should look similar to this:

```bash
$ gradle init

Select type of project to generate:
  1: basic
  2: application
  3: library
  4: Gradle plugin
Enter selection (default: basic) [1..4] 3

Select implementation language:
  1: C++
  2: Groovy
  3: Java
  4: Kotlin
  5: Scala
  6: Swift
Enter selection (default: Java) [1..6] 3

Select build script DSL:
  1: Groovy
  2: Kotlin
Enter selection (default: Groovy) [1..2] 1

Select test framework:
  1: JUnit 4
  2: TestNG
  3: Spock
  4: JUnit Jupiter
Enter selection (default: JUnit 4) [1..4] 1

Project name (default: my-java-lib):
Source package (default: my.java.lib):

> Task :init
Get more help with your project: https://docs.gradle.org/6.8.3/samples/sample_building_java_libraries.html

BUILD SUCCESSFUL in 37s
2 actionable tasks: 2 executed
```

The above creates a Rust crate named `my-java-lib` and a gradle library called
`my-java-lib`.


Running `tree`, you will see a bunch of directories:

```bash
$ tree
   .
   |-gradle
   |---wrapper
   |-lib
   |---src
   |-----main
   |-------java
   |---------my
   |-----------java
   |-------------lib
   |-------resources
   |-----test
   |-------java
   |---------my
   |-----------java
   |-------------lib
   |-------resources
   |-src
```

[rust toolchain]: https://rustup.rs/
[Gradle installed]: https://docs.gradle.org/current/userguide/installation.html
[Building Java Libraries Sample]: https://docs.gradle.org/current/samples/sample_building_java_libraries.html

## Rust glue

We'll need to add this to your `Cargo.toml`:

```bash
[lib]
crate-type = ["cdylib"]
name = "my_java_lib"

[dependencies]
log = "^0.4.6"

[build-dependencies]
flapigen = "0.6.0-pre7"
bindgen = { version = "0.57.0", default-features = false}
```


### `build.rs`
Now we'll create a [build script] in `build.rs` by adding the following:

[build script]: https://doc.rust-lang.org/cargo/reference/build-scripts.html

```rust
use flapigen::{LanguageConfig, JavaConfig};
use std::{
    env,
    path::{Path, PathBuf},
};

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let jni_c_headers_rs = Path::new(&out_dir).join("jni_c_header.rs");
    gen_jni_bindings(&jni_c_headers_rs);

    let java_cfg = JavaConfig::new(
        Path::new("lib")
        .join("src")
        .join("main")
        .join("java")
        .join("my")
        .join("java")
        .join("lib"),
        "my.java.lib".into(),
    );

    let in_src = Path::new("src").join("java_glue.rs.in");
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_src = Path::new(&out_dir).join("java_glue.rs");
    let flap_gen =
        flapigen::Generator::new(
            LanguageConfig::JavaConfig(java_cfg)
        ).rustfmt_bindings(true);

    flap_gen.expand("java bindings", &in_src, &out_src);
    println!("cargo:rerun-if-changed={}", in_src.display());
}

fn gen_jni_bindings(jni_c_headers_rs: &Path) {
    let java_home = env::var("JAVA_HOME").expect("JAVA_HOME env variable not settted");
    let java_include_dir = Path::new(&java_home).join("include");
    let target = env::var("TARGET").expect("target env var not setted");

    let java_sys_include_dir = java_include_dir.join(if target.contains("windows") {
        "win32"
    } else if target.contains("darwin") {
        "darwin"
    } else {
        "linux"
    });

    let include_dirs = [java_include_dir, java_sys_include_dir];
    println!("jni include dirs {:?}", include_dirs);

    let jni_h_path = search_file_in_directory(&include_dirs[..], "jni.h").expect("Can not find jni.h");
    println!("cargo:rerun-if-changed={}", jni_h_path.display());

    gen_binding(&include_dirs[..], &jni_h_path, jni_c_headers_rs).expect("gen_binding failed");
}

fn search_file_in_directory<P: AsRef<Path>>(dirs: &[P], file: &str) -> Result<PathBuf, ()> {
    for dir in dirs {
        let dir = dir.as_ref().to_path_buf();
        let file_path = dir.join(file);
        if file_path.exists() && file_path.is_file() {
            return Ok(file_path);
        }
    }
    Err(())
}

fn gen_binding<P: AsRef<Path>>(
    include_dirs: &[P],
    c_file_path: &Path,
    output_rust: &Path,
) -> Result<(), String> {
    let mut bindings: bindgen::Builder = bindgen::builder().header(c_file_path.to_str().unwrap());

    bindings = include_dirs.iter().fold(bindings, |acc, x| {
        acc.clang_arg("-I".to_string() + x.as_ref().to_str().unwrap())
    });

    let generated_bindings = bindings
        .generate()
        .map_err(|_| "Failed to generate bindings".to_string())?;

    generated_bindings
        .write_to_file(output_rust)
        .map_err(|err| err.to_string())?;

    Ok(())
}
```

This buildscript is long because:
1) it looks at the `JAVA_HOME` environment variable
2) looks for the [JNI] headers
3) uses [rust-bindgen] to generate bindings to the java runtime
4) Generates flaipgen FFI functions
5) Generates java classes for the flapigen classes and puts them in
`lib/src/main/java/my/java/lib/`

[JNI]: https://en.wikipedia.org/wiki/Java_Native_Interface
[rust-bindgen]: https://rust-lang.github.io/rust-bindgen/

### `src/*.rs`
Now we'll add a `src/java_glue.rs.in` file with something like the following:

```rust
// src/java_glue.rs.in
use crate::jni_c_header::*;

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


This simple example was published in the [flapigen book], and we can copy and paste it here.

[flapigen book]: https://dushistov.github.io/flapigen-rs/foreign-class.html

The `src/lib.rs` should currently have some basic tests. We'll change it to the following:

```rust
// src/lib.rs
#![allow(
    clippy::enum_variant_names,
    clippy::unused_unit,
    clippy::let_and_return,
    clippy::not_unsafe_ptr_arg_deref,
    clippy::cast_lossless,
    clippy::blacklisted_name,
    clippy::too_many_arguments,
    clippy::trivially_copy_pass_by_ref,
    clippy::let_unit_value,
    clippy::clone_on_copy
)]
mod jni_c_header;

include!(concat!(env!("OUT_DIR"), "/java_glue.rs"));
```

This is a typical Rust pattern when using build scripts. The code takes the
file in `${OUT_DIR}/java_glue.rs` and includes the contents into `src/lib.rs` in the
build directory. The result will be as if we hand-wrote the generated code in
our `lib.rs` file.

We will also need a `src/jni_c_header.rs` with the following:

```rust
// src/jni_c_header.rs
#![allow(
    non_upper_case_globals,
    dead_code,
    non_camel_case_types,
    improper_ctypes,
    non_snake_case,
    clippy::unreadable_literal,
    clippy::const_static_lifetime
)]

include!(concat!(env!("OUT_DIR"), "/jni_c_header.rs"));
```


This section uses flapigen to expand the [`foreign_class`] macro into many
java functions. If you want to see what that looks like, install [`cargo-expand`]
and run `cargo expand`. You will get a lot of generated rust code.

[`foreign_class`]: https://dushistov.github.io/flapigen-rs/foreign-class.html
[`cargo-expand`]: https://crates.io/crates/cargo-expand

Once everything is setup, run `cargo check` or `cargo build` to generate the java files in `lib/src/main/java/my/java/lib/`

## Java Glue

Make sure to run `cargo build` as we'll be using the files generate the
following directory `lib/src/main/java/my/java/lib/` to continue:

```bash
Foo.java
InternalPointerMarker.java
JNIReachabilityFence.java
Library.java
```

The gradle setup step created `Library.java`, and flapigen the other three files. `Foo.java` is the file we care about, and it should look like this:

```java
// Automatically generated by flapigen
package my.java.lib;

public final class Foo {

    public Foo(int a0) {
        mNativeObj = init(a0);
    }
    private static native long init(int a0);

    public final void set_field(int a0) {
        do_set_field(mNativeObj, a0);
    }
    private static native void do_set_field(long self, int a0);

    public final int val() {
        int ret = do_val(mNativeObj);

        return ret;
    }
    private static native int do_val(long self);

    public synchronized void delete() {
        if (mNativeObj != 0) {
            do_delete(mNativeObj);
            mNativeObj = 0;
       }
    }
    @Override
    protected void finalize() throws Throwable {
        try {
            delete();
        }
        finally {
             super.finalize();
        }
    }
    private static native void do_delete(long me);
    /*package*/ Foo(InternalPointerMarker marker, long ptr) {
        assert marker == InternalPointerMarker.RAW_PTR;
        this.mNativeObj = ptr;
    }
    /*package*/ long mNativeObj;
}
```

The src/java_glue.rs file we wrote in the section above instructed `flappigen` to provision and manipulate `mNativeObj`.

Let's run `./gradlew test` to execute the java tests. This step ensures the tests have been setup correctly before we go and break them.

To run the java tests, run `./gradlew test`. This won't test that we've hooked
up the rust quite yet but it's important to make sure your tests run correctly
before we go and break them. :wink:

### FooTest.java

Now, let's add a failing java unit test which calls our rust. The purpose of
this is to ensure that the unit test is actually called and that the library is
loaded correctly. Create the file
`lib/src/test/java/my/java/lib/FooTest.java`:

```java
// lib/src/test/java/my/java/lib/FooTest.java
package my.java.lib;

import org.junit.Test;
import static org.junit.Assert.*;
import my.java.lib.Foo;

public class FooTest {
    @Test public void testSomeLibraryMethod() {
        Foo foo = new Foo(10);
        assertTrue("Foo.val", foo.val() == 15); // This will fail later.
    }
}
```

Now if you run `./gradlew test` you should get the following error:

```bash
$ ./gradlew test

> Task :lib:test FAILED

my.java.lib.FooTest > testSomeLibraryMethod FAILED
    java.lang.UnsatisfiedLinkError at FooTest.java:9

2 tests completed, 1 failed

FAILURE: Build failed with an exception.

* What went wrong:
Execution failed for task ':lib:test'.
> There were failing tests. See the report at: file:///home/simlay/projects/infinyon/fluvio-demo-apps-rust/my-java-lib-blog-post/lib/build/reports/tests/test/index.html

* Try:
Run with --stacktrace option to get the stack trace. Run with --info or --debug option to get more log output. Run with --scan to get full insights.

* Get more help at https://help.gradle.org

BUILD FAILED in 741ms
3 actionable tasks: 2 executed, 1 up-to-date
```

This means we need to link our rust as a static library.

## Graldle build

The `gradle init` step in the [setup](#setup) generated a template of the
project. We just need to add to `lib/build.gradle`:

```groovy
// Append to `lib/build.gradle`
def rustBasePath = ".."

// execute cargo metadata and get path to target directory
tasks.create(name: "cargo-output-dir", description: "Get cargo metadata") {
    new ByteArrayOutputStream().withStream { os ->
        exec {
            commandLine 'cargo', 'metadata', '--format-version', '1'
            workingDir rustBasePath
            standardOutput = os
        }
        def outputAsString = os.toString()
        def json = new groovy.json.JsonSlurper().parseText(outputAsString)

        logger.info("cargo target directory: ${json.target_directory}")
        project.ext.cargo_target_directory = json.target_directory
    }
}
// Build with cargo
tasks.create(name: "cargo-build", type: Exec, description: "Running Cargo build", dependsOn: "cargo-output-dir") {
    workingDir rustBasePath
    commandLine 'cargo', 'build', '--release'
}

tasks.create(name: "rust-deploy", type: Sync, dependsOn: "cargo-build") {
    from "${project.ext.cargo_target_directory}/release"
    include "*.dylib","*.so"
    into "rust-lib/"
}
clean.dependsOn "clean-rust"
tasks.withType(JavaCompile) {
    compileTask -> compileTask.dependsOn "rust-deploy"
}

sourceSets {
    main {
        java {
            srcDir 'src/main/java'
        }
        resources {
            srcDir 'rust-lib'
        }
    }
}
```

In short, this adds:
* runs `cargo build --release` as a build step
* Copies the cydylib from `./target/release/` into `lib/rust-lib`
* And then adds the `rust-lib` directory as a `resource` to the [Gradle SourceSets].

[Gradle SourceSets]: https://docs.gradle.org/current/dsl/org.gradle.api.tasks.SourceSet.html

To verify that the rust is in our Jar, run:

```bash
./gradlew build -x test && jar tf lib/build/libs/lib.jar
```
It should look like:

```bash
META-INF/
META-INF/MANIFEST.MF
my/
my/java/
my/java/lib/
my/java/lib/Foo.class
my/java/lib/Library.class
my/java/lib/InternalPointerMarker.class
my/java/lib/JNIReachabilityFence.class
libmy_java_lib.so
```

## The magic step - loading your runtime library from your jar.

It seem's there's no way to test you your library is linked correctly at
compile time using gradle or java. When the program starts, the user of a shared library must have:
```java
static { System.loadLibrary("my_java_lib"); }
```
in their source someplace to [load the library at runtime]

We strive to make our client libraries have a very nice user experience. So
requiring the user add the shared library to their isn't ideal.  Fortunately,
          [a nice internet patron has a workround] which looks at the jar,
          unzips it in a temp directory and then calls `System.load`.

Simply download [Adam Heinrich's NativUtils.java] to
`lib/src/main/java/my/java/lib/NativeUtils.java`, change the package to
`my.java.lib`.

Then go back to your flaipgen `Foo` class and change it to:

```rust
foreign_class!(class Foo {
    self_type Foo;
    constructor Foo::new(_: i32) -> Foo;
    fn Foo::set_field(&mut self, _: i32);
    fn Foo::val(&self) -> i32;
    foreign_code r#"
        static {
            try {
                NativeUtils.loadLibraryFromJar("/libmy_java_lib.so");
            } catch (java.io.IOException e) {
                e.printStackTrace();
            }
        }"#;
});
```

The `foreigh_code` block will add `static { ... }` block to the `Foo` class
declaration in the java.


[load the library at runtime]: https://www.tutorialspoint.com/java/lang/runtime_loadlibrary.htm
[a nice internet patron has a workround]: https://www.adamh.cz/blog/2012/12/how-to-load-native-jni-library-from-jar/
[Adam Heinrich's NativUtil.java]: https://raw.githubusercontent.com/adamheinrich/native-utils/master/src/main/java/cz/adamh/utils/NativeUtils.java


## Testing it all out

In our [FooTest.java](#footestjava), we wrote an assert function that would
fail. Let's verify that this actually fails. Do this by running `./gradlew test`. It should fail with the following assertion error:

```bash
$ ./gradlew test

> Task :lib:cargo-build
    Finished release [optimized] target(s) in 0.04s

> Task :lib:test FAILED
JNI_OnLoad begin

my.java.lib.FooTest > testSomeLibraryMethod FAILED
    java.lang.AssertionError at FooTest.java:10

2 tests completed, 1 failed

FAILURE: Build failed with an exception.

* What went wrong:
Execution failed for task ':lib:test'.
> There were failing tests. See the report at: file:///home/simlay/projects/infinyon/fluvio-demo-apps-rust/my-java-lib-blog-post/lib/build/reports/tests/test/index.html

* Try:
Run with --stacktrace option to get the stack trace. Run with --info or --debug option to get more log output. Run with --scan to get full insights.

* Get more help at https://help.gradle.org

BUILD FAILED in 943ms
6 actionable tasks: 2 executed, 4 up-to-date
```

Now, let's update our `FooTest.java` to be:

```java
package my.java.lib;

import org.junit.Test;
import static org.junit.Assert.*;
import my.java.lib.Foo;

public class FooTest {
    @Test public void testSomeLibraryMethod() {
        Foo foo = new Foo(10);
        assertTrue("Foo.val", foo.val() == 10);
        foo.set_field(15);
        assertTrue("Foo.val", foo.val() == 15);
    }
}
```

And you should see:

```bash
$ ./gradlew test

> Task :lib:cargo-build
    Finished release [optimized] target(s) in 0.04s

    BUILD SUCCESSFUL in 624ms
    6 actionable tasks: 1 executed, 5 up-to-date
```

And there you go! You've now called rust from java and can distribute the rust
in the jar!

## Conclusion

This post is a bit longer and the code is a bit more verbose than I normally
post about but this is a pretty clean setup. You can get the source for this
post in our [fluvio-demo-apps-rust] repository.

[fluvio-demo-apps-rust]: https://github.com/infinyon/fluvio-demo-apps-rust/tree/master/my-java-lib-blog-post

Adding cleaner docs to JavaDoc is also another thing we could talk about we
wanted to keep this post short. You can checkout how we did better [documentation generation] with [Fluvio Java Client].

[Fluvio Java Client]: https://github.com/infinyon/fluvio-client-java/blob/d5dd4c6e9bc9422c8f07b6a7e24dc2fa4530602f/src/java_glue.rs.in#L44-L104
[documentation generation]: https://infinyon.github.io/fluvio-client-java/com/infinyon/fluvio/package-summary.html
