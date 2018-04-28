#![feature(plugin, decl_macro, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket_cors;
extern crate postgres;
extern crate serde_json;
extern crate rocket;
extern crate chrono;
extern crate yyid;

#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use postgres::{Connection, TlsMode};
use rocket::response::content;
use rocket_contrib::{Json};
use std::collections::LinkedList;
use chrono::prelude::*;
use chrono::{Duration, Utc,NaiveDate, NaiveDateTime};
use rocket::request::{Request,FromRequest};
use rocket::request;
use rocket::Outcome;
use rocket::http::Status;
use yyid::yyid_string;



static Y_DELTA : f32 = 0.3541;
static X_DELTA : f32 = 1.014;


#[derive(Serialize, Deserialize)]
struct User {
    name: String,
    firstname: String,
    mail: String,
    pswd: String
}

#[derive(Serialize, Deserialize)]
struct TokenReturn {
    value : String,
    date : String 
}

#[derive(Serialize, Deserialize)]
struct Token(String);

#[derive(Serialize, Deserialize)]
struct SearchStruct {
    tags: LinkedList<String>,
    contrats: LinkedList<String>

}

#[derive(Serialize, Deserialize)]
struct Contract {
    id_contract: i32,
    name: String
}

#[derive(Serialize, Deserialize)]
struct Company {
    id_company: i32,
    name: String,
    adress: String,
    longitude: f32,
    latitude: f32,
    mail_hr: String,
    website_company: String,
    country : String,
    city: String,
    zip_code: i32
}

