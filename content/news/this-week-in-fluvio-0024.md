---
title: "This Week in Fluvio #24"
date: 2022-03-03
weight: 20
---
Welcome to This Week in Fluvio, our weekly newsletter
for development updates to [Fluvio open source]. Fluvio is a distributed,
programmable streaming platform written in Rust.

{{< banner >}}

## Data Retention

If you are producing a lot of data into Fluvio, you might be interested in keeping that data fresh for your consumers. Older data can be automatically pruned.

The default retention on new topics is `7 days` with a default segment size of `1 GB`. So any data residing in older 1 GB segments will be pruned `7 days` after the last write. The current segment is left alone.

### Example data lifecycle

For a given topic with a retention of `7 days` using `1 GB` segments

* Day 0: 2.5 GB is written (total topic data: 2.5 GB)

| Topic Segment # | Segment size | Days since last write |
|-----------------|--------------|-----------------------|
| 0               | 1 GB         | 0                     |
| 1               | 1 GB         | 0                     |
| 2               | 0.5 GB       | N/A                   |

* Day 6: Another 2 GB is written (total topic data: 4.5 GB,)

| Topic Segment # | Segment size | Days since last write |
|-----------------|--------------|-----------------------|
| 0               | 1 GB         | 6                     |
| 1               | 1 GB         | 6                     |
| 2               | 1 GB         | 0                     |
| 3               | 1 GB         | 0                     |
| 4               | 0.5 GB       | N/A                   |

* Day 7: 2 segments from Day 0 are 7 days old. They are pruned (total topic data: 2.5 GB)

| Topic Segment # | Segment size | Days since last write |
|-----------------|--------------|-----------------------|
| 2               | 1 GB         | 1                     |
| 3               | 1 GB         | 1                     |
| 4               | 0.5 GB       | N/A                   |

* Day 14: 2 segments from Day 7 are 7 days old. They are pruned (total topic data: 0.5 GB)

| Topic Segment # | Segment size | Days since last write |
|-----------------|--------------|-----------------------|
| 4               | 0.5 GB       | N/A                   |

The newest segment is left alone and only begins to age once a new segment is being written to.


For more detail check out the [docs for more about data retention in Fluvio]({{<ref "/docs/operations/retention">}})

---

Get in touch with us on [Github Discussions] or join [our Discord channel] and come say hello!

For the full list of changes this week, be sure to check out [our CHANGELOG].

Until next week!

[Fluvio open source]: https://github.com/infinyon/fluvio
[our CHANGELOG]: https://github.com/infinyon/fluvio/blob/master/CHANGELOG.md
[our Discord channel]: https://discordapp.com/invite/bBG2dTz
[Github Discussions]: https://github.com/infinyon/fluvio/discussions