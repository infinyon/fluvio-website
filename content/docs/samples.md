---
title: __Samples (fuvio elements)__
weight: 100000
hidden: true
---

Welcome to the intro guide to Fluvio! The Concepts section helps you learn about all parts of the Fluvio system and to gain deeper understanding of how Fluvio works.

### Inline Highlights

-> **Prerequisites:** Examples in this section require an existing Fluvio cluster and a topic named "my-topic".<br> Step-by-step instructions are available in [Quick Start](/docs/getting-started/quick-start).


~> **Caution:** Lorem ipsum is a pseudo-Latin text used in web design, typography, layout, and printing in place of English to emphasise design elements over content. It’s also called placeholder (or filler) text. It’s a convenient tool for mock-ups. 


### Block Highlights

{{< idea >}}

**Prerequisites:** Lorem ipsum is a pseudo-Latin text used in web design
* bullet
* list

{{< /idea >}}

{{< caution >}}

 **Caution:** Lorem ipsum is a pseudo-Latin text used in web design, typography, layout, and printing in place of English to emphasise design elements over content. It’s also called placeholder (or filler) text. It’s a convenient tool for mock-ups. 

* bullet 
* list

{{< /caution >}}


{{< fluvio >}}
$ fluvio auth-token create -h
fluvio-auth-token-create 0.1.0
Create auth token

USAGE:
    fluvio auth-token create [FLAGS] [OPTIONS] --max-spu <integer> --min-spu <integer> --token-name <string> --secret <alpha-numeric>

FLAGS:
    -g, --generate-secret    Generate a random secret
    -h, --help               Prints help information

OPTIONS:
    -n, --token-name <string>        Token name
    -s, --secret <alpha-numeric>     Token secret of 16 characters in length

SUBCOMMANDS:
    consume       Read messages from a topic/partition
{{< /fluvio >}}

```
$ fluvio auth-token create -h
fluvio-auth-token-create 0.1.0
Create auth token

USAGE:
    fluvio auth-token create [FLAGS] [OPTIONS] --max-spu <integer> --min-spu <integer> --token-name <string> --secret <alpha-numeric>

FLAGS:
    -g, --generate-secret    Generate a random secret
    -h, --help               Prints help information

OPTIONS:
    -n, --token-name <string>        Token name
    -s, --secret <alpha-numeric>     Token secret of 16 characters in length
    -m, --min-spu <integer>          First SPU id in the match range (inclusive)
    -x, --max-spu <integer>          Last SPU id in the match range (inclusive)
    -t, --token-type <token-type>    Types [possible values: Custom, Managed, Any]
    -c, --sc <host:port>             Address of Streaming Controller
    -P, --profile <profile>          Profile name
```

```javascript
var showMessage = function (){
    alert("Hello World!");
};

showMessage();

var sayHello = function (firstName) {
    alert("Hello " + firstName);
};

showMessage();

sayHello("Bill");
```

```js
flvClient.connect("server:port").then((sc) => {

    sc.leader("my-topic", 0).then((leader) => {
        leader.produce("hello world").then(len => {
            console.log("message sent!");
        });
    })

});
```

```rust
//!----------------------------------
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
            config: config,
        }
    }

    /// format metadata cache into a table string
    #[allow(dead_code)]
    pub fn table_fmt(&self) -> String {
        let mut table = String::new();
        let newline = format!("\n");

        table.push_str(&self.partitions.table_fmt());
        table
    }
}
```