#[derive(Serialize, Deserialize)]
struct CreateInternship {
    id_internship: i32,
    name: String,
    id_user: i32,
    start_date: NaiveDate,
    end_date: NaiveDate,
    degree: String,
    description: String,
    type_of_contract : String,
    pros: String,
    cons: String
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
struct TagsComplete {
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

fn is_valid(key: &str) -> bool {
    let conn = Connection::connect("postgres://killy:rustycode44@54.38.244.17:5432/rustDb",
                               TlsMode::None).unwrap();

    let result = conn.query(r#"SELECT date from token where value = $1"#, &[&key]).unwrap();
    if result.len() == 1 {
            let now = Utc::now();
            let date : String= result.get(0).get(0);
            let token_date = date.parse::<DateTime<Utc>>().unwrap();
            let diff = token_date - now;
            
            if diff > Duration::days(7) {
                false
            } else {
                let result = conn.query("UPDATE token set date = $1 where value = $2", &[&now.to_string(), &key]);
                result.is_ok()
            }
    }else {
            false
    }

}

impl<'a, 'r> FromRequest<'a, 'r> for Token {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Token,()> {
        
        let keys: Vec<_> = request.headers().get("Authorization").collect();
        if keys.len() != 1 {
            return Outcome::Failure((Status::new(401, "Your auth token is missing from the request headers"), ()));
        }

        let key = keys[0];
        if !is_valid(keys[0]) {
            return Outcome::Failure((Status::new(401, "Your auth token is invalid"), ()));
        }

        return Outcome::Success(Token(key.to_string()));
    }
}

#[get("/")]
fn hello() -> &'static str {
    "Welcome to HORO API"
}

#[post("/refresh_token",format = "application/json", data = "<input>")]
fn refresh_token(input: Json<ConnectionApp>) -> content::Json<String> {
    let conn = Connection::connect("postgres://killy:rustycode44@54.38.244.17:5432/rustDb",
                               TlsMode::None).unwrap();
    let dt = Utc::now();
    let result = conn.query(
    r#"
        SELECT value, date from token natural join users    
        WHERE mail = $1
        AND password = $2
    "#,
    &[&input.mail, &input.pswd]).unwrap();
     if result.len() == 1 {
         let token_value = result.get(0);
         let token = TokenReturn{
             value : token_value.get(0),
             date : token_value.get(1)
         };
            let result = conn.query("UPDATE token set date = $1 where value = $2", &[&dt.to_string(), &token.value]);
                content::Json(json!({"status" : 200, "user_token" : token.value}).to_string())
        }else { 
            let result = conn.query("Select id_user from users where mail = $1", &[&input.mail]);
            let id : i32 = result.unwrap().get(0).get(0);
            let token_str = yyid_string();
            let result = conn.query("INSERT into token (value, date, id_user) VALUES ($1,$2,$3)",&[&token_str,&Utc::now().to_string(),&id]);
            content::Json(json!({"status" : 200, "message" : "Token Created", "token" : token_str}).to_string())
        }
}


#[post("/signin",format = "application/json", data = "<input>")]
fn signin(input: Json<User>) -> content::Json<String> {
    let conn = Connection::connect("postgres://killy:rustycode44@54.38.244.17:5432/rustDb",
                               TlsMode::None).unwrap();

    let result = conn.query(
    r#"
        INSERT INTO users (name, firstname, mail, role, password)
        VALUES ($1,$2,$3,$4,$5)
    "#,
    &[&input.name, &input.firstname,&input.mail,&"student",&input.pswd]);
     if result.is_ok() {
            let result = conn.query("Select id_user from users where mail = $1", &[&input.mail]);
            let id : i32 = result.unwrap().get(0).get(0);
            let token_str = yyid_string();
            let result = conn.query("INSERT into token (value, date, id_user) VALUES ($1,$2,$3)",&[&token_str,&Utc::now().to_string(),&id]);
            content::Json(json!({"status" : 200, "message" : "User created", "token" : token_str}).to_string())
    }else{
            content::Json(json!({"status" : 404,"message" : "An error occured while creating the user"}).to_string())
    }
    }


#[post("/login",format = "application/json", data = "<input>")]
fn authenticate(input: Json<ConnectionApp>) -> content::Json<String> {
    let conn = Connection::connect("postgres://killy:rustycode44@localhost:5432/rustDb",
                               TlsMode::None).unwrap();
    
    let result = conn.query(
    r#"
        SELECT id_user,name, firstname
        FROM users
        WHERE mail = $1
        AND password = $2
    "#,
    &[&input.mail, &input.pswd]).unwrap();

        if !result.is_empty() && result.len() == 1 {
            let user = result.get(0);
            let user_conn = Connected{
                name: user.get(1),
                firstname: user.get(2),
            }; 
            let id : i32 = user.get(0);
            let result = conn.query(
                r#"
                    SELECT value from token     
                    WHERE id_user = $1
                "#,
            &[&id]).unwrap();
            let token_str : String = result.get(0).get(0);
            content::Json(json!({"status" : 200, "user" : user_conn, "token" : token_str}).to_string())
        }else { 
            content::Json(json!({"status" : 400, "user" : " "}).to_string())
        }
    }


#[post("/create_company",format = "application/json", data = "<input>")]
fn create_company(input: Json<Company>) -> content::Json<String> {
    let conn = Connection::connect("postgres://killy:rustycode44@localhost:5432/rustDb",
                               TlsMode::None).unwrap();

    let result = conn.query(
    r#"
        INSERT INTO company (id_company, name, adress, longitude, latitude, mail_hr, website_company, country, city, zip_code)
        VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10)
    "#,
    &[&input.id_company, &input.name, &input.adress, &input.longitude, &input.latitude, &input.mail_hr, &input.website_company, &input.country, &input.city, &input.zip_code]);
     if result.is_ok() {
            content::Json(json!({"status" : 200, "message" : "Company created"}).to_string())
    }else{
            content::Json(json!({"status" : 404,"message" : "An error occured while creating the company"}).to_string())
    }
}

#[post("/create_internship",format = "application/json", data = "<input>")]
fn create_internship(token :Token,input: Json<CreateInternship>) -> content::Json<String> {
    let conn = Connection::connect("postgres://killy:rustycode44@localhost:5432/rustDb",
                               TlsMode::None).unwrap();

    let result = conn.query(
    r#"
        INSERT INTO company (id_internship, name, id_user, start_date, end_date, degree, description, type_of_contract, pros, cons)
        VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10)
    "#,
    &[&input.id_internship, &input.name, &input.id_user, &input.start_date, &input.end_date, &input.degree, &input.description, &input.type_of_contract, &input.pros, &input.cons]);
     if result.is_ok() {
            content::Json(json!({"status" : 200, "message" : "Internship created"}).to_string())
    }else{
            content::Json(json!({"status" : 404,"message" : "An error occured while creating the internship"}).to_string())
    }
}

