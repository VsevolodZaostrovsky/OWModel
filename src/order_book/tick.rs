use serde_derive::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
pub struct Tick
{
    pub NO: i64,
    pub SECCODE: String,
    pub BUYSELL: String,
    pub TIME: String,
    pub ORDERNO: i64,
    pub ACTION: i8,
    pub PRICE: f64,
    pub VOLUME: i64,

    #[serde(deserialize_with = "csv::invalid_option")]
    pub TRADENO: Option<f64>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub TRADEPRICE: Option<f64>
}