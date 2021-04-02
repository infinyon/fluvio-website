---
title: Sending tuples from Node to Rust and back
author:
    name: "Nick Mosher"
    github: "nicholastmosher"
description: "How I added tuple support to node-bindgen"
date: 2021-04-02
slug: node-bindgen-tuples
url: /blog/2021/04/node-bindgen-tuples
img: blog/images/batch-producer/rust-node-blog-social.png
tile: blog/images/batch-producer/rust-node-blog-tile.png
twitter-card: summary_large_image
code:
    height: 740
---

This week in Fluvio, I want to talk about an interesting problem I encountered
while implementing a Batch Producer API for the Fluvio client. As part of our
feature development process, we update each of our language clients with
new APIs for interacting with the new functionality. This particular problem
cropped up while I was implementing the Node client API for batch record producing,
and has to do with passing Tuples from Node to Rust.

In our batch producer API, the goal is to send multiple records to Fluvio in one
request. Records are represented as a value and potentially a key. I chose to
expose this functionality with a function that accepts any iterator over items
of type `(Option<K>, V)`. The elements of this tuple represent the potential key
and the definite value. The specific function I wanted to write in Rust and call
from Node is the following:

```rust
async fn send_all<I, K, V>(records: I) 
where
    K: Into<Vec<u8>>,
    V: Into<Vec<u8>>,
    I: IntoIterator<Item=(Option<K>, V)>,
{ /* ... */ }
```

To be as flexible as possible, we're using a couple of generics so that callers
have more freedom with the values they can pass to us. An english description of
this API might go something like this:

> Provide a value `records` which can be turned into an iterator over tuples
> `(Option<K>, V)`, where K and V are each types that can be converted into
> byte buffers.

Here are some quick examples of values you could provide to this API:

```rust
let records = vec![ (Some("Hello"), String::from("World!")) ];
let records = Some( ( Some(vec![0x48, 0x65, 0x6c, 0x6c, 0x6f]), "World!" ) );
let records = {
    let mut r = HashSet::new();
    r.insert( (Some("Hello"), "World!") );
    r
};
```

Now let's think about what we want out of a Node API. In Javascript, we don't really
abstract over different types of iterators often enough for it to be relevant, so we
can settle with passing a plain array. What about the elements of our array though?
Javascript doesn't technically have tuples, but a common pattern to use instead is
simply choosing a fixed-size array and putting all of their values into it. In fact,
typescript even supports annotating this situation! Let's take a look at how this
plays out:

```typescript
type KeyValue = [string, string];
interface TopicProducer {
    sendAll(records: KeyValue[]): Promise<void>
}
```

I created the type alias `KeyValue` to represent our tuple type, where each item in
the tuple is a `string`. Our function `sendAll` wants us to give it an array of these
tuples.

Alright, so now we've got a handle of the underlying Rust API that we want to bind
to, and we know what we want our Node API to look like as well. Let's take a look at
how to glue them together using [node-bindgen].

[node-bindgen]: https://github.com/infinyon/node-bindgen/

### Node-bindgen

Node-bindgen is a Rust crate for automatically generating glue code for Node programs
that want to interact with Rust functions and types. It works by providing an attribute,
`#[node_bindgen]`, that can be applied to functions and implementation blocks. At
compile-time, node-bindgen generates conversion code for Node and Rust code to
pass values back and forth, leveraging Node's N-API. An example node-bindgen function
might look like this:

```rust
use node_bindgen::derive::node_bindgen;

#[node_bindgen]
fn sum_all(ints: Vec<i32>) -> i32 {
    ints.iter().fold(0, |a, b| a + b)
}
```

The way this works is via a pair of traits for converting values to and from the
Node representation. We have `trait JSValue<'_>`, which describes how to convert
a JS value into a Rust value, and we have `trait TryIntoJs`, which does the opposite.
This is nice because it allows us to define conversions for new types, as well as
to define conversions for _compound_ types, built from other convertible types.

We've already seen an example of a compound data type, `Vec<i32>`! Vec has a blanket
implementation of `JSValue` for any other type `T` as long as `T` also implements
`JSValue`. Great! So in principle, as long as we're just combining basic types, everything
should be fine and dandy, right? Let's try it out with the API we want:

```rust
#[node_bindgen]
async fn send_all(records: Vec<(String, String)>) {
    // todo
}
```

When I compiled it, I was met with this error message:

