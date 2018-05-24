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
use chrono::{Utc, NaiveDate};
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
    contrats: LinkedList<i32>,
    pos : Position
}

#[derive(Serialize, Deserialize)]
struct SearchStructIntern {
    company: i32,
    tags: LinkedList<String>,
    contrats: LinkedList<i32>
}

#[derive(Serialize, Deserialize)]
struct Contract {
    id_contract: i32,
    name: String
}
#[derive(Serialize, Deserialize)]
struct CompanyList {
    id_company: i32,
    name: String
}

#[derive(Serialize, Deserialize)]
struct Company {
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
    name: String,
    id_user: i32,
    start_date: String,
    end_date: String,
    degree: String,
    description: String,
    type_of_contrat : i32,
    pros: String,
    cons: String,
    id_company: i32,
    tags: LinkedList<i32>
}

#[derive(Serialize, Deserialize)]
struct InternshipDisplay {
    id_internship : i32,
    internship_name : String,
    start_date : chrono::NaiveDate,
    end_date : chrono::NaiveDate, 
    degree : String, 
    description : String, 
    pros: String, 
    cons: String,
    contrat_name : String, 
    users_name: String, 
    users_firstname: String, 
    users_mail: String
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
struct EnterpriseDisplay {
    id: i32,
    name: String,
    adress: String,
    longitude: f32,
    latitude: f32,
    mail_hr: String,
    website_company: String,
    country : String,
    city: String,
    zip_code: i32,
    internship :LinkedList<InternshipDisplay>
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

#[get("/ets/<id>")]
fn company_display(_token : Token, id : i32)-> content::Json<String>{
    let mut ets : LinkedList<EnterpriseDisplay> = LinkedList::new();
    let query = &format!("SELECT * FROM company WHERE id_company = {}", id);
    let conn = Connection::connect("postgres://killy:rustycode44@54.38.244.17:5432/rustDb",TlsMode::None).unwrap();
    let result = conn.query(query, &[]).unwrap().len();
    if result > 0 {
        for row in &conn.query(query, &[]).unwrap(){
            let _id_c : i32 = row.get(0);
        let mut list: LinkedList<InternshipDisplay> = LinkedList::new();
            for row_inter in &conn.query("SELECT id_internship, internship.name, start_date, end_date, 
                                degree, description, pros, cons,contrat.name, users.name, 
                                users.firstname, users.mail
                                FROM internship
                                INNER JOIN contrat on (type_of_contrat = id_contrat)
                                INNER JOIN users on (internship.id_user = users.id_user)
                                NATURAL JOIN has_been_made_in
                                WHERE id_company = $1 ", &[&id]).unwrap(){
                                    let internship = InternshipDisplay{
                                        id_internship : row_inter.get(0),
                                        internship_name : row_inter.get(1),
                                        start_date : row_inter.get(2),
                                        end_date : row_inter.get(3), 
                                        degree : row_inter.get(4), 
                                        description : row_inter.get(5), 
                                        pros: row_inter.get(6), 
                                        cons: row_inter.get(7),
                                        contrat_name : row_inter.get(8), 
                                        users_name: row_inter.get(9), 
                                        users_firstname: row_inter.get(10), 
                                        users_mail: row_inter.get(11)
                                    };
                                    list.push_back(internship);
                                }    
                let ets_itm = EnterpriseDisplay{
                    id: row.get(0),
                    name: row.get(1),
                    adress: row.get(2),
                    longitude: row.get(3),
                    latitude: row.get(4),
                    mail_hr: row.get(5),
                    website_company: row.get(6),
                    city: row.get(7),
                    country: row.get(8),
                    zip_code: row.get(9),
                    internship :list
                    };
                    ets.push_back(ets_itm)
                
            } 
            let mut iter = ets.iter();
            content::Json(json!({"Company" : iter.next()}).to_string())
    }else {
            content::Json(json!({"Company" : format!("No Company with id {}",id)}).to_string())
    }
}

#[get("/tags")]
fn tags(_token :Token ) -> content::Json<String>{
    
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

#[get("/contract")]
fn contract(_token: Token) -> content::Json<String>{
    
    let conn = Connection::connect("postgres://killy:rustycode44@localhost:5432/rustDb",TlsMode::None).unwrap();
    let mut list: LinkedList<Contract> = LinkedList::new(); 

    for row in &conn.query("SELECT id_contrat, name FROM contrat", &[]).unwrap() {
        let contract = Contract {
            id_contract: row.get(0),
            name: row.get(1)        
        };
        list.push_back(contract);
    }   
    content::Json(json!({"contract" : list}).to_string())
}

#[get("/company")]
fn company(_token : Token) -> content::Json<String>{
    
    let conn = Connection::connect("postgres://killy:rustycode44@localhost:5432/rustDb",TlsMode::None).unwrap();
    let mut list: LinkedList<CompanyList> = LinkedList::new(); 

    for row in &conn.query("SELECT id_company, name FROM company", &[]).unwrap() {
        let company = CompanyList {
            id_company: row.get(0),
            name: row.get(1)        
        };
        list.push_back(company);
    }   
    content::Json(json!({"company" : list}).to_string())
}

#[get("/tags")]
fn tags(_token :Token ) -> content::Json<String>{
    
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

#[get("/contract")]
fn contract(_token: Token) -> content::Json<String>{
    
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

#[get("/company")]
fn company(_token : Token) -> content::Json<String>{
    
    let conn = Connection::connect("postgres://killy:rustycode44@localhost:5432/rustDb",TlsMode::None).unwrap();
    let mut list: LinkedList<CompanyList> = LinkedList::new(); 

    for row in &conn.query("SELECT id_company, name FROM company", &[]).unwrap() {
        let company = CompanyList {
            id_company: row.get(0),
            name: row.get(1)        
        };
        list.push_back(company);
    }   
    content::Json(json!({"company" : list}).to_string())
}

#[get("/ets/<id>")]
fn company_display(_token : Token, id : i32)-> content::Json<String>{
    print!("Bonjour");
    let mut ets : LinkedList<EnterpriseDisplay> = LinkedList::new();
    let query = &format!("SELECT * FROM company WHERE id_company = {}", id);
    print!("{}", query);
    let conn = Connection::connect("postgres://killy:rustycode44@54.38.244.17:5432/rustDb",TlsMode::None).unwrap();
    let result = conn.query(query, &[]).unwrap().len();
    if result > 0 {
        for row in &conn.query(query, &[]).unwrap(){
            let _id_c : i32 = row.get(0);
        let mut list: LinkedList<InternshipDisplay> = LinkedList::new();
            for row_inter in &conn.query("SELECT id_internship, internship.name, start_date, end_date, 
                                degree, description, pros, cons,contrat.name, users.name, 
                                users.firstname, users.mail
                                FROM internship
                                INNER JOIN contrat on (type_of_contrat = id_contrat)
                                INNER JOIN users on (internship.id_user = users.id_user)
                                NATURAL JOIN has_been_made_in
                                WHERE id_company = $1 ", &[&id]).unwrap(){
                                    let internship = InternshipDisplay{
                                        id_internship : row_inter.get(0),
                                        internship_name : row_inter.get(1),
                                        start_date : row_inter.get(2),
                                        end_date : row_inter.get(3), 
                                        degree : row_inter.get(4), 
                                        description : row_inter.get(5), 
                                        pros: row_inter.get(6), 
                                        cons: row_inter.get(7),
                                        contrat_name : row_inter.get(8), 
                                        users_name: row_inter.get(9), 
                                        users_firstname: row_inter.get(10), 
                                        users_mail: row_inter.get(11)
                                    };
                                    list.push_back(internship);
                                }    
                let ets_itm = EnterpriseDisplay{
                    id: row.get(0),
                    name: row.get(1),
                    adress: row.get(2),
                    longitude: row.get(3),
                    latitude: row.get(4),
                    mail_hr: row.get(5),
                    website_company: row.get(6),
                    city: row.get(7),
                    country: row.get(8),
                    zip_code: row.get(9),
                    internship :list
                    };
                    ets.push_back(ets_itm)
                
            } 
            let mut iter = ets.iter();
            content::Json(json!({"Company" : iter.next()}).to_string())
    }else {
            content::Json(json!({"Company" : format!("No Company with id {}",id)}).to_string())
    }
}

#[post("/refresh_token",format = "application/json", data = "<input>")]
fn refresh_token(input: Json<ConnectionApp>) -> content::Json<String> {
    let conn = Connection::connect("postgres://killy:rustycode44@localhost:5432/rustDb",
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
            let _result = conn.query("UPDATE token set date = $1 where value = $2", &[&dt.to_string(), &token.value]);
                content::Json(json!({"status" : 200, "user_token" : token.value}).to_string())
        }else { 
            let result = conn.query("Select id_user from users where mail = $1", &[&input.mail]);
            let id : i32 = result.unwrap().get(0).get(0);
            let token_str = yyid_string();
            let _result = conn.query("INSERT into token (value, date, id_user) VALUES ($1,$2,$3)",&[&token_str,&Utc::now().to_string(),&id]);
            content::Json(json!({"status" : 200, "message" : "Token Created", "token" : token_str}).to_string())
        }
}


#[post("/signin",format = "application/json", data = "<input>")]
fn signin(input: Json<User>) -> content::Json<String> {
    let conn = Connection::connect("postgres://killy:rustycode44@localhost:5432/rustDb",
            TlsMode::None).unwrap();

    let result = conn.query(
    r#"
        INSERT INTO users (name, firstname, mail, role, password)
        VALUES ($1,$2,$3,$4,$5)
    "#,
    &[&input.name, &input.firstname,&input.mail,&"student",&input.pswd]);
    if result.is_ok() {
            let result = conn.query("Select id_user, firstname, name from users where mail = $1", &[&input.mail]);
            let user = result.unwrap();
            let id : i32 = user.get(0).get(0);
            let _firstname : String = user.get(0).get(1);
            let _name : String = user.get(0).get(2);
            let token_str = yyid_string();

            let _result = conn.query("INSERT into token (value, date, id_user) VALUES ($1,$2,$3)",&[&token_str,&Utc::now().to_string(),&id]);
            let user_con = Connected{
                name: user.get(0).get(2),
                firstname: user.get(0).get(1),
            };
            content::Json(json!({"status" : 200, "message" : "User created", "token" : token_str, "user" : user_con}).to_string())
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
            content::Json(json!({"status" : 200, "user" : user_conn, "token" : token_str, "message" : format!("Bienvenue {} !", user_conn.name)}).to_string())
        }else { 
            content::Json(json!({"status" : 400, "user" : " ", "message" : "E-mail/Password mismatch"}).to_string())
        }
    }


#[post("/create_company",format = "application/json", data = "<input>")]
fn create_company(input: Json<Company>) -> content::Json<String> {
    let conn = Connection::connect("postgres://killy:rustycode44@localhost:5432/rustDb",
            TlsMode::None).unwrap();

    let result = conn.query(
    r#"
        INSERT INTO company (name, adress, longitude, latitude, mail_hr, website_company, country, city, zip_code)
        VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9)
    "#,
    &[&input.name, &input.adress, &input.longitude, &input.latitude, &input.mail_hr, &input.website_company, &input.country, &input.city, &input.zip_code]);
    if result.is_ok() {
            content::Json(json!({"status" : 200, "message" : "Company created"}).to_string())
    }else{
            content::Json(json!({"status" : 404,"message" : "An error occured while creating the company"}).to_string())
    }
}

#[post("/create_internship",format = "application/json", data = "<input>")]
fn create_internship(_token :Token,input: Json<CreateInternship>) -> content::Json<String> {
    let conn = Connection::connect("postgres://killy:rustycode44@localhost:5432/rustDb",
            TlsMode::None).unwrap();
    
    let start_date = date_converter(input.start_date.clone());
    let end_date = date_converter(input.end_date.clone());

    let result = conn.query(
    r#"
        INSERT INTO internship (name, id_user, start_date, end_date, degree, description, type_of_contrat, pros, cons)
        VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9)
    "#,
    &[&input.name, &input.id_user, &start_date, &end_date, &input.degree, &input.description, &input.type_of_contrat, &input.pros, &input.cons]);
    if result.is_ok() {
            let result = conn.query(r#"SELECT id_internship from internship WHERE name = $1 and id_user=$2"#,&[&input.name, &input.id_user]);
            let id_internship : i32 = result.unwrap().get(0).get(0);
            let comp_query = conn.query("INSERT INTO has_been_made_in (id_company,id_intership) VALUES ($1,$2)", &[&input.id_company,&id_internship]);
            if comp_query.is_ok() {
                for tag in &input.tags{
                    let _tags_res = conn.query("INSERT INTO has_tags (id_internship, id_tag) VALUES ($1,$2)", &[&id_internship,&tag]);
                }
                content::Json(json!({"status" : 200, "message" : "Internship created"}).to_string())
            }
            else {
                println!("{}",comp_query.unwrap_err());
                content::Json(json!({"status" : 404,"message" : "An error occured while creating the internship"}).to_string())
            }
            
    }else{
            println!("{}",result.unwrap_err());
            content::Json(json!({"status" : 404,"message" : "An error occured while creating the internship"}).to_string())
    }

}

#[post("/search_internships", format="application/json", data="<input>")]
fn search_internships(_token : Token,input : Json<SearchStructIntern>) -> content::Json<String>{
    let mut contrats : String = "".to_string();
    let _tags : String;
    let mut internship : String = "".to_string();
    let mut resulting =false;
    let conn = Connection::connect("postgres://killy:rustycode44@54.38.244.17:5432/rustDb",TlsMode::None).unwrap();
    let mut list: LinkedList<InternshipDisplay> = LinkedList::new(); 
    let mut result = conn.query("SELECT DISTINCT id_internship from internship", &[]);
    if input.tags.len() >0 {
        let mut in_tags = "".to_string();
        for elem in input.tags.iter(){
            in_tags = in_tags + &format!("'{}',",elem.to_string());
        }
        in_tags.pop();
            result = conn.query(&format!("SELECT DISTINCT id_internship from tag 
                    INNER JOIN has_tag on (tag.id_tag = has_tag.id_tag)
                    WHERE tag.name in ({})", in_tags), &[]);
    }
            
    for row in result.unwrap().iter(){
        resulting = true;
        let tmp : i32 = row.get(0);
        internship = internship + &format!("{},", tmp);
    }
        if resulting{
            internship.pop();

            if input.contrats.len() > 0 {
                for elem in input.contrats.iter(){
                    contrats = contrats + &format!("'{}',", elem)
                }
                contrats.pop();
                for row in conn.query(&format!("SELECT Distinct internship.id_internship, internship.name,users.name, users.firstname, users.mail,internship.start_date,
                internship.end_date,internship.degree,internship.description,internship.pros, internship.cons, contrat.name FROM company 
                INNER JOIN has_been_made_in on (company.id_company = has_been_made_in.id_company) 
                INNER JOIN internship on (internship.id_internship = has_been_made_in.id_internship)
                INNER JOIN users on (users.id_user = internship.id_user)
                INNER JOIN contrat on (internship.type_of_contrat=contrat.id_contrat)
                WHERE internship.id_internship in ({})
                AND internship.type_of_contrat in ({})
                AND company.id_company = {}", internship, contrats, input.company),&[]).unwrap().iter(){
                    let internship = InternshipDisplay {
                    id_internship : row.get(0),
                    internship_name : row.get(1),
                    users_name: row.get(2), 
                    users_firstname: row.get(3), 
                    users_mail: row.get(4),
                    start_date : row.get(5),
                    end_date : row.get(6), 
                    degree : row.get(7), 
                    description : row.get(8), 
                    pros: row.get(9), 
                    cons: row.get(10),
                    contrat_name : row.get(11),
                };
                list.push_back(internship);
                }
            } else {

                for row in conn.query(&format!("SELECT Distinct internship.id_internship, internship.name,users.name, users.firstname, users.mail,internship.start_date,
                internship.end_date,internship.degree,internship.description,internship.pros, internship.cons, contrat.name FROM company 
                INNER JOIN has_been_made_in on (company.id_company = has_been_made_in.id_company) 
                INNER JOIN internship on (internship.id_internship = has_been_made_in.id_internship) 
                WHERE internship.id in ('{}')", internship),&[]).unwrap().iter(){
                    let internship = InternshipDisplay {
                    id_internship : row.get(0),
                    internship_name : row.get(1),
                    users_name: row.get(2), 
                    users_firstname: row.get(3), 
                    users_mail: row.get(4),
                    start_date : row.get(5),
                    end_date : row.get(6), 
                    degree : row.get(7), 
                    description : row.get(8), 
                    pros: row.get(9), 
                    cons: row.get(10),
                    contrat_name : row.get(11),
                };
                list.push_back(internship);
                }
            }
            content::Json(json!({"Internship" : list}).to_string())
        }else {
            content::Json(json!({"Internship" : list}).to_string())
        }
}

#[post("/search_ets", format="application/json", data="<input>")]
fn search_ets(_token : Token,input : Json<SearchStruct>) -> content::Json<String>{
    let mut contrats : String = "".to_string();
    let _tags : String;
    let mut internship : String = "".to_string();
    let mut resulting =false;
    println!("{}", input.pos.center_lat);
    let conn = Connection::connect("postgres://killy:rustycode44@54.38.244.17:5432/rustDb",TlsMode::None).unwrap();
    let mut list: LinkedList<EnterpriseInit> = LinkedList::new(); 
    let mut result = conn.query("SELECT DISTINCT id_internship from internship", &[]);
    if input.tags.len() <= 0 && input.contrats.len() <= 0 {
            if input.pos.zoom_level == -1 {
                    for row in conn.query("SELECT Distinct company.id_company, company.name, company.longitude, company.latitude  FROM company ",
                        &[]).unwrap().iter(){
                        let enterprise = EnterpriseInit {
                        id: row.get(0),
                        name: row.get(1),
                        longitude : row.get(2),
                        latitude : row.get(3)
                    };
                    list.push_back(enterprise);
                    }
                    content::Json(json!({"points" : list}).to_string())
                } else {
                    for row in conn.query("SELECT Distinct company.id_company, company.name, company.longitude, company.latitude  FROM company 
                        WHERE (latitude > $1 AND latitude < $2) 
                        AND (longitude > $3 AND longitude < $4)",
                        &[&scale_float_sup(input.pos.center_lat, input.pos.zoom_level, true),
                        &scale_float_add(input.pos.center_lat, input.pos.zoom_level, true),
                        &scale_float_sup(input.pos.center_long, input.pos.zoom_level, false),
                        &scale_float_add(input.pos.center_long, input.pos.zoom_level, false)
                        ]).unwrap().iter(){
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
    
    }else {
    if input.tags.len() >0 {
        let mut in_tags = "".to_string();
        for elem in input.tags.iter(){
            in_tags = in_tags + &format!("'{}',",elem.to_string());
        }
        in_tags.pop();
            result = conn.query(&format!("SELECT DISTINCT id_internship from tag 
                    INNER JOIN has_tag on (tag.id_tag = has_tag.id_tag)
                    WHERE tag.name in ({})", in_tags), &[]);
    }
            
    for row in result.unwrap().iter(){
        resulting = true;
        let tmp : i32 = row.get(0);
        internship = internship + &format!("{},", tmp);
    }
        if resulting{
            internship.pop();
            
            if input.contrats.len() > 0 {
                for elem in input.contrats.iter(){
                    contrats = contrats + &format!("'{}',", elem)
                }
                contrats.pop();
                if input.pos.zoom_level == -1 {
                    for row in conn.query(&format!("SELECT Distinct company.id_company, company.name, company.longitude, company.latitude  FROM company 
                        INNER JOIN has_been_made_in on (company.id_company = has_been_made_in.id_company) 
                        INNER JOIN internship on (internship.id_internship = has_been_made_in.id_internship) 
                        WHERE internship.id_internship in ({})
                        AND internship.type_of_contrat in ({})", internship, contrats), 
                        &[]).unwrap().iter(){
                    let enterprise = EnterpriseInit {
                    id: row.get(0),
                    name: row.get(1),
                    longitude : row.get(2),
                    latitude : row.get(3)
                };
                list.push_back(enterprise);
                }
                }
                else {
                    for row in conn.query(&format!("SELECT Distinct company.id_company, company.name, company.longitude, company.latitude  FROM company 
                        INNER JOIN has_been_made_in on (company.id_company = has_been_made_in.id_company) 
                        INNER JOIN internship on (internship.id_internship = has_been_made_in.id_internship) 
                        WHERE internship.id_internship in ({})
                        AND internship.type_of_contrat in ({})
                        AND (latitude > $1 AND latitude < $2) 
                        AND (longitude > $3 AND longitude < $4)", internship, contrats), 
                        &[&scale_float_sup(input.pos.center_lat, input.pos.zoom_level, true),
                        &scale_float_add(input.pos.center_lat, input.pos.zoom_level, true),
                        &scale_float_sup(input.pos.center_long, input.pos.zoom_level, false),
                        &scale_float_add(input.pos.center_long, input.pos.zoom_level, false)
                        ]).unwrap().iter(){
                    let enterprise = EnterpriseInit {
                    id: row.get(0),
                    name: row.get(1),
                    longitude : row.get(2),
                    latitude : row.get(3)
                };
                list.push_back(enterprise);
                }
                }
                
            } else {
                if input.pos.zoom_level == -1 {
                    for row in conn.query(&format!("SELECT Distinct company.id_company, company.name, company.longitude, company.latitude  FROM company 
                        INNER JOIN has_been_made_in on (company.id_company = has_been_made_in.id_company) 
                        INNER JOIN internship on (internship.id_internship = has_been_made_in.id_internship) 
                        WHERE internship.id_internship in ({})", internship),
                        &[]).unwrap().iter(){
                        let enterprise = EnterpriseInit {
                        id: row.get(0),
                        name: row.get(1),
                        longitude : row.get(2),
                        latitude : row.get(3)
                    };
                    list.push_back(enterprise);
                    }
                } else {
                    for row in conn.query(&format!("SELECT Distinct company.id_company, company.name, company.longitude, company.latitude  FROM company 
                        INNER JOIN has_been_made_in on (company.id_company = has_been_made_in.id_company) 
                        INNER JOIN internship on (internship.id_internship = has_been_made_in.id_internship) 
                        WHERE internship.id_internship in ({})
                        AND (latitude > $1 AND latitude < $2) 
                        AND (longitude > $3 AND longitude < $4)", internship),
                        &[&scale_float_sup(input.pos.center_lat, input.pos.zoom_level, true),
                        &scale_float_add(input.pos.center_lat, input.pos.zoom_level, true),
                        &scale_float_sup(input.pos.center_long, input.pos.zoom_level, false),
                        &scale_float_add(input.pos.center_long, input.pos.zoom_level, false)
                        ]).unwrap().iter(){
                        let enterprise = EnterpriseInit {
                        id: row.get(0),
                        name: row.get(1),
                        longitude : row.get(2),
                        latitude : row.get(3)
                    };
                    list.push_back(enterprise);
                    }
                }
            }
            content::Json(json!({"points" : list}).to_string())
        }else {
            content::Json(json!({"points" : list}).to_string())
        }
    }
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

fn date_converter(date: String) -> chrono::NaiveDate {

        let date : Vec<_> = date.split("/").collect();
        let date_fmt = NaiveDate::from_ymd(date[2].parse::<i32>().unwrap(), date[1].parse::<u32>().unwrap(), date[0].parse::<u32>().unwrap());

        return date_fmt;
}

fn is_valid(key: &str) -> bool {
    let conn = Connection::connect("postgres://killy:rustycode44@54.38.244.17:5432/rustDb",
            TlsMode::None).unwrap();
    let result = conn.query(&format!("SELECT date from token where value='{}'", key), &[]);
    let mut count = 0;
    for _ in result.unwrap().iter(){
        count+=1;
    }
    if count ==1 {
            let now = Utc::now();
            let result = conn.query("UPDATE token set date = $1 where value = $2", &[&now.to_string(), &key]);
            result.is_ok()
    }
    else{
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

fn date_converter(date: String) -> chrono::NaiveDate {

        let date : Vec<_> = date.split("/").collect();
        let date_fmt = NaiveDate::from_ymd(date[2].parse::<i32>().unwrap(), date[1].parse::<u32>().unwrap(), date[0].parse::<u32>().unwrap());

        return date_fmt;
}

fn main() {
    let default = rocket_cors::Cors::default();
    rocket::ignite()
    .attach(default)
    .mount("/", routes![hello,
                        signin, 
                        authenticate,
                        tags,
                        create_company, 
                        create_internship,
                        company, 
                        contract,
                        search_ets,
                        refresh_token, 
                        search_internships,
                        company_display])
    .launch();
} 
