use crate::comment::Comment;
use anyhow::Result;
use futures::stream::TryStreamExt;
use mongodb::{options::ClientOptions, Client as MongoClient, Collection};
use uuid::Uuid;

#[derive(Clone)]
pub struct Mongo {
    mongo_client: MongoClient,
}

impl Mongo {
    pub async fn new(uri: &str) -> Result<Self> {
        let mongo_options = ClientOptions::parse(uri).await?;
        let mongo_client = MongoClient::with_options(mongo_options)?;

        mongo_client
            .database("comments_db")
            .run_command(mongodb::bson::doc! {"ping": 1})
            .await?;
        log::info!("Connected to MongoDB");

        let mongo = Self { mongo_client };
        mongo.ensure_indexes().await?;

        Ok(mongo)
    }

    async fn ensure_indexes(&self) -> Result<()> {
        let collection = self.comments_collection();

        let index = mongodb::IndexModel::builder()
            .keys(mongodb::bson::doc! {
                "topic_id": 1,
                "timestamp": 1
            })
            .build();

        collection.create_index(index).await?;
        log::info!("Created compound index on topic_id and timestamp");

        Ok(())
    }

    fn comments_collection(&self) -> Collection<Comment> {
        self.mongo_client
            .database("comments_db")
            .collection("comments")
    }

    pub async fn add_comment(&self, comment: Comment) -> Result<Comment> {
        let collection = self.comments_collection();
        collection.insert_one(&comment).await?;

        Ok(comment)
    }

    pub async fn get_comments_by_topic(&self, topic_id: &Uuid) -> Result<Vec<Comment>> {
        let collection = self.comments_collection();
        let filter = mongodb::bson::doc! { "topic_id": mongodb::bson::Binary {
                   subtype: mongodb::bson::spec::BinarySubtype::Generic,
                   bytes: topic_id.as_bytes().to_vec(),
               }
        };

        let comments: Vec<Comment> = collection
            .find(filter)
            .sort(mongodb::bson::doc! { "timestamp": 1 })
            .await?
            .try_collect()
            .await?;

        Ok(comments)
    }
}