```bash
error[E0277]: the trait bound `(String, String): JSValue<'_>` is not satisfied
 --> tuples/src/lib.rs:3:1
  |
3 | #[node_bindgen]
  | ^^^^^^^^^^^^^^^ the trait `JSValue<'_>` is not implemented for `(String, String)`
  |
  = note: required because of the requirements on the impl of `JSValue<'_>` for `Vec<(String, String)>`
  = note: required because of the requirements on the impl of `ExtractArgFromJs<'_>` for `Vec<(String, String)>`
  = note: this error originates in an attribute macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
error: could not compile `nj-example-tuple`

To learn more, run the command again with --verbose.
```

Ok, well I didn't _actually_ think this was going to be easy, did I?
From the looks of it, `node-bindgen` is missing implementations of `JSValue` for tuple types!
But this is actually kind of cool, now I have a chance to make an improvement to
`node-bindgen` that will be useful to other users even outside of Fluvio.

### Implementing `JSValue` for `(A, B, ...)`

So now I know that I'll need to write some new `JSValue` implementations. We will need
a special implementation for each size of tuple that we want to support. Right away, it
smells like a perfect recipe for a macro! But before diving straight into writing macros,
I wanted to first try out this strategy with just one case and make sure it works.

For the first go-round, I'll write an implementation for 2-Tuples. This implementation will
need to be generic over the two component pieces, and it will require that both pieces also
implement `JSValue`. Our impl block will look like this:

```rust
impl<'a, A, B> JSValue<'a> for (A, B)
where
    A: JsValue<'a>,
    B: JsValue<'a>,
{
    fn convert_to_rust(env: &'a JsEnv, js_value: napi_value) -> Result<Self, NjError> {
        // TODO
    }
}
```

Alright, so far so good, but how to we implement `convert_to_rust`? Well `JsEnv` is a type
that represents the environment surrounding the value that we are attempting to convert to
Rust. Contained somewhere within it are all the values that were in scope when the Node
program called into our Rust function. We also have `js_value`, which I think of as a
handle to the specific value that we want to retrieve from the environment.

So let's take a step back for a moment and think about the values that we'll actually be
dealing with. In Node, a tuple is just a list of values. And we already have a way of
receiving lists from Node, it's the `impl<T> JSValue<'_> for Vec<T>`! Let's take a look at
how that works:

```rust
impl<'a, T> JSValue<'a> for Vec<T> where T: JSValue<'a>,
{
    fn convert_to_rust(env: &'a JsEnv, js_value: napi_value) -> Result<Self, NjError> {
        use crate::sys::napi_get_array_length;
        if !env.is_array(js_value)? {
            return Err(NjError::Other("not array".to_owned()));
        }

        let mut length: u32 = 0;
        napi_call_result!(napi_get_array_length(env.inner(), js_value, &mut length))?;

        let mut elements = vec![];
        for i in 0..length {
            let js_element = env.get_element(js_value, i)?;
            elements.push(T::convert_to_rust(env, js_element)?);
        }

        Ok(elements)
    }
}
```

Alright, so it looks like what's happening is:

- First, we check with the `JsEnv` that the value given to us was actually an array.
I think of this as the moral equivalent in JS of using `Array.isArray(js_value)`.
  
- Then, we ask the environment to give us the length of the array that was passed to us.

- Finally, we loop over all the slots in the JS array and ask the environment for the
object handle of the value at that position in the array. Then, since we are expecting
all of those values to have the same type `T`, we use `T`'s own implementation of
`convert_to_rust` to convert the element of the array and put it into our Vec.

Ok, awesome. I think there is a lot of information here that we can re-purpose for
what we need to do for tuples. Here's the plan for things we'll do the same and things
we'll change about this process:

- Since we know that JS "tuples" are arrays, we'll use the same techniques to check
that our value is an array and to get values out of it.
  
- We know that tuples have a fixed length, so we'll check that the length of the array
that is passed to us matches the number of elements in the tuple we are expecting. For
our initial implementation with `(A, B)`, we'll make sure the array has length 2.
  
- When we convert the array elements to Rust, instead of using just a single type
`T` for all the elements like Vec did, we'll use `A` and `B`'s implementations of
`convert_to_rust` for the corresponding elements of the array.
  
Alright, let's see if we can execute on this plan:

