---
title: Producer Behavior
weight: 80
---

The client should managed send messages with a strong strategy, specially when you  want to send multiple messages at the same time (batch) or on the same connection, without waiting for the ack for each message before sending the next one
If I send a batch, and the client says there is an error, does it means all messages failed, or some may have been wrote in the queue on the server ? If so, how does the producer react N
?
