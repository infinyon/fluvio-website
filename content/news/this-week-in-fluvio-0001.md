---
title: This Week in Fluvio 1
date: 2021-08-03
weight: 20
toc: false
---

Welcome to the very first edition of This Week in Fluvio, our weekly newsletter
for development updates to [Fluvio open source]. Fluvio is a distributed,
programmable streaming platform written in Rust.

[Fluvio open source]: https://github.com/infinyon/fluvio

## Release of Fluvio `0.9.0`

Today we're releasing a new "major version" of Fluvio, which includes crate-level
and protocol-level breaking changes from the `0.8.x` line of releases. Note that
although Fluvio is still pre-1.0, we do our best to stick to semantic versioning,
treating the second digit as our major version. Let's dive in and see what new
features and breaking changes we have to talk about.

### New target support and the Tier system

We've been putting in a lot of work to support new build targets so that users can
run the Fluvio CLI on more platforms and architectures. We have also introduced a
Tier system that describes which targets must be able to build and pass tests. Here
is a breakdown of our current Tiers, what they mean, and which executables belong
to those tiers:

#### Tier 1

Tier 1 targets are those that _must_ compile and pass tests. We have configured our
CI to reject any changes that cause any of these targets to stop compiling successfully
or to stop passing tests. The following executable/target combinations are currently
considered Tier 1:

- Fluvio CLI (`fluvio`)
  - `x86_64-unknown-linux-musl` (Linux x86 64-bit)
  
- Fluvio cluster (`fluvio-run`)
  - `x86_64-unknown-linux-musl` (Linux x86 64-bit)

#### Tier 2

Tier 2 targets are those that must _compile_, but for which we don't yet run tests
or block progress on those tests passing.

- Fluvio CLI (`fluvio`)
  - `aarch64-unknown-linux-musl` (Linux ARM 64-bit)
  - `x86_64-pc-windows-msvc` (Windows x86 64-bit)
  - `aarch64-apple-darwin` (Apple M1)
  - `x86_64-apple-darwin` (Apple x86 64-bit)
  - `arm-unknown-linux-gnueabihf` (Raspberry Pi zero)
  - `armv7-unknown-linux-gnueabihf` (Raspberry Pi)

### Better Kubernetes Support

With 0.9.0, Fluvio now supports any standards compliant Kubernetes distribution.  It defaults to standard storage drivers, but can be configured to use different drivers.  We have tested with the following Kubernetes distributions:
- Minikube
- Kind
- K3d
- AWS EKS


Please see Fluvio's [Kubernetes documentation](https://fluv.io/docs/kubernetes.html) for more information. 

#### Helm changes

Fluvio's CLI bundles helm charts for easy installation.  Fluvio's charts are no longer published to the Fluvio registry. 

Please use following commands to update your Fluvio installation:

```
fluvio cluster upgrade --sys      # upgrade CRD
fluvio cluster upgrade            # upgrade rest 
```


### Error Handling for SmartStreams ([#1198][1])

One of Fluvio's premiere features is SmartStreams, which allow users to write
custom WebAssembly modules to perform server-side data processing. Until recently,
there was no way for user code to return Errors to indicate that something had
gone wrong while processing records.

Prior to `0.9.0`, [the only type of SmartStream was filters], which looked something
like this:

[the only type of SmartStream was filters]: https://www.infinyon.com/blog/2021/06/smartstream-filters/

```rust
use fluvio_smartstream::{smartstream, Record};

#[smartstream(filter)]
pub fn filter_odd(record: &Record) -> bool {
    // Parse the input bytes as a UTF-8 string, or return false
    let string_result = std::str::from_utf8(record.value.as_ref());
    let string = match string_result {
        Ok(s) => s,
        _ => return false,
    };
  
    // Parse the string as an i32, or return false
    let int_result = string.parse::<i32>();
    let int = match int_result {
        Ok(i) => i,
        _ => return false,
    };
  
    int % 2 == 0
}
```

Note that this function is _required_ to return a boolean, which indicates whether
the Records should be kept in the stream (if true) or discarded (if false). However,
there is no way to indicate whether a logic error has occurred during processing.

For example, what happens if the Record data we are given is not valid UTF-8 data?
We have no way to report this situation to the consumer, and therefore the best
course of action we have is to just return `false` and discard any records that
are invalid. This means that we risk mixing up the logically distinct cases of:

- "we have valid Records, and successfully discarded some of them", and
- "we have an invalid Record, so ignore it"

and since we had no way to report this to the consumer, it is very difficult to debug
this situation.

#### Enter SmartStream Error handling

With the `0.9.0` update, SmartStreams are now written like this!

```rust
use fluvio_smartstream::{smartstream, Record, Result};

#[smartstream(filter)]
pub fn filter_odd(record: &Record) -> Result<bool> {
  let string = std::str::from_utf8(record.value.as_ref())?;
  let int = string.parse::<i32>()?;
  Ok(int % 2 == 0)
}
```