```rust
impl<'a, A, B> JSValue<'a> for (A, B)
where
    A: JSValue<'a>,
    B: JSValue<'a>,
{
    fn convert_to_rust(env: &'a JsEnv, js_value: napi_value) -> Result<Self, NjError> {
        use crate::sys::napi_get_array_length;
        if !env.is_array(js_value)? {
            return Err(NjError::Other("not array".to_owned()));
        }

        let required_length = 2; // Since this tuple has 2 elements
        let mut length: u32 = 0;
        napi_call_result!(napi_get_array_length(env.inner(), js_value, &mut length))?;
        if length != required_length {
            return Err(NjError::Other(format!("Expected array of length {}", required_length)));
        }
        
        let napi_value_a = env.get_element(js_value, 0)?;
        let a = A::convert_to_rust(env, napi_value_a)?;

        let napi_value_b = env.get_element(js_value, 1)?;
        let b = B::convert_to_rust(env, napi_value_b)?;

        let tuple = (a, b);
        Ok(tuple)
    }
}
```

Ok, let's try it out with the `send_all` function and see if it worked!

```bash
❯ cargo build
   Compiling nj-core v4.2.0 (/Users/nick/infinyon/node-bindgen/nj-core)
   Compiling node-bindgen v4.3.0 (/Users/nick/infinyon/node-bindgen)
   Compiling nj-example-tuple v0.1.0 (/Users/nick/infinyon/node-bindgen/examples/tuples)
warning: unused variable: `records`
 --> tuples/src/lib.rs:4:19
  |
4 | async fn send_all(records: Vec<(String, String)>) {
  |                   ^^^^^^^ help: if this is intentional, prefix it with an underscore: `_records`
  |
  = note: `#[warn(unused_variables)]` on by default

warning: 1 warning emitted

    Finished dev [unoptimized + debuginfo] target(s) in 4.05s
```

Woohoo! We've got it compiling. Let's write a quick body that just prints everything
it receives so that we can just check that everything Really Actually Works.

```rust
#[node_bindgen]
async fn send_all(records: Vec<(String, String)>) {
    for (key, value) in records {
        println!("Got Key={}, Value={}", key, value);
    }
}
```

We'll call it from a typescript module like this:

```typescript
const assert = require('assert');
let addon: TestTuples = require('./dist');

type KeyValue = [string, string];
interface TestTuples {
    sendAll(records: KeyValue[]): Promise<void>
}

