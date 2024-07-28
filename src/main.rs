use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct User {
    id: u32,
    name: String,
    email: String,
}

#[get("/users")]
async fn get_users() -> impl Responder {
    // Simulated user data
    let users = vec![
        User {
            id: 1,
            name: "Alice".to_string(),
            email: "alice@example.com".to_string(),
        },
        User {
            id: 2,
            name: "Bob".to_string(),
            email: "bob@example.com".to_string(),
        },
    ];

    HttpResponse::Ok().json(users)
}

#[post("/users")]
async fn create_user(user: web::Json<User>) -> impl Responder {
    // Here you would typically save the user to a database
    println!("Creating user: {:?}", user);

    HttpResponse::Created().json(user.into_inner())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_users)
            .service(create_user)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}