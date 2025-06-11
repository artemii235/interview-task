mod comment;
mod envvars;
mod mongo;
mod redis;

use actix_web::middleware::Logger;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use anyhow::Result;
use chrono::Utc;
use tokio::sync::mpsc::{channel, Sender};
use uuid::Uuid;

use crate::comment::{Comment, CommentRequest};
use crate::mongo::Mongo;
use crate::redis::{process_topics, Redis};

#[derive(serde::Serialize)]
#[serde(tag = "status", content = "data")]
pub enum HandlerResponse<T> {
    Success(T),
    Fail { code: u32, message: String },
}

#[post("/comments")]
async fn post_comment(
    data: web::Data<AppState>,
    body: web::Json<CommentRequest>,
) -> impl Responder {
    let request = body.into_inner();

    log::info!(
        "New comment from {} in topic {}",
        request.sender,
        request.topic_id
    );

    let comment = Comment {
        topic_id: request.topic_id,
        sender: request.sender,
        text: request.text,
        timestamp: Utc::now(),
    };

    match data.mongo.add_comment(comment).await {
        Ok(comment) => {
            if let Err(e) = data.topics_tx.try_send(comment.topic_id) {
                log::error!("Failed to send topic ID to topics channel: {}. The state is inconsistent. Retry should be implemented", e);
            }

            let result = HandlerResponse::Success(comment);
            HttpResponse::Created().json(result)
        }
        Err(e) => {
            log::error!("Failed to add comment: {}", e);
            let result = HandlerResponse::<()>::Fail {
                code: 1,
                message: "Failed to add comment".to_string(),
            };
            HttpResponse::InternalServerError().json(result)
        }
    }
}

/// Get comments for a topic
#[get("/comments/{topic_id}")]
async fn get_comments(data: web::Data<AppState>, path: web::Path<Uuid>) -> impl Responder {
    let topic_id = path.into_inner();

    match data.mongo.get_comments_by_topic(&topic_id).await {
        Ok(comments) => {
            let result = HandlerResponse::Success(comments);
            HttpResponse::Ok().json(result)
        }
        Err(e) => {
            log::error!("Failed to get comments: {}", e);
            let result = HandlerResponse::<()>::Fail {
                code: 2,
                message: "Failed to get comments".to_string(),
            };
            HttpResponse::InternalServerError().json(result)
        }
    }
}

/// Get recently updated topics
#[get("/topics/recent")]
async fn get_recent_topics(data: web::Data<AppState>) -> impl Responder {
    match data.redis.get_recent_topics().await {
        Ok(topics) => {
            let result = HandlerResponse::Success(topics);
            HttpResponse::Ok().json(result)
        }
        Err(e) => {
            log::error!("Failed to get recent topics: {}", e);
            let result = HandlerResponse::<()>::Fail {
                code: 3,
                message: "Failed to get recent topics".to_string(),
            };
            HttpResponse::InternalServerError().json(result)
        }
    }
}

/// Application state
#[derive(Clone)]
struct AppState {
    mongo: Mongo,
    redis: Redis,
    topics_tx: Sender<Uuid>,
}

#[actix_web::main]
async fn main() -> Result<()> {
    env_logger::init();

    log::info!("Initializing Mongo connection...");
    let mongo = Mongo::new(&envvars::MONGO_URL).await?;
    log::info!("Mongo connection initialized successfully");

    log::info!("Initializing Redis connection...");
    let redis = Redis::new(&envvars::REDIS_URL).await?;
    log::info!("Redis connection initialized successfully");

    let (topics_tx, topics_rx) = channel(100);

    tokio::spawn({
        let redis = redis.clone();
        process_topics(redis, topics_rx)
    });

    let app_state = AppState {
        mongo,
        redis,
        topics_tx,
    };

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(app_state.clone()))
            .service(post_comment)
            .service(get_comments)
            .service(get_recent_topics)
    })
    .bind(&*envvars::HOST)?
    .run()
    .await
    .map_err(anyhow::Error::from)
}
