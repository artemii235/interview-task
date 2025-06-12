use anyhow::Result;
use redis::aio::MultiplexedConnection;
use redis::AsyncCommands;
use redis::Client;
use tokio::sync::mpsc::Receiver;
use uuid::Uuid;

const RECENT_TOPICS_KEY: &str = "recent_topics";

#[derive(Clone)]
pub struct Redis {
    client: Client,
    connection: MultiplexedConnection,
}

impl Redis {
    pub async fn new(uri: &str) -> Result<Self> {
        let client = Client::open(uri)?;
        let mut connection = client.get_multiplexed_async_connection().await?;
        redis::cmd("PING")
            .query_async::<()>(&mut connection)
            .await?;

        Ok(Self { client, connection })
    }

    async fn add_topic(&self, topic_id: Uuid) -> Result<()> {
        let mut redis_conn = self.connection.clone();

        let _: (i32, i32, ()) = redis::pipe()
            .lrem(RECENT_TOPICS_KEY, 0, topic_id.as_bytes())
            .lpush(RECENT_TOPICS_KEY, topic_id.as_bytes())
            .ltrim(RECENT_TOPICS_KEY, 0, 9)
            .query_async(&mut redis_conn)
            .await?;

        Ok(())
    }

    pub async fn get_recent_topics(&self) -> Result<Vec<Uuid>> {
        let mut redis_conn = self.connection.clone();
        let topics: Vec<Vec<u8>> = redis_conn.lrange(RECENT_TOPICS_KEY, 0, 9).await?;
        Ok(topics
            .into_iter()
            .map(|bytes| Uuid::from_slice(&bytes))
            .collect::<Result<Vec<_>, _>>()?)
    }
}

pub async fn process_topics(redis: Redis, mut rx: Receiver<Uuid>) {
    while let Some(topic) = rx.recv().await {
        if let Err(e) = redis.add_topic(topic).await {
            log::error!(
                "Error adding topic to Redis: {}. Need alert here and/or retry logic",
                e
            );
        }
    }
}
