#[derive(Serialize, Deserialize)]
struct User {
    name: String,
    firstname: String,
    mail: String,
    pswd: String
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