const records: KeyValue[] = [ ["Apple", "Banana"] ];
addon.sendAll(records)
```

And the moment of truth:

```bash
❯ npx ts-node ./test.ts
Got Key=Apple, Value=Banana
```

Great! We can now pass tuples from Node to Rust, and everything gets converted as expected!
I won't write it all out here, but I did test with other permutations of types, array lengths, etc.,
and everything works pretty much how you'd expect it to. If you want to see a bit more of an example,
check out the new [node-bindgen example for tuples] :)

[node-bindgen example for tuples]: https://github.com/infinyon/node-bindgen/tree/master/examples/tuples

### Bonus: Writing a macro to implement more tuples

Alright, so we've written enough to fit our use-case, but we can't stop there!
We want to be able to pass other sizes of tuples between Rust and Node. However,
we don't want to have to hand-write the implementations for each size of tuple, so
let's see if we can make a macro do the job for us!

What we want is a macro that looks something like this:

```rust
impl_js_value_for_tuple!(A);
impl_js_value_for_tuple!(A, B);
impl_js_value_for_tuple!(A, B, C);
```

Ok, so let's give this a shot:

```rust
macro_rules! impl_js_value_for_tuple_first_attempt {
    ( $( $t:ident ),+ $(,)? ) => {
        impl<'a $(, $t)+ > crate::JSValue<'a> for ($($t,)+)
        where
            $($t: JSValue<'a> + Send,)+
        {
            fn convert_to_rust(env: &'a JsEnv, js_value: napi_value) -> Result<Self, NjError> {
                use crate::sys::napi_get_array_length;
                if !env.is_array(js_value)? {
                    return Err(NjError::Other("Tuples must come from JS arrays".to_owned()));
                }

                let mut length: u32 = 0;
                napi_call_result!(napi_get_array_length(env.inner(), js_value, &mut length))?;
                let required_length = ???;
                if length != required_length {
                    return Err(NjError::Other(format!("{n}Tuple must have exactly length {n}", n = required_length)));
                }

                $(
                    let js_element = env.get_element(js_value, ???)?;
                    #[allow(non_snake_case)]
                    let $t = $t::convert_to_rust(env, js_element)?;
                )+

                Ok(( $($t,)+ ))
            }
        }
    }
}
```

So this comes pretty close, but there are a couple problems. Firstly, we want each
tuple implementation to check that the provided Node array has the same length as the
tuple type we are dealing with, but this form of macro doesn't provide any amenities
for counting the size of the type, or for selecting individual elements from the array
by index. There are some tricks that we _might_ be able to leverage from the
[little book of macros], but that seems like it would make things much more complicated
than I want to deal with.

[little book of macros]: https://danielkeep.github.io/tlborm/book/index.html

Well, let's look for inspiration. I know I've seen libraries use this pattern of implementing
traits for compound types, where the component types implement the traits. Let's take a look
at `serde` and see how that handles tuples.

Aha! It looks like serde does [almost the exact same thing] I tried to do, it just does it
in a way that is straightforward and actually works! The serde `tuple_impls` macro takes
a `$len` expression with the number of elements in the tuple, identifiers for the generic
parameter, _and_ tokens `$n:tt` indicating the indices of each item in the tuple.

[almost the exact same thing]: https://github.com/serde-rs/serde/blob/master/serde/src/ser/impls.rs#L306-L346 

Let's see if we can leverage this pattern to do what we want.

```rust
macro_rules! impl_js_value_for_tuple {
    ( $( $len:expr => ( $( $n:tt $t:ident ),+ $(,)? ))+ ) => {
        $(
            impl<'a $(, $t)+ > crate::JSValue<'a> for ($($t,)+)
            where
                $($t: JSValue<'a> + Send,)+
            {
                fn convert_to_rust(env: &'a JsEnv, js_value: napi_value) -> Result<Self, NjError> {
                    use crate::sys::napi_get_array_length;
                    if !env.is_array(js_value)? {
                        return Err(NjError::Other("Tuples must come from JS arrays".to_owned()));
                    }

                    let mut length: u32 = 0;
                    napi_call_result!(napi_get_array_length(env.inner(), js_value, &mut length))?;
                    let required_length = $len;
                    if length != required_length {
                        return Err(NjError::Other(format!("{n}Tuple must have exactly length {n}", n = required_length)));
                    }

                    $(
                        let js_element = env.get_element(js_value, $n)?;
                        #[allow(non_snake_case)]
                        let $t = $t::convert_to_rust(env, js_element)?;
                    )+

                    Ok(( $($t,)+ ))
                }
            }
        )+
    }
}

impl_js_value_for_tuple! {
    1 => (0 T0)
    2 => (0 T0, 1 T1)
    3 => (0 T0, 1 T1, 2 T2)
    4 => (0 T0, 1 T1, 2 T2, 3 T3)
    5 => (0 T0, 1 T1, 2 T2, 3 T3, 4 T4)
    6 => (0 T0, 1 T1, 2 T2, 3 T3, 4 T4, 5 T5)
    7 => (0 T0, 1 T1, 2 T2, 3 T3, 4 T4, 5 T5, 6 T6)
    8 => (0 T0, 1 T1, 2 T2, 3 T3, 4 T4, 5 T5, 6 T6, 7 T7)
    9 => (0 T0, 1 T1, 2 T2, 3 T3, 4 T4, 5 T5, 6 T6, 7 T7, 8 T8)
}
```

Bingo! We now have working implementations for converting tuple types from Node
to Rust. This pattern also works out in the opposite direction, and I was able
to write a macro to [impl TryIntoJs for tuples] as well! Mission accomplished,
we can now pass values both ways in tuples.

[impl TryIntoJs for tuples]: https://github.com/infinyon/node-bindgen/blob/master/nj-core/src/convert.rs#L327-L362

## Conclusion

Thanks for reading this far! I hope you enjoyed my little foray into
the world of Rust/Node glue code with node-bindgen, I know I had a lot of fun
working with the blanket impls and tuple macros, it gives me the warm fuzzy feeling
of writing slick composable code 😎.

If you'd like to learn more about the work we do on Fluvio or node-bindgen,
be sure to check out [our website at fluvio.io] and our [Github repo]. If you have
any questions or want to learn more about Fluvio, you can come talk to us on our
[Discord server]. Until next time!

[our website at fluvio.io]: https://fluvio.io
[Github repo]: https://github.com/infinyon/fluvio
[Discord server]: https://discordapp.com/invite/bBG2dTz

#### Quick links:

- [Getting started with Fluvio](/docs/getting-started/)
- [Fluvio CLI reference](/docs/cli-reference/)
- [Fluvio Architecture](/docs/architecture/)
