---
title: Introduction to Fluvio
---

Welcome to the intro guide to Fluvio! The Concepts section helps you learn about all parts of the Fluvio system and to gain deeper understanding of how Fluvio works.

This is a second paragraph,

This is a third paragraph,

## What is Fluvio?

To work with Fluvio, ...

{{< idea >}}
Lorem ipsum is a pseudo-Latin text used in web design, typography, layout, and printing in place of English to emphasise design elements over content. It's also called placeholder (or filler) text. It's a convenient tool for mock-ups. It helps to outline the visual elements of a document or presentation, eg typography, font, or layout. Lorem ipsum is mostly a part of a Latin text by the classical author and philosopher Cicero. Its words and letters have been changed by addition or removal, so to deliberately render its content nonsensical; it's not genuine, correct, or comprehensible Latin anymore. While lorem ipsum's still resembles classical Latin, it actually has no meaning whatsoever. As Cicero's text doesn't contain the letters K, W, or Z, alien to latin, these, and others are often inserted randomly to mimic the typographic appearence of European languages, as are digraphs not to be found in the original.
{{< /idea >}}

<br/>

{{< highlight tcsh>}}
$ fluvio
fluvio 0.1.0
Fluvio Command Line Interface

USAGE:
    fluvio <SUBCOMMAND>

FLAGS:
    -h, --help    Prints help information

SUBCOMMANDS:
    consume       Read messages from a topic/partition
    produce       Write log records to a topic/partition
    spu           SPU operations
    topic         Topic operations
    auth-token    Athorization token operations
    advanced      Advanced operations
    help          Prints this message or the help of the given subcommand(s)s

$  ./target/debug/fluvio auth-token list "test" -o 1
error: cannot retrieve auth topics: Connection refused (os error 61)    
{{< / highlight >}}

* This is a test
{{< highlight rust >}}
//!
//! # Streaming Coordinator Metadata
//!
//! Metadata stores a copy of the data from KV store in local memory.
//!----------------------------------

impl ScMetadata {
    pub fn shared_metadata(config: ScConfig) -> Arc<Self> {
        Arc::new(ScMetadata::new(config))
    }

    /// private function to provision metadata
    fn new(config: ScConfig) -> Self {
        ScMetadata {
            auth_tokens: Arc::new(AuthTokenMemStore::default()),
            spus: Arc::new(SpuMemStore::default()),
            partitions: Arc::new(PartitionMemStore::default()),
            topics: Arc::new(TopicMemStore::default()),
            config: config,
        }
    }

    /// reference to auth tokens
    pub fn auth_tokens(&self) -> &Arc<AuthTokenMemStore> {
        &self.auth_tokens
    }

    /// reference to spus
    pub fn spus(&self) -> &Arc<SpuMemStore> {
        &self.spus
    }

    /// reference to partitions
    pub fn partitions(&self) -> &Arc<PartitionMemStore> {
        &self.partitions
    }

    /// reference to topics
    pub fn topics(&self) -> &Arc<TopicMemStore> {
        &self.topics
    }

    /// reference to config
    pub fn config(&self) -> &ScConfig {
        &self.config
    }

    /// format metadata cache into a table string
    #[allow(dead_code)]
    pub fn table_fmt(&self) -> String {
        let mut table = String::new();
        let newline = format!("\n");

        table.push_str(&self.auth_tokens().table_fmt());
        table.push_str(&newline);
        table.push_str(&self.spus.table_fmt());
        table.push_str(&newline);
        table.push_str(&self.topics.table_fmt());
        table.push_str(&newline);
        table.push_str(&self.partitions.table_fmt());
        table
    }
}
{{< / highlight >}}

* Another test

#### Next Steps
* [Getting Started]({{< relref "getting-started/overview.md" >}})
* [Compare Fluvio with Compare with Other Software]({{< relref "fluvio-vs-others/overview.md" >}})