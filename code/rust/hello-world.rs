use fluvio::FluvioError;
use fluvio::Offset;
use async_std::stream::StreamExt;

#[async_std::main]
async fn main() {
    // Collect our arguments into a slice of &str
    let args: Vec<String> = std::env::args().collect();
    let args_slice: Vec<&str> = args.iter().map(|s| &**s).collect();

    let result = match &*args_slice {
        [_, "produce"] => {
            produce("Hello", "Fluvio!").await
        },
        [_, "consume"] => {
            consume().await
        },
        _ => {
            println!("Usage: hello-fluvio [produce|consume]");
            return;
        },
    };

    if let Err(err) = result {
        println!("Got error: {}", err);
    }
}

async fn produce(key: &str, value: &str) -> Result<(), FluvioError> {
    let producer = fluvio::producer("hello-fluvio").await?;
    producer.send(key, value).await?;
    producer.flush().await?;
    Ok(())
}

async fn consume() -> Result<(), FluvioError> {
    let consumer = fluvio::consumer("hello-fluvio", 0).await?;
    let mut stream = consumer.stream(Offset::beginning()).await?;

    // Iterate over all events in the topic
    while let Some(Ok(record)) = stream.next().await {
        let key_bytes = record.key().unwrap();
        let key = String::from_utf8_lossy(key_bytes).to_string();
        let value = String::from_utf8_lossy(record.value()).to_string();
        println!("Consumed record: Key={}, value={}", key, value);
    }
    Ok(())
}