#[get("/test-db")]
fn test_db() -> &'static str {
    let conn = Connection::connect("postgres://killy:rustycode44@localhost:5432/rustDb",
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
fn init(token : Token) -> content::Json<String>{
    
    let conn = Connection::connect("postgres://killy:rustycode44@localhost:5432/rustDb",TlsMode::None).unwrap();
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
fn tags(token :Token ) -> content::Json<String>{
    
    let conn = Connection::connect("postgres://killy:rustycode44@localhost:5432/rustDb",TlsMode::None).unwrap();
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

#[get("/tags_autocomplete/<str>")]
fn tags_autocomplete(str : String) -> content::Json<String>{

    let conn = Connection::connect("postgres://killy:rustycode44@localhost:5432/rustDb",TlsMode::None).unwrap();
    let mut list: LinkedList<TagsComplete> = LinkedList::new(); 
	
    for row in &conn.query(&format!("SELECT id_tag, name FROM tag WHERE name LIKE '%{}%' ",str ),&[]).unwrap() 
	{
        let tags = TagsComplete {
            id_tag: row.get(0),
            name: row.get(1)        
        };
        list.push_back(tags);
    }   
    content::Json(json!({"tags" : list}).to_string())
}

#[get("/contract")]
fn contract() -> content::Json<String>{
    
    let conn = Connection::connect("postgres://killy:rustycode44@localhost:5432/rustDb",TlsMode::None).unwrap();
    let mut list: LinkedList<Contract> = LinkedList::new(); 

    for row in &conn.query("SELECT id_contract, name FROM contract", &[]).unwrap() {
        let contract = Contract {
            id_contract: row.get(0),
            name: row.get(1)        
        };
        list.push_back(contract);
    }   
    content::Json(json!({"contract" : list}).to_string())
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
fn init_post(token : Token, input : Json<Position>) -> content::Json<String>{
    
    let conn = Connection::connect("postgres://killy:rustycode44@localhost:5432/rustDb",TlsMode::None).unwrap();
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


#[post("/search_ets", format="application/json", data="<input>")]
fn search_ets(token : Token,input : Json<SearchStruct>) -> content::Json<String>{
    let mut contrats : String = "".to_string();
    let mut tags : String = "".to_string();
    let mut query : String = "".to_string();
    let conn = Connection::connect("postgres://killy:rustycode44@54.38.244.17:5432/rustDb",TlsMode::None).unwrap();
    let mut list: LinkedList<EnterpriseInit> = LinkedList::new(); 
    if input.contrats.len() >0 && input.tags.len() >0 {
        let first = true;
       /* for elem in input.contrats{
            
        }*/

        for elem in input.tags.iter(){
            tags = tags + elem + "','"
        }
        tags.pop();
        tags.pop();
        tags.pop();

        let result = conn.query(&format!("SELECT Distinct company.id_company, company.name, company.latitude, company.longitude  FROM company 
                INNER JOIN has_been_made_in on (company.id_company = has_been_made_in.id_company) 
                INNER JOIN internship on (internship.id_internship = has_been_made_in.id_internship) 
                INNER JOIN has_tag on (internship.id_internship = has_tag.id_internship)
                INNER JOIN tag on (tag.id_tag = has_tag.id_tag)
                 WHERE tag.name in ('{}')", tags),&[]);
        for row in result.unwrap().iter(){
            let enterprise = EnterpriseInit {
            id: row.get(0),
            name: row.get(1),
            longitude : row.get(2),
            latitude : row.get(3)
        };
        list.push_back(enterprise);
        }
        content::Json(json!({"points" : list}).to_string())
    }else{

        content::Json(json!({"points" :list}).to_string())
    }
    }

fn main() {
    let default = rocket_cors::Cors::default();
    rocket::ignite().attach(default).mount("/", routes![hello, signin, authenticate,init, init_post, tags, tags_autocomplete, create_company, /*create_internship,*/ contract,search_ets,refresh_token]).launch();
} 
