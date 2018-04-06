#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

use rocket::http::RawStr;
use rocket::response::content;

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[get("/hello/<name>")]
fn helloname(name: &RawStr) -> String {
    format!("Hello, {}!", name.as_str())
}


#[get("/vntm/<name>")]
fn vntm(name: &RawStr) -> content::Html<String> {
    content::Html(format!("<!DOCTYPE html><html><body><h1 style=\"text-decoration: blink\">Va bien niquer ta mère {} !!!</h1></body></html>", name.as_str()))
    }

fn main() {
    rocket::ignite().mount("/", routes![hello, helloname, vntm]).launch();
} 