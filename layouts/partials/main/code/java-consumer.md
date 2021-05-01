```TypeScript
package fluvio.java.app;
import com.infinyon.fluvio.Fluvio;
import com.infinyon.fluvio.PartitionConsumer;
import com.infinyon.fluvio.PartitionConsumerStream;
import com.infinyon.fluvio.Offset;
import com.infinyon.fluvio.Record;

public class App {
    public static void main(String[] args) throws Exception {
        Fluvio fluvio = Fluvio.connect();
        PartitionConsumer consumer = fluvio.partition_consumer("hello-java", 0);

        PartitionConsumerStream stream = consumer.stream(Offset.beginning());
        Record record = stream.next();
            System.out.printf("Consumed record, key=%s, value=%s\n", record.key_string(), record.value_string());
    }
}

```