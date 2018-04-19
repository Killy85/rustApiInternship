#![feature(plugin, decl_macro, custom_derive)]
#![plugin(rocket_codegen)]

extern crate postgres;
extern crate serde_json;
extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use postgres::{Connection, TlsMode};
use rocket::response::content;
use rocket_contrib::{Json};
use std::collections::LinkedList;

static X_DELTA : f32 = 0.3541;
static Y_DELTA : f32 = 1.014;

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
struct Tagsinit {
    id_tag: i32,
    name: String
}


#[derive(Serialize, Deserialize)]
struct EnterpriseInit {
    id: i32,
    name: String,
    longitude: f32,
    latitude: f32
}

#[derive(Serialize, Deserialize)]
struct Position{
    center_lat : f32,
    center_long : f32,
    zoom_level : i16
}

#[get("/")]
fn hello() -> &'static str {
    "Welcome to HORO API"
}


#[post("/auth/signin",format = "application/json", data = "<input>")]
fn signin(input: Json<User>) -> content::Json<String> {
    let conn = Connection::connect("postgres://killy:test123@localhost:5432/rustDb",
                               TlsMode::None).unwrap();

    let result = conn.query(
    r#"
        INSERT INTO users (name, firstname, mail, role, password)
        VALUES ($1,$2,$3,$4,$5)
    "#,
    &[&input.name, &input.firstname,&input.mail,&"student",&input.pswd]);
     if result.is_ok() {
            content::Json(json!({"status" : 200, "message" : "User created"}).to_string())
    }else{
            content::Json(json!({"status" : 404,"message" : "An error occured while creating the user"}).to_string())
    }
    }


#[post("/auth/auth",format = "application/json", data = "<input>")]
fn authenticate(input: Json<ConnectionApp>) -> content::Json<String> {
    let conn = Connection::connect("postgres://killy:test123@localhost:5432/rustDb",
                               TlsMode::None).unwrap();
    
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
            let user_conn = Connected{
                name: user.get(0),
                firstname: user.get(1),
            }; 
            content::Json(json!({"status" : "200", "user" : user_conn}).to_string())
        }else { 
            content::Json(json!({"status" : "400", "user" : " "}).to_string())
        }
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

#[get("/tags")]
fn tags() -> content::Json<String>{
    
    let conn = Connection::connect("postgres://killy:test123@10.44.2.8:5432/rustDb",TlsMode::None).unwrap();
    let mut list: LinkedList<Tagsinit> = LinkedList::new(); 

    for row in &conn.query("SELECT id_tag, name FROM tag", &[]).unwrap() {
        let tags = Tagsinit {
            id_tag: row.get(0),
            name: row.get(1)        
        };
        list.push_back(tags);
    }   
    content::Json(json!({"tags" : list}).to_string())
}


fn scale_float_add(input : f32, zoom_level : i16, is_lat : bool) -> f32 {
    if is_lat{
        input + (Y_DELTA * (zoom_level as f32/10.0))
    }else{
        input + (X_DELTA * (zoom_level as f32/10.0))
    }
}

fn scale_float_sup(input : f32, zoom_level : i16, is_lat : bool) -> f32 {
    if is_lat{
        input - (Y_DELTA * (zoom_level as f32/10.0))
    }else{
        input - (X_DELTA * (zoom_level as f32/10.0))
    }
}

#[post("/init", format="application/json", data="<input>")]
fn init_post(input : Json<Position>) -> content::Json<String>{
    
    let conn = Connection::connect("postgres://killy:test123@10.44.2.8:5432/rustDb",TlsMode::None).unwrap();
    let mut list: LinkedList<EnterpriseInit> = LinkedList::new(); 

    for row in &conn.query("SELECT id_company, name, longitude, latitude 
                FROM company WHERE (latitude > $1 AND latitude < $2) 
                AND (longitude > $3 AND longitude < $4)",
          &[&scale_float_sup(input.center_lat, input.zoom_level, true),
          &scale_float_add(input.center_lat, input.zoom_level, true),
          &scale_float_sup(input.center_long, input.zoom_level, false),
          &scale_float_add(input.center_long, input.zoom_level, false)
          ])
          .unwrap() {
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
    rocket::ignite().mount("/", routes![hello,test_db, signin, authenticate,init, init_post, tags]).launch();
} 