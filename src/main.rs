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
use serde_json::Value;
use rocket_contrib::{Json, Value as OtherValue};
use std::collections::LinkedList;

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
    id: i32
}

#[derive(Serialize, Deserialize)]
struct EnterpriseInit {
    id: i32,
    name: String,
    longitude: f32,
    latitude: f32
}

#[get("/")]
fn hello() -> &'static str {
    "Welcome to HORO API"
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
    let conn = Connection::connect("postgres://killy:test123@10.44.2.8:5432/rustDb",
                               TlsMode::None).unwrap();
    for row in &conn.query("SELECT id_internship FROM internship", &[]).unwrap() {
        let person = Internship {
            id: row.get(0)
        };
        println!("Found person {}",person.id);
    }
    "oui"
}       

    #[get("/init")]
fn init() -> content::Json<String>{
    
    let conn = Connection::connect("postgres://killy:test123@10.44.2.8:5432/rustDb",TlsMode::None).unwrap();
    let mut list: LinkedList<EnterpriseInit> = LinkedList::new(); 

    for row in &conn.query("SELECT id_company, name, longitude, latitude FROM company", &[]).unwrap() {
        let enterprise = EnterpriseInit {
            id: row.get(0),
            name: row.get(1),
            longitude : row.get(2),
            latitude : row.get(3)
        };
        list.push_back(enterprise);
    }   
    content::Json(json!({"points" : list}).to_string())
}

fn main() {
    rocket::ignite().mount("/", routes![hello,test_db, signin, authenticate,init]).launch();
} 