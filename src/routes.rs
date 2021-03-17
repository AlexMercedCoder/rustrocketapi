use rocket_contrib::json::Json;
use std::collections::HashMap;


// Macro for annotating our route methods
#[get("/")]
pub fn index() -> Json<HashMap<String, String>> {
    let mut my_map = HashMap::new();
    my_map.insert(String::from("cheese"), String::from("gouda"));
    my_map.insert(String::from("bread"), String::from("rye"));
    // turn hashmap into json and return it
    return Json(my_map);
}

// Macro for annotating our route methods
#[get("/cheese/<cheeseType>")]
pub fn cheese(cheeseType: String) -> String {
    format!("So... you like {} cheese!", cheeseType)
}

// Macro for annotating our route methods
#[get("/cheese?<cheeseType>")]
pub fn queso(cheeseType: String) -> String {
    format!("So... you still like {} cheese!", cheeseType)
}