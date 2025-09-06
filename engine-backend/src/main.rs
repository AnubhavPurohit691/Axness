use std::time::Duration;

use kafka::consumer::{self, Consumer};

fn main() {
    let mut consumer = match connect_to_kafka() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("failed to connect with kafka{}", e);
            return;
        }
    };
    loop{
        for ms in consumer.poll().unwrap().iter(){
            for m in ms.messages(){
                match std::str::from_utf8(m.value){
                    Ok(value)=>println!("{}",value),
                    Err(e)=>eprintln!("failed to parse{}",e)
                }
            }
            consumer.consume_messageset(ms).unwrap()
        }
        consumer.commit_consumed().unwrap();
        std::thread::sleep(Duration::from_millis(100));
    }
    
}

fn connect_to_kafka() -> Result<Consumer, kafka::error::Error> {
    Consumer::from_hosts(vec!["localhost:9092".to_string()])
        .with_topic_partitions("axness".to_owned(), &[0])
        .with_fallback_offset(consumer::FetchOffset::Earliest)
        .with_group("my_grp".to_string())
        .with_offset_storage(Some(consumer::GroupOffsetStorage::Kafka))
        .create()
}