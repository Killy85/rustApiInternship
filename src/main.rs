#![feature(plugin)]
#![plugin(rocket_codegen)]


extern crate rocket;
extern crate postgres;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;


use postgres::{Connection, TlsMode};
use rocket::http::RawStr;
use rocket::response::content;


#[derive(Serialize, Deserialize, Debug, Clone)]
struct Internship {
    id: i32,
    title: String
}

#[derive(Serialize, Deserialize, Debug)]
struct Ets{
    name: &'static str, 
    internships: Vec<Internship>
}

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

#[get("/test-db")]
fn test_db() -> content::Json<String> {
    let conn = Connection::connect("postgres://wjdpqrrq:kxYU23ThjIOSmtVqi6lX4BpSUdQXMG7e@horton.elephantsql.com:5432/wjdpqrrq",
                               TlsMode::None).unwrap();
    let mut entreprise = Ets{ name : "Welcome to CapGe", internships : Vec::new()};
    for row in &conn.query("SELECT id, title FROM internship", &[]).unwrap() {
        let person = Internship {
            id: row.get(0),
            title: row.get(1),
        };
        println!("Found person {}  {}",person.id, person.title);
        entreprise.internships.push(person);
    }
    content::Json(serde_json::to_string(&entreprise).unwrap())
    }
    
#[get("/poney")]
fn poney() -> content::Html<&'static str> {
    content::Html("<!DOCTYPE html><html><body><h1 style=\"text-decoration: blink\">je suis un poney</h1></body></html>")
    }

fn main() {
    rocket::ignite().mount("/", routes![hello, helloname, vntm, poney, test_db]).launch();
} 