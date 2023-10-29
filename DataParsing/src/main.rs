mod orderbook;

use crate::orderbook::OrderBook;
use tqdm::tqdm;

use std::{path::Path, fs::File, io::Error, io::BufReader, io::BufRead};


fn read_file(filename: String) -> Vec<String> {
    let mut all_names : Vec<String> = Vec::new();
    let mut _create_path: String = "data/".to_owned();
    _create_path.push_str(&filename);
    
    let path = Path::new(_create_path.as_str());
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("Couldn't open {}: {}", display, Error::to_string(&why)),
        Ok(file) => file,
    };

    let reader = BufReader::new(file);
    let lines = reader.lines();

    for result in lines
    {
        if let Ok(line) = result
        {
            all_names.push(format!("data/{}", line));
        }
    }
    
    all_names
}

fn main()
{

    OrderBook::calculate_features("data/USD_RUB_T+1__2022-10-03", "USD/RUB_T+1".to_string(), 0.0025);


}