```diff
$ istioctl proxy-status details-v1-6dcc6fbb9d-wsjz4.default
--- Pilot Clusters
+++ Envoy Clusters
@@ -374,36 +374,14 @@
             "edsClusterConfig": {
                "edsConfig": {
                   "ads": {

                   }
                },
                "serviceName": "outbound|443||public-cr0bdc785ce3f14722918080a97e1f26be-alb1.kube-system.svc.cluster.local"
-            },
-            "connectTimeout": "1.000s",
-            "circuitBreakers": {
-               "thresholds": [
-                  {
-
-                  }
-               ]
-            }
-         }
-      },
-      {
-         "cluster": {
-            "name": "outbound|53||kube-dns.kube-system.svc.cluster.local",
-            "type": "EDS",
-            "edsClusterConfig": {
-               "edsConfig": {
-                  "ads": {
-
-                  }
-               },
-               "serviceName": "outbound|53||kube-dns.kube-system.svc.cluster.local"
             },
             "connectTimeout": "1.000s",
             "circuitBreakers": {
                "thresholds": [
                   {

                   }
             }
        }
    }
}
```

|   Operating System     |         Instructions           |
|------------------------|--------------------------------|
| MacOS                  | Use the official installer from <a href="https://nodejs.org" target="_blank">Node.js</a> to install on **macOS**.  |
| Windows                | Use the official installer from <a href="https://nodejs.org" target="_blank">Node.js</a> to install on **Windows**. |
| Linux                  | Use the instructions provided by your **Linux** package manager. <br/> Node.js maintains a list of <a href="https://nodejs.org/en/download/package-manager" target="_blank">supported packages</a>.  |

### Links

* [Visit W3Schools (route page)](https://www.w3schools.com)
* <a href="https://www.w3schools.com" target="_blank">Visit W3Schools (new tab)</a>

### Definition List

A list of definitions:

First Term
: This is the definition of the first term.

Second Term
: This is one definition of the second term. 
: This is another definition of the second term.

### Quotes

> Lorem ipsum is a pseudo-Latin text used in web design, typography, layout, and printing in place of English to emphasise design elements over content. It's also called placeholder (or filler) text.

### Underscore

This is an <ins>underscored block of</ins> text.

### Bullet Lists

* Item 1
* Item 2

**Labeled** Lists

* **Label-1**: items 1
* **Lable-2**: items 2

* Unordered list can use asterisks
- Or minuses
+ Or pluses


### Numbered list

Some _Numbers_

1. First ordered list item
2. Another item
  * Unordered sub-list. 
1. Actual numbers don't matter, just that it's a number
  1. Ordered sub-list
4. And another item.

   You can have properly indented paragraphs within list items. Notice the blank line above, and the leading spaces (at least one, but we'll use three here to also align the raw Markdown).

   To have a line break without a paragraph, you will need to use two trailing spaces.  
   Note that this line is separate, but within the same paragraph.  
   (This is contrary to the typical GFM line break behaviour, where trailing spaces are not required.)

### Images

{{< image src="common/sample.svg" alt="Sample Image" justify="center" width="560" type="scaled-90">}}

## Header H2
Lorem ipsum is a pseudo-Latin text used in web design, typography, layout, and printing in place of English to emphasise design elements over content. It's also called placeholder (or filler) text. It's a convenient tool for mock-ups. 


### Header H3 `with` Stuff
Lorem ipsum is a `pseudo-Latin` text used in web design, typography, layout, and printing in place of English to emphasise design elements over content. It's also called placeholder (or filler) text. It's a convenient tool for mock-ups. 

#### Header h4
Lorem ipsum is a pseudo-Latin text used in web design, typography, layout, and printing in place of English to emphasise design elements over content. It's also called placeholder (or filler) text. It's a convenient tool for mock-ups. 

## Header H2
Lorem ipsum is a pseudo-Latin text used in web design, typography, layout, and printing in place of English to emphasise design elements over content. It's also called placeholder (or filler) text. It's a convenient tool for mock-ups. 

##### Header h5
Lorem ipsum is a pseudo-Latin text used in web design, typography, layout, and printing in place of English to emphasise design elements over content. It's also called placeholder (or filler) text. It's a convenient tool for mock-ups. 

###### Header h6
Lorem ipsum is a pseudo-Latin text used in web design, typography, layout, and printing ...


#### Next Steps
---------------
* [Getting Started](...)


#### Related Topics
-------------------
* [Compare with Other Software](...)