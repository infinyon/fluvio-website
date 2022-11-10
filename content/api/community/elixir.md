---
title: Elixir 
weight: 15
---

Support for [Elixir](https://elixir-lang.org/) is provided through [fluvio-ex](https://github.com/viniarck/fluvio-ex), a community supported project by [@aviniarck](https://github.com/viniarck).


## Create a topic

This will create a topic `lobby` with 1 partition and 1 replica.

```elixir
alias Fluvio.Admin

{:ok, pid} = Admin.start_link()
{:ok, _} = Admin.create_topic(pid, "lobby", %{partitions: 1, replication: 1})
```

[Link to original source](https://github.com/viniarck/fluvio-ex/blob/master/examples/admin.exs)

## Producer

In this example, a Producer for the topic `lobby` is created. Then the message `hello` is sent to the topic 20 times.

```elixir
alias Fluvio.Producer

{:ok, pid} = Producer.start_link(%{topic: "lobby"})

{:ok, _} = Producer.send(pid, "hello")
{:ok, _} = Producer.flush(pid)

[] =
  1..20
  |> Stream.chunk_every(10)
  |> Stream.flat_map(fn chunk ->
    [
      chunk
      |> Enum.map(fn value ->
        Task.async(fn -> {Producer.send(pid, to_string(value)), value} end)
      end)
      |> Task.await_many()
      |> Enum.filter(&match?({{:error, _msg}, _value}, &1)),
      [{Producer.flush(pid), :flush}]
      |> Enum.filter(&match?({{:error, _msg}, _value}, &1))
    ]
  end)
  |> Stream.concat()
  |> Enum.to_list()
  ```

[Link to original source](https://github.com/viniarck/fluvio-ex/blob/master/examples/producer.exs)

## Consumer

In this example, a Consumer for the topic `lobby` is created, starting from offset `0`. As records are received, the record contents are printed with `IO.inspect`.

```elixir
alias Fluvio.Consumer

{:ok, pid} = Consumer.start_link(%{topic: "lobby", offset: [from_beginning: 0]})

Consumer.stream_each(pid, fn result ->
  case result do
    {:ok, record} -> IO.inspect(record)
    {:error, msg} -> IO.inspect("Error: #{msg}")
  end
end)
```

[Link to original source](https://github.com/viniarck/fluvio-ex/blob/master/examples/consumer.exs)