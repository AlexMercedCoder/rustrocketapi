use rocket_contrib::json::Json;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use postgres::{Client, NoTls, Error};
use dotenv::dotenv;
use std::env;
use openssl::ssl::{SslConnector, SslMethod, SslVerifyMode};
use postgres_openssl::MakeTlsConnector;

//CREATE HEROKU TLS BUILDER
fn get_connector() -> MakeTlsConnector {
    // Create Ssl postgres connector without verification as required to connect to Heroku.
    let mut builder = SslConnector::builder(SslMethod::tls()).unwrap();
    builder.set_verify(SslVerifyMode::NONE);
    MakeTlsConnector::new(builder.build())
}

// CREATE STRUCT THAT IS SERIALIZABLE INTO JSON
#[derive(Serialize, Deserialize, Debug)]
pub struct Person {
    id: i32,
    name: String,
    age: i32
}

// CREATE FUNCTION TO ESABLISH DATABASE CONNECTION
fn getConn() -> Result<Client, Error> {
    // GET DATABASE URL ENV VARIABLE
    let uri;
    let connector = get_connector();
    dotenv().ok();
    match env::var("DATABASE_URL2") {
        Ok(val) => uri = val,
        Err(_e) => uri = "none".to_string(),
    }
    print!("{}", uri);
    // return database connection
    return Client::connect(&uri, connector);
}

// INDEX ROUTE TO GET ALL PEOPLE
#[get("/")]
pub fn index() -> Json<Vec<Person>> {
    //declare vector to hold people
    let mut result: Vec<Person> = Vec::new();

    //query database and build vector
    match getConn(){
        Ok(val) => {
            let mut client = val;
            print!("Hello");
            for row in client.query("SELECT * FROM people;", &[]).unwrap() {
                print!("{:?}", row);
                print!("goodbye");
                let id:i32 = row.get(0);
                let name: String= row.get(1);
                let age: i32 = row.get(2);
                result.push(Person { id, name, age});
                
            }
        },
        Err(err) => print!("ERROR: {}", err),
    }
    
    // turn hashmap into json and return it
    return Json(result);
}

// CREATE ROUTE TO CREATE A NEW PERSON
#[post("/<name>/<age>",)]
pub fn create(name: String, age: i32) -> String {

    //insert new person into database
    match getConn(){
        Ok(val) => {
            let mut client = val;
            client.execute("INSERT INTO people (name, age) VALUES ($1, $2)",
            &[&name, &age]).ok();
        },
        Err(err) => print!("{}", err),
    }
    
    // return string denoting completion
    return String::from("Request Complete");
}

// UPDATE ROUTE TO UPDATE A PERSON
#[put("/<id>/<name>/<age>",)]
pub fn update(id: i32, name: String, age: i32) -> String {

    //insert new person into database
    match getConn(){
        Ok(val) => {
            let mut client = val;
            client.execute("UPDATE people SET name=$1, age=$2 WHERE id=$3;",
            &[&name, &age, &id]).ok();
        },
        Err(err) => print!("{}", err),
    }
    
    // return string denoting completion
    return String::from("Request Complete");
}

// DELETE ROUTE TO DELETE PEOPLE
#[delete("/<id>",)]
pub fn destroy(id: i32) -> String {

    //insert new person into database
    match getConn(){
        Ok(val) => {
            let mut client = val;
            client.execute("DELETE FROM people WHERE id=$1",
            &[&id]).ok();
        },
        Err(err) => print!("{}", err),
    }
    
    // return string denoting completion
    return String::from("Request Complete");
}