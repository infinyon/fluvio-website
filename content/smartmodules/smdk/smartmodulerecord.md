---
title: SMDK - SmartModuleRecord
menu: SmartModuleRecord
weight: 40
toc: false
---

Inside the SmartModule Context, Fluvio `Record` becomes `SmartModuleRecord`.

`SmartModuleRecord` is a wrapper on the standard `Record`, which provides extra
details on the `Record` such as the moment when the `Record` was produced.

-> `SmartModuleRecord` was introduced on [Fluvio v0.10.14][1], we recommend upgrading to `SmartModuleRecord` as future versions will completely deprecate `Record` in the SmartModule Context.

## Usage

When building any type of SmartModules you will have access to each record, in
the SmartModule function these records are represented with the
`SmartModuleRecord` struct.

Consider the following SmartModule of type `Map`, which processes weather data:

```rust
use fluvio_smartmodule::{smartmodule, Result, SmartModuleRecord, RecordData};

#[smartmodule(map)]
pub fn map(record: &SmartModuleRecord) -> Result<(Option<RecordData>, RecordData)> {
    let string = std::str::from_utf8(record.value.as_ref())?; // parses UTF-8 string from bytes
    let feels_like = string.parse::<i32>()?; // attemps to parse the string value into a `i32`
    let feels_like = feels_like.to_string(); // turs the integer into a string

    Ok((key, feels_like.into())) // returs to the stream
}
```

For weather casting it would be helpful to also have access to the moment when
the data was captured. We could have our wather value mapped to a string which
also holds a timestamp.

```diff
use fluvio_smartmodule::{smartmodule, Result, SmartModuleRecord, RecordData};

#[smartmodule(map)]
pub fn map(record: &SmartModuleRecord) -> Result<(Option<RecordData>, RecordData)> {
    let string = std::str::from_utf8(record.value.as_ref())?; // parses UTF-8 string from bytes
    let feels_like = string.parse::<i32>()?; // attemps to parse the string value into a `i32`
-   let feels_like = feels_like.to_string(); // turs the integer into a string
+   let feels_like_with_timestamp = format!("{}_{}", record.timestamp(), feels_like);

    Ok((key, feels_like_with_timestamp.into())) // returs to the stream
}
```

With this small change we are injecting insightful data to our record. This is
a simple use case but we can have more based on the implementation and business
requirements.

[1]: https://github.com/infinyon/fluvio/blob/master/CHANGELOG.md#platform-version-01014---2023-07-28
