```TypeScript
package fluvio.java.app;
import com.infinyon.fluvio.Fluvio;
import com.infinyon.fluvio.TopicProducer;

public class App {
    public static void main(String[] args) throws Exception {
        Fluvio fluvio = Fluvio.connect();
        TopicProducer producer = fluvio.topic_producer("hello-java");
        
        producer.send(String.valueOf("1").getBytes(), ("Hello", "Fluvio!").getBytes());
    }
}
```