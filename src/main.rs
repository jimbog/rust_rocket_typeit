#[macro_use] extern crate rocket;
use rocket::Config;
use rocket::figment::Figment;
use enigo::{Enigo, KeyboardControllable};

#[get("/typeit?<text>")]
fn typeit(text: &str) -> String {
    let mut enigo = Enigo::new();

    println!("{}", text);
    enigo.key_sequence_parse(text);
    format!("Received and printed text: {}", text)
}
#[get("/test")]
fn test() -> String {
    println!("TESTING");
    format!("Testing request received")
}

#[rocket::main]
async fn main() {
    let figment = Figment::from(Config::default())
        .merge(("address", "0.0.0.0"))
        .merge(("port", 9000));

    let result = rocket::custom(figment)
        .mount("/", routes![typeit, test])
        .launch()
        .await;

    if let Err(e) = result {
        eprintln!("Rocket failed to launch: {}", e);
    }
}
