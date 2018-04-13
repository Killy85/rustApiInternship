#![feature(plugin, decl_macro, custom_derive)]
#![plugin(rocket_codegen)]

extern crate postgres;
extern crate serde_json;
extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use postgres::{Connection, TlsMode};
use rocket::http::RawStr;
use rocket::response::content;
use serde_json::Error;
use rocket_contrib::{Json, Value};
use rocket::State;
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Serialize, Deserialize)]
struct User {
    name: String,
    firstname: String,
    mail: String,
    pswd: String
}
#[derive(Serialize, Deserialize)]
struct ConnectionApp {
    mail: String,
    pswd: String
}

#[derive(Serialize, Deserialize)]
struct Connected {
    name: String,
    firstname: String
}

struct Internship {
    id: i32,
    title: String
}

struct Enterprise {
    id: i32,
    nom: String,
    adresse: String
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


#[post("/auth/signin",format = "application/json", data = "<input>")]
fn signin(input: Json<User>) -> content::Html<&'static str> {
    println!("That's a test");
    content::Html("oui oui bien")
    }


#[post("/auth/auth",format = "application/json", data = "<input>")]
fn authenticate(input: Json<ConnectionApp>) -> content::Html<String> {
    let conn = Connection::connect("postgres://killy:test123@localhost:5432/rustDb",
                               TlsMode::None).unwrap();
    let message : String;
    let result = conn.query(
    r#"
        SELECT name, firstname
        FROM users
        WHERE mail = $1
        AND password = $2
    "#,
    &[&input.mail, &input.pswd]).unwrap();

    if !result.is_empty() && result.len() == 1 {
        let user = result.get(0);
        let userConn = Connected{
            name: user.get(0),
            firstname: user.get(1),
        };
        message ="Tu passes!".to_string();
    }else {
        message = "Tu passes pas !".to_string();
    }
    
    content::Html(format!("{}", message))
    }

#[get("/test-db")]
fn test_db() -> &'static str {
    let conn = Connection::connect("postgres://wjdpqrrq:kxYU23ThjIOSmtVqi6lX4BpSUdQXMG7e@horton.elephantsql.com:5432/wjdpqrrq",
                               TlsMode::None).unwrap();
    for row in &conn.query("SELECT id, title FROM internship", &[]).unwrap() {
        let person = Internship {
            id: row.get(0),
            title: row.get(1),
        };
        println!("Found person {}  {}",person.id, person.title);
    }
    "oui"
    }


    #[get("/test-db2")]
fn db_enterprise() -> &'static str {
    let conn = Connection::connect("postgres://wjdpqrrq:kxYU23ThjIOSmtVqi6lX4BpSUdQXMG7e@horton.elephantsql.com:5432/wjdpqrrq",
                               TlsMode::None).unwrap();
    for row in &conn.query("SELECT id, nom, adresse FROM entreprise", &[]).unwrap() {
        let enterprise = Enterprise {
            id: row.get(0),
            nom: row.get(1),
            adresse: row.get(2),
        };
        println!("Found enterprise {}  {}  {}",enterprise.id, enterprise.nom, enterprise.adresse);
    }
    "oui"
    }

    
#[get("/poney")]
fn poney() -> content::Html<&'static str> {
    content::Html("<!DOCTYPE html><html><body><h1 style=\"text-decoration: blink\">je suis un poney</h1></body></html>")
    }

fn main() {
    rocket::ignite().mount("/", routes![hello, helloname, vntm, poney, test_db, db_enterprise, signin, authenticate]).launch();
} 