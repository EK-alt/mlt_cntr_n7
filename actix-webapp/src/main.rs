use std::env::{self, set_var};

use actix_cors::Cors;
use actix_service::Service;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use chrono::Utc;
mod views;

// use std::env;

use gel_tokio::{self, Client};
use std::fs;

async fn calc() {
    let res1 = 2 + 1;
    println!("res1 = {}", res1);
    let res2 = 2 + 2;
    println!("res2 = {}", res2);
    let res3 = 2 + 3;
    println!("res3 = {}", res3);
    let res4 = 2 + 4;
    println!("res4 = {}", res4);
}

fn calc1() {
    let res1 = 2 + 1;
    println!("res1 = {}", res1);
    let res2 = 2 + 2;
    println!("res2 = {}", res2);
    let res3 = 2 + 3;
    println!("res3 = {}", res3);
    let res4 = 2 + 4;
    println!("res4 = {}", res4);
}

// #[actix_web::main]
// async fn main() -> anyhow::Result<()> {
//     let current_dir = env::current_dir().expect("Failed to get current directory");
//     let cert_path = current_dir.join("server-cert.pem");
//     // Convert the PathBuf to a String.  Handle the error if needed.
//     let cert_path_str = cert_path
//         .to_str()
//         .expect("Failed to convert path to string");
//     // let now_utc = Utc::now();
//     env::set_var("EDGEDB_HOST", "edgedb_rust");
//     // env::set_var("EDGEDB_PORT", "5657");
//     env::set_var("EDGEDB_SERVER_PORT", "5657");
//     env::set_var("EDGEDB_USER", "edgedb");
//     env::set_var("EDGEDB_PASSWORD", "edgedb");
//     env::set_var("EDGEDB_DATABASE", "edgedb");
//     env::set_var("EDGEDB_TLS_CA_FILE", cert_path_str);

//     let tls_ca_file = env::var("EDGEDB_TLS_CA_FILE").unwrap();
//     println!("EDGEDB_TLS_CA_FILE: {}", tls_ca_file);
//     println!("EDGEDB_TLS_CA_FILE: {}", cert_path_str);

//     let pem_content = fs::read_to_string(cert_path)?;
//     println!("pem_content {}", pem_content);

//     let conn = gel_tokio::create_client().await?;
//     // let conn = gel_tokio::create_client(&gel_tokio::Options {
//     //     host: "localhost".to_string(), // The Docker host IP (NOT localhost)
//     //     port: 5657,                    // The port mapped on the host machine
//     //     user: "edgedb",
//     //     password: "edgedb",
//     //     database: "edgedb",
//     //     ..Default::default() // Important: Use Default::default() to avoid setting tls
//     // })
//     // .await
//     // .expect("Client should have initiated");
//     let val = conn
//         .query_required_single::<i64, _>("SELECT 7*8", &())
//         .await?;

//     println!("7*8 is: {}", val);
//     Ok(())
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let cert_path = current_dir.join("server-cert.pem");
    // Convert the PathBuf to a String.  Handle the error if needed.
    let cert_path_str = cert_path
        .to_str()
        .expect("Failed to convert path to string");
// let tls_ca_file = env::var("EDGEDB_TLS_CA_FILE").unwrap();
    // println!("EDGEDB_TLS_CA_FILE: {}", tls_ca_file);
    println!("EDGEDB_TLS_CA_FILE: {}", cert_path_str);

    let pem_content = std::fs::read_to_string(cert_path)?;
    println!("pem_content {}", pem_content);

    let args: Vec<String> = env::args().collect();
    let now_utc = Utc::now();
    println!("Current time in UTC #: {}", now_utc);

    /* Test */
    calc().await;
    calc1();
    let cors = Cors::default()
        .allow_any_origin()
        .allow_any_method()
        .allow_any_header();

    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        let app = App::new()
            .wrap_fn(|req, srv| {
                println!("{:?}, {}-{}", req, req.method(), req.uri());

                let future = srv.call(req);

                async {
                    let result = future.await?;
                    Ok(result)
                }
            })
            .configure(views::views_factory)
            .wrap(cors);

        return app;
    })
    .bind("0.0.0.0:9093")?
    .run()
    .await /* ?; */
    // }
    // Ok(())
}
