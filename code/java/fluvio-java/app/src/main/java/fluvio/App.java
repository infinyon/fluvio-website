package fluvio;

import com.infinyon.fluvio.Fluvio;
import com.infinyon.fluvio.TopicProducer;
import com.infinyon.fluvio.PartitionConsumer;
import com.infinyon.fluvio.PartitionConsumerStream;
import com.infinyon.fluvio.Offset;
import com.infinyon.fluvio.Record;
import java.text.MessageFormat;
import java.util.*;

/**
 * This is an example of a basic Fluvio workflow in Java 
 *
 * 1. Create a topic to store data in via CLI
 * 2. Establish a connection to the Fluvio cluster
 * 3. Create a producer and send some bytes
 * 4. Create a consumer, and stream the data back
 */
public class App {
    public static final String TOPIC_NAME = "hello-java";
    public static final int PARTITION = 0;
    public static final int NUM_MESSAGES = 1;

    public static void main(String[] args) throws Exception {
        // Currently the Java client does not support creating topics
        // Using the fluvio CLI
        final String cmd = String.format("fluvio topic create %s", TOPIC_NAME);
        try {
            Process process = Runtime.getRuntime().exec(cmd);
        } catch (Exception e) {
            e.printStackTrace(System.err);
        }

        // Connect to cluster
        Fluvio fluvio = Fluvio.connect();

        // Produce to topic
        TopicProducer producer = fluvio.producer(TOPIC_NAME);

        for (int i = 0; i < NUM_MESSAGES; i++) {
            producer.send(String.valueOf(i).getBytes(), MessageFormat.format("Hello World! - Time is {0,time}", new Date()).getBytes());
        }

        // Consume from topic
        PartitionConsumer consumer = fluvio.consumer(TOPIC_NAME, PARTITION);
        PartitionConsumerStream stream = consumer.stream(Offset.from_end(NUM_MESSAGES - 1));
        for (int i = 0; i < NUM_MESSAGES; i++) {
            Record record = stream.next();
            System.out.printf("Consumed record, key=%s, value=%s\n", record.key_string(), record.value_string());
        }
    }
}
