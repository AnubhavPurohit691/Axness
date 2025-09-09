
use kafka::consumer::{self, Consumer};




pub fn connect_to_kafka(topic:String) -> Result<Consumer, kafka::error::Error> {
    Consumer::from_hosts(vec!["localhost:9092".to_string()])
        .with_topic_partitions(topic, &[0])
        .with_fallback_offset(consumer::FetchOffset::Latest)
        .with_group("my_grp".to_string())
        .with_offset_storage(Some(consumer::GroupOffsetStorage::Kafka))
        .create()
}