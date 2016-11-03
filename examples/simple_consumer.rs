extern crate rdkafka;

use rdkafka::config::KafkaConfig;
use rdkafka::util::get_rdkafka_version;
use rdkafka::consumer::CreateConsumer;

fn consume_and_print(topic: &str) {
    let mut consumer = KafkaConfig::new()
        .set("group.id", "marmellata")
        .set("metadata.request.timeout.ms", "20000")
        .create_consumer()
        .expect("Consumer creation failed");

    consumer.broker_add("localhost:9092");

    consumer.subscribe(topic).expect("Can't subscribe to topic");

    println!("Consumer initialized");

    loop {
        match consumer.poll(1000) {
            Ok(None) => {}
            Ok(Some(m)) => {
                println!("M: {:?} {:?} {:?} {:?}", m.get_payload(), m.get_key(), m.get_partition(), m.get_offset());
                if String::from_utf8_lossy(m.get_payload().unwrap()) == "QUIT" {
                    break;
                }
            }
            Err(e) => println!("E: {:?}", e),
        }
    }
    println!("END LOOP");
}

fn main() {
    let (version_n, version_s) = get_rdkafka_version();
    println!("rd_kafka_version: 0x{:08x}, {}", version_n, version_s);

    consume_and_print("topic1");
}