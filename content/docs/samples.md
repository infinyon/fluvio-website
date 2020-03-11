---
title: __Samples (fuvio elements)__
weight: 100000
hidden: true
---

Welcome to the intro guide to Fluvio! The Concepts section helps you learn about all parts of the Fluvio system and to gain deeper understanding of how Fluvio works.

## Header H2
Lorem ipsum is a pseudo-Latin text used in web design, typography, layout, and printing in place of English to emphasise design elements over content. It's also called placeholder (or filler) text. It's a convenient tool for mock-ups. 

Some **Lists**

* Item 1
* Item 2

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

* Unordered list can use asterisks
- Or minuses
+ Or pluses

### Header H3
Lorem ipsum is a pseudo-Latin text used in web design, typography, layout, and printing in place of English to emphasise design elements over content. It's also called placeholder (or filler) text. It's a convenient tool for mock-ups. 

#### Header h4
Lorem ipsum is a pseudo-Latin text used in web design, typography, layout, and printing in place of English to emphasise design elements over content. It's also called placeholder (or filler) text. It's a convenient tool for mock-ups. 

## Header H2
Lorem ipsum is a pseudo-Latin text used in web design, typography, layout, and printing in place of English to emphasise design elements over content. It's also called placeholder (or filler) text. It's a convenient tool for mock-ups. 

##### Header h5
Lorem ipsum is a pseudo-Latin text used in web design, typography, layout, and printing in place of English to emphasise design elements over content. It's also called placeholder (or filler) text. It's a convenient tool for mock-ups. 

###### Header h6
Lorem ipsum is a pseudo-Latin text used in web design, typography, layout, and printing ...


</br>
{{< idea >}}
Lorem ipsum is a pseudo-Latin text used in web design, typography, layout, and printing in place of English to emphasise design elements over content. It's also called placeholder (or filler) text. It's a convenient tool for mock-ups. It helps to outline the visual elements of a document or presentation, eg typography, font, or layout. Lorem ipsum is mostly a part of a Latin text by the classical author and philosopher Cicero. Its words and letters have been changed by addition or removal, so to deliberately render its content nonsensical; it's not genuine, correct, or comprehensible Latin anymore. While lorem ipsum's still resembles classical Latin, it actually has no meaning whatsoever. As Cicero's text doesn't contain the letters K, W, or Z, alien to latin, these, and others are often inserted randomly to mimic the typographic appearence of European languages, as are digraphs not to be found in the original.
{{< /idea >}}

{{< idea >}}
Lorem ipsum is a pseudo-Latin text used in web design
{{< /idea >}}

{{< caution >}}
Lorem ipsum is a pseudo-Latin text used in web design, typography, layout, and printing in place of English to emphasise design elements over content. It’s also called placeholder (or filler) text. It’s a convenient tool for mock-ups. 
{{< /caution >}}

{{< caution >}}
Lorem ipsum is a pseudo-Latin text used in web design
{{< /caution >}}


{{< text >}}
Lorem ipsum is a pseudo-Latin text used in web design, typography, layout, and printing in place of English to emphasise design elements over content. It’s also called placeholder (or filler) text. It’s a convenient tool for mock-ups. 
{{< /text >}}

{{< text >}}
Lorem ipsum is a pseudo-Latin text used in web design
{{< /text >}}

{{< cli yaml >}}
$ fluvio auth-token list "test" -o 1
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
{{< /cli>}}

{{< cli yaml>}}
$ fluvio auth-token list "test" -o 1
error: cannot retrieve auth topics: Connection refused (os error 61) 
{{< /cli>}}

{{< cli yaml>}}
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
{{< /cli>}}

{{< code rust >}}
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
{{< /code>}}

{{< cli >}}
$ consul config read -kind service-defaults -name web
{
   "Kind": "service-defaults",
   "Name": "web",
   "Protocol": "http"
}
{{< /cli>}}


{{< cli >}}
$ test
apiVersion: "rbac.istio.io/v1alpha1"
kind: ServiceRole
metadata:
    name: mongodb-viewer
    namespace: default
spec:
    rules:
    - services: ["mongodb.default.svc.cluster.local"]
    constraints:
    - key: "destination.port"
        values: ["27017"]
{{< /cli>}}

{{< cli json >}}
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
{{< /cli>}}

{{< links "Next Steps" >}}
* [Getting Started](...)

{{< links "Related Items" >}}
* [Compare Fluvio with Compare with Other Software]({{< relref "fluvio-vs-others/overview" >}})
{{< /links >}}
