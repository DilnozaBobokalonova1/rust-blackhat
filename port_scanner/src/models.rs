// a powerful Rust library for serializing &
// deserializing data between different formats
use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct Port {
    pub port: u16,
    pub is_open: bool,
}

#[derive(Debug, Clone)]
pub struct Subdomain {
    pub domain: String,
    pub open_ports: Vec<Port>
}

//using Deserialize to specify how data should be 
//converted from a serialized format (like JSON) 
//into Rust data structures
#[derive(Debug, Deserialize, Clone)]
pub struct CrtShEntry {
    pub name_value: String,
}
