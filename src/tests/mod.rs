use std::error::Error;
use crate::linkeddata::{AsLinkedData, LinkedData};
use serde::{Serialize, Deserialize};
use crate::linkeddata::activity_streams::{MaybeObject, TaggedObject};

#[derive(Debug, Serialize, Deserialize)]
struct TestLD {
    a: String,
}

impl AsLinkedData for TestLD {
    type ContextType = &'static str;
    fn get_ld_context(&self) -> Self::ContextType {
        "TEST"
    }
}

/// Attempt to parse ./data.json as an ActivityStreams object
///
/// This test also provides additional compile time validation of the AxtivityStream object types by ensuring they implement Serde's Serialize and Deserialize.
#[test]
fn test_parse() -> Result<(), Box<dyn Error>> {
    let resp = include_str!("./data.json");
    let value = serde_json::from_str::<LinkedData<TaggedObject, serde_json::Value>>(&*resp).unwrap();
    println!("{:#?}", value);
    Ok(())
}
