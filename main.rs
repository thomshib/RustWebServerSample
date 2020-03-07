#![feature(plugin)]
#![feature(proc_macro_hygiene, decl_macro)]

extern crate  serde;
extern crate chrono;

extern crate rocket_contrib;
extern crate rocket_codegen;

#[macro_use]
extern crate serde_derive;

#[macro_use] extern crate rocket;

use chrono::prelude;
use chrono::{DateTime, TimeZone, NaiveDateTime, Utc};

use rocket_contrib::json::Json;


#[derive(Serialize)]
struct Timestamp{
    time: String,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/time")]
fn time_now() -> Json<Timestamp>{
    let now: DateTime<Utc> = Utc::now();
    let timestamp_value = Timestamp{time: now.to_rfc3339()};
    Json(timestamp_value)

}

fn main(){
 rocket::ignite()
    .mount("/", routes![index, time_now])
    .launch();

}