
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct L3Quote
{
    pub id: i64,
    pub price: f64,
    pub size: i64,
}