Notice that now, the filter function returns a `Result<bool>`, meaning that
SmartStream authors now have tha ability to return an `Err` describing any problems
that happened when running their code.

The `fluvio_smartstream::Result` type allows you to return _any_ error type
that implements `std::error::Error`*, which means that you can simply propagate
most errors up and out using `?`, like we do in the example above.

> *and `Send + Sync + 'static`. We use the [`eyre`] crate to capture returned errors.

[`eyre`]: https://github.com/yaahc/eyre

This SmartStream parses incoming records as integers, then filters out odd numbers.
When we run a consumer with this SmartStream, we can see the filtered data, and we
can see the consumer deliver our error to us if we give an input that can't be parsed
as an integer.

<video controls width="860px" title="Producing integers to the filter-odd topic, followed by the word nine, results in an error">
  <source src="/news/images/0001/filter-odd.mov">
</video>

<br/>
<br/>

### SmartStream Maps

Another big feature that we've had in the works for a while is a new type of SmartStream,
`#[smartstream(map)]`, used to _transform_ the data in each record in a stream. This feature
has been available in "preview" since 0.8.5, but we did not want to release it until we had
error-handling ready for SmartStreams, which happened this release! Let's take a look at
what a SmartStream Map function looks like.

```rust
use fluvio_smartstream::{smartstream, Record, RecordData, Result};

#[smartstream(map)]
pub fn map(record: &Record) -> Result<(Option<RecordData>, RecordData)> {
  let key = record.key.clone();

  let string = std::str::from_utf8(record.value.as_ref())?;
  let int = string.parse::<i32>()?;
  let value = (int * 2).to_string();

  Ok((key, value.into()))
}
```

> See the [full source code for this example on GitHub]!

[full source code for this example on GitHub]: https://github.com/infinyon/fluvio/tree/master/src/smartstream/examples/map_double

In this example, we are reading in Records and first parsing them as UTF-8 strings,
then parsing those strings as integers. If either of those steps fails, 
the error is returned with `?` and the consumer receives an error item in the stream.

Notice that the return type for Map is different from we have seen before with Filters.
In order to edit the Records in our stream, we manipulate them in our function and
then return the transformed output. The successful return value is a tuple of the new
Key (optional) and Value for the output record. The `RecordData` type may be constructed from
any type that has `impl Into<Vec<u8>>`, so you can just use `.into()` for a lot of types
such as String when you want to return them.

Let's take a look at how this example works!

<video controls width="860px" title="Producing integers to the doubling topic, followed by the word nine, results in an error">
  <source src="/news/images/0001/map-double.mov">
</video>

<br>
<br>

As you can see, when we give valid integers, our output comes back transformed as expected -
the integers have been doubled. However, if we give invalid input, the Consumer CLI prints
the error that was returned by the user code, along with diagnostic information such as the
record's offset.

### Internal improvements

For the full list of updates in this release, [check out our CHANGELOG]. Here are some highlights:

[check out our CHANGELOG]: https://github.com/infinyon/fluvio/blob/master/CHANGELOG.md

#### Improved `install.sh` to work for more targets ([#1269][2])

Prior to `0.9.0`, the one-line install script only worked for MacOS and Linux on x86 architectures.
Now, it also works for non-x86 targets such as `arm` and `armv7`, which, notably, allows it to
work directly on Raspberry Pi.

#### Updated `ConsumerConfig` builder API to match standard builder patterns ([#1271][3])

Our `ConsumerConfig` type is used when constructing consumers programmatically. Prior to `0.9.0`,
we used an [owned-builder] pattern (chained by passing `mut self`), which is less flexible than the
[mutable-builder] pattern (chained by passing `&mut self`) that we have now adopted.

[owned-builder]: https://docs.rs/derive_builder/0.10.2/derive_builder/#owned-aka-consuming
[mutable-builder]: https://docs.rs/derive_builder/0.10.2/derive_builder/#mutable-aka-non-consuming-recommended

#### Improved `#[derive(fluvio_protocol::{Encoder, Decoder})]` for enums ([#1232][4])

We use the procedural macros `fluvio_protocol::{Encoder, Decoder}` to derive traits by the same
name. These macros used to have a limitation where they did not work on enums that carry data,
but now this works as expected.

## Conclusion

That's it for this week, we'll be publishing a newsletter once a week from now on, so stay tuned
for more Fluvio updates! If you have any questions or would like to get involved, feel free to
[join our Discord channel]!

Until next week!

[1]: https://github.com/infinyon/fluvio/issues/1198 
[2]: https://github.com/infinyon/fluvio/issues/1269
[3]: https://github.com/infinyon/fluvio/issues/1271
[4]: https://github.com/infinyon/fluvio/pull/1232
[join our Discord channel]: https://discordapp.com/invite/bBG2dTz
