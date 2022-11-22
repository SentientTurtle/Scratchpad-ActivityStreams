use serde::{Deserialize, Serialize};
use crate::linkeddata::util::FoldedSlice;

mod linkeddata;

#[derive(Serialize, Deserialize, Debug)]
struct Test {
    name: String,
    data: FoldedSlice<String>
}

pub fn main() {
    let vec = vec!["Hello!".to_string()];
    let test = Test {
        name: "Test".to_string(),
        data: FoldedSlice::from(vec)
    };
    let json = serde_json::to_string(&test).unwrap();
    println!("{}", json);
    let string = "{\"name\":\"Test\",\"data\":\"Hello!\"}";
    let test2 = serde_json::from_str::<Test>(string).unwrap();
    println!("{:?}", test);
    println!("{:?}", test2);
}