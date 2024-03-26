use actix_cors::Cors;
use actix_web::web::Json;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::fs::File;
use web_push::IsahcWebPushClient;
use web_push::*;

#[macro_use]
extern crate fstrings;

#[derive(Serialize, Deserialize)]
struct PushNotificationRequest {
    subscription_id: String,
    data: PushNotificationData,
}

#[derive(Serialize, Deserialize)]
struct PushNotificationData {
    title: Option<String>,
    message: String,
    icon: Option<String>,
    badge: Option<String>,
    timestamp: Option<i64>, // Unix timestamp
    data: PushNotificationRedirect,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PushNotificationRedirect {
    redirect_url: String,
}

#[derive(Serialize, Deserialize)]
struct SubscriptionRequest {
    subscription_id: String,
    info: SubscriptionInfo,
}

fn subscription_path(subscription_id: &str) -> String {
    f!("subscriptions/{subscription_id}.json")
}

fn save_subscription_info(subscription_id: &str, subscription_info: SubscriptionInfo) {
    // Save subscription info json file called subscription_info.json
    let file = File::create(subscription_path(subscription_id)).unwrap();
    serde_json::to_writer(file, &subscription_info).unwrap();
}

fn load_subscription_info(subscription_id: &str) -> Result<SubscriptionInfo, std::io::Error> {
    // Load subscription info json file called subscription_info.json
    let file = File::open(subscription_path(subscription_id))?;
    let subscription_info: SubscriptionInfo = serde_json::from_reader(file).unwrap();
    Ok(subscription_info)
}

#[post("/push-message")]
// send message, takes in a string
async fn push_message(Json(pnr): web::Json<PushNotificationRequest>) -> impl Responder {
    // Load subscription info
    let subscription_info = match load_subscription_info(&pnr.subscription_id) {
        Ok(subscription_info) => subscription_info,
        Err(_) => {
            return "Subscription not found";
        }
    };

    // Read signing material for payload.
    let file = File::open("keys/private_key.pem").unwrap();
    tokio::spawn(async move {
        let sig_builder = VapidSignatureBuilder::from_pem(file, &subscription_info)
            .unwrap()
            .build()
            .unwrap();

        // Now add payload and encrypt.
        let mut builder = WebPushMessageBuilder::new(&subscription_info);
        let content = serde_json::to_string(&pnr).unwrap();
        let content = content.as_bytes();

        builder.set_payload(ContentEncoding::Aes128Gcm, content);
        builder.set_ttl(10 * 60); // 10 minutes
        builder.set_vapid_signature(sig_builder);

        let client = IsahcWebPushClient::new().unwrap();

        // wait until timestamp to send the notification
        if let Some(timestamp) = &pnr.data.timestamp {
            let current_time = chrono::Utc::now().timestamp_millis();
            let time_to_wait_in_millis = timestamp - current_time;
            if time_to_wait_in_millis > 0 {
                println!("Waiting for {} milliseconds", time_to_wait_in_millis);
                tokio::time::sleep(tokio::time::Duration::from_millis(
                    time_to_wait_in_millis as u64,
                ))
                .await;
            }
        }

        let web_push_message = builder.build().unwrap();
        // Finally, send the notification!
        client.send(web_push_message).await.unwrap();
    });

    "Message request successfully"
}

#[get("/public-key")]
async fn get_public_key() -> impl Responder {
    let public_key = std::fs::read_to_string("keys/public_key.txt").unwrap();
    let public_key = public_key.strip_suffix('\n').unwrap();
    public_key.to_string()
}

#[post("/subscribe")]
async fn subscribe(Json(subscription): web::Json<SubscriptionRequest>) -> impl Responder {
    println!("Subscription info: {:?}", subscription.info);
    save_subscription_info(&subscription.subscription_id, subscription.info);
    "Subscribed successfully"
}

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().finish()
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let address = "0.0.0.0";
    let port = 1236;

    println!("Server running on http://{}:{}", address, port);
    HttpServer::new(|| {
        // let cors = Cors::permissive();
        let cors = Cors::default()
            .allow_any_header()
            .allow_any_method()
            .allow_any_origin();

        App::new()
            .wrap(cors)
            .service(health)
            .service(subscribe)
            .service(get_public_key)
            .service(push_message)
    })
    .bind((address, port))?
    .run()
    .await
}
