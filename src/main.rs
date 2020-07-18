#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate lazy_static;
extern crate rocket_cors;
use std::sync::{Arc, Mutex}; //using for global veriable
use std::collections::HashMap;
use rocket_contrib::json::{Json, JsonValue};
use rocket::http::Method;
use rocket_cors::{
    AllowedHeaders, AllowedOrigins, Error,
    Cors, CorsOptions,
};

use rocket::State;

type ID = usize;        // defining global veriable
#[derive(Debug, PartialEq, Eq, Deserialize)]
struct Message{
    id: ID,
    contents: String
}

fn make_cors() -> Cors {
    let allowed_origins = AllowedOrigins::some_exact(&[
        // "http://best-ball.surge.sh",
        "http://192.168.1.104:8080/rocket/"
    ]);
    CorsOptions{
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post].into_iter()
        .map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&[ // it brings metadata
            "Authorization",
            "Accept",
            "Access-Control-Allow-Origin",
        ]),
        allow_credentials: true,
        ..Default::default() 
    }
    .to_cors()
    .expect("Error while building th cors")
}

#[get("/")]
fn hello() -> JsonValue {
    json!([
        {
            "id": "01",
            "name": "Aqib"
        },
        {
            "id": "02",
            "name": "Ammar"
        },
        {
            "id": "03",
            "name": "Zahid"
        }
    ])
}

type MessageMap = Mutex<HashMap<ID, String>>;

#[post("/add", data="<user_input>")] //post has route
fn helloPost(user_input: Json<Message>, map: State<'_, MessageMap>) { // '_ its mean lifetime
    println!("{:?}", user_input.0.contents);  //will come in tuple format
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
    .mount("/", routes![hello, helloPost]).attach(make_cors())
    .manage(Mutex::new(HashMap::<ID, String>::new()))
}

fn main() {
    rocket().launch();
}