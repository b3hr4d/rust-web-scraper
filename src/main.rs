mod models;

use models::{ Body, ChatCompletion, Message };

use std::vec;

use actix_web::{ post, App, web, HttpServer, HttpResponse };

#[post("/chat")]
async fn chat_with_gpt(body: web::Json<Message>) -> HttpResponse {
    let url = "https://chatgpt-api.shn.hk/v1/";

    let model = "gpt-3.5-turbo".to_string();

    let content = body.content.to_string();
    let role = body.role.to_string();

    let messages = vec![Message {
        role,
        content,
    }];

    let body = Body { model, messages };

    let client = reqwest::Client::new();

    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&body).unwrap())
        .send().await
        .unwrap()
        .text().await
        .unwrap();

    print!("{}", response);

    let gpt_response: ChatCompletion = serde_json::from_str(&response).unwrap();

    let choice = &gpt_response.choices[0];

    HttpResponse::Ok().body(format!("{}", choice.message.content))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| { App::new().service(chat_with_gpt) })
        .bind(("127.0.0.1", 8080))?
        .run().await
}