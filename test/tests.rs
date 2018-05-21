let rocket = rocket::ignite();
let client = Client::new(rocket).expect("valid rocket instance");
let mut response = client.get("/").dispatch();

assert_eq!(response.status(), Status::Ok);
assert_eq!(response.content_type(), Some(ContentType::Plain));
assert_eq!(response.body_string(), Some("Welcome to HORO API.".into()));