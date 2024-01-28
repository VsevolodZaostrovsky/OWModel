pub mod quote_l3;
pub mod tick;
pub mod side;


use chrono::NaiveTime;

use std::path::Path;
use std::io::{Write, BufWriter};
use std::fs::File;
use std::collections::{BTreeMap, VecDeque, HashMap, HashSet};

use csv;

use quote_l3::L3Quote;
use tick::Tick;
use side::Side;



fn __make_file(filename: String) -> BufWriter<File>{
    let path = Path::new(&filename);
    let display = path.display();
    let file = match File::create(&path) {
        Err(why) => panic!("Couldn't open {}: {}", display, std::io::Error::to_string(&why)),
        Ok(file) => BufWriter::new(file),
    };
    file
}





#[derive(Debug, PartialEq)]
pub struct OrderBook
{
    pub instrument: String,
    pub instruments: HashSet<String>,
    pub date: NaiveTime,
    // pub date: String,

    pub bid: BTreeMap<i64, VecDeque<L3Quote>>,
    pub ask: BTreeMap<i64, VecDeque<L3Quote>>,
    // pub trading_instensity: 
    // pub volatility: Vec<f64>

    // далее можно улучшить, и сторить для всех инструментов. Надо вначале пройтись по всему файлу 
    // и собрать названия инструментов и разбинуть нужным образом файл, для будущей паралельки
    // pub bid: HashMap<String, BTreeMap<i64, VecDeque<L3Quote>>>,
    // pub ask: HashMap<String, BTreeMap<i64, VecDeque<L3Quote>>>,

    // на самом деле, хорошим решением будет написать общий класс, и сделать подкласс ордербук
    // и но надо продумать по какому паттерну делать 
    // также в моей задумке мы имеем библиотеку алгоритмов, которые используют реализацию этого стакана


    price_step: f64,
    price_step_inv: f64,

    bid_ids: HashMap<i64, i64>,
    ask_ids: HashMap<i64, i64>,
    
    high_bid: Vec<f64>,
    low_ask: Vec<f64>
}

impl OrderBook
{
    /// Returns empty side.
    fn new_side() -> BTreeMap<i64, VecDeque<L3Quote>>
    {
        BTreeMap::new()
    }

    /// Returns empty order book for given instrument and price step.
    pub fn new(instrument: String, price_step: f64) -> OrderBook
    {
        let mut instruments: HashSet<String> = HashSet::new();
        instruments.insert(instrument.clone());

        OrderBook{ instrument, instruments, date: NaiveTime::parse_from_str("10:00:00", "%H:%M:%S").unwrap(), bid: OrderBook::new_side(), ask: OrderBook::new_side(), 
                   price_step, price_step_inv: 1./price_step, bid_ids: HashMap::new(), 
                   ask_ids: HashMap::new(), low_ask: Vec::new(), high_bid: Vec::new() 
        }
    }

    /// Returns the matched side's triplet consisting of quotes, quote ids and "ASK" or "BID" string.
    fn match_side_type(&mut self, side_type: &Side) -> (&mut BTreeMap<i64, VecDeque<L3Quote>>, &mut HashMap<i64, i64>, &'static str, &mut Vec<f64>)
    {
        match side_type {
            Side::ASK => (&mut self.ask, &mut self.ask_ids, "ASK", &mut self.high_bid),
            Side::BID => (&mut self.bid, &mut self.bid_ids, "BID", &mut self.low_ask)
        }
    }


    /// Add quote in order book.
    /// 
    /// # Panics
    /// This function will panic if id is not unique and is already presented in this side of order book.
    fn add(&mut self, side_type: &Side, price_step_inv: f64, price : f64, size: i64, orderno: i64)
    {
        // !!!!!! нужно оптимизировать случай, когда цена нулевая и проверка на ошибки  
        let price_key: i64;
        let temp: f64; 
        let (tree, map, text, mut_mass) = self.match_side_type(side_type);

        if map.get(&orderno) == None
        {
            if price == 0.0 {
                temp = *mut_mass.last().unwrap();
            } else {
                temp = price;
            }
            price_key = (temp * price_step_inv) as i64;
            tree.entry(price_key).or_insert(VecDeque::new()).push_back(L3Quote {
                id : orderno,
                price : temp,
                size,
            });
            map.insert(orderno, price_key);
        }
        else
        {
            panic!("Id exists in {}.", text);
        }
    }


    /// Remove quote from order book
    /// 
    /// # Panics
    /// This function will panic if we try to remove non-existing id.
    fn remove(&mut self, side_type: &Side, ORDERNO: i64, volume: i64)
    {
        // !!!!!! оптимизировать когда нулевой объем
        let mut flag = false;
        let (tree, map, text, _) = self.match_side_type(side_type);
        if let Some(price_level) = map.get(&ORDERNO)
        {    
            if volume == 0{
                tree.entry(*price_level).and_modify(|x| x.retain(|q| q.id != ORDERNO));
                map.remove(&ORDERNO);
            } else {
                // переписать через filter
                tree.entry(*price_level).and_modify(
                    |x: &mut VecDeque<L3Quote>| x.iter_mut().for_each(
                        |q|{
                            if q.id == ORDERNO 
                            { 
                                if q.size == volume{
                                    flag = true;
                                } else {
                                    q.size = volume
                                }
                            }
                        }
                    )
                );
                if flag{
                    tree.entry(*price_level).and_modify(|x| x.retain(|q| q.id != ORDERNO));
                    map.remove(&ORDERNO);
                }
            }
            self.clean();
        }
        else
        {
            panic!("No such id in {}.\nTried to remove id {}", text, ORDERNO);
        }
    }
    
    /// Analysing of deal's quotes
    pub fn deal(&mut self, side_type: &Side, ORDERNO: i64, volume: i64)
    {
        // !!!!!!!! тут все отвратительно и не читаемо
        let (tree, map, text, _) = self.match_side_type(side_type);
        if let Some(price_level) = map.get(&ORDERNO)
        {    
            // переписать через filter
            let mut flag_to_remove : bool = false;
            tree.entry(*price_level).and_modify(
                |x| x.iter_mut().for_each(
                    |q| {
                        if q.id == ORDERNO {
                            q.size -= volume;
                            if q.size == 0{ flag_to_remove = true; }
                        }
                    }
                )
            );
            if flag_to_remove{
                self.remove(&side_type, ORDERNO, 0);
            }
        }
        else
        {
            panic!("No such id in {}.\nTried to remove id {}", text, ORDERNO);
        }
    }

    /// Clean empty quotes from order book and check if bid & ask overlap.
    fn clean(&mut self)
    {
        self.ask.retain(|_k,v| v.len() != 0);
        self.bid.retain(|_k,v| v.len() != 0);
        // self.check_bid_ask_overlap();
    }

    /// Check bid & ask for overlap and panic if the intersection is not empty.
    fn check_bid_ask_overlap(&self)
    {
        if let Some(max_bid) = self.bid.keys().max()
        {
            if let Some(min_ask) = self.ask.keys().min()
            {
                if max_bid > min_ask
                {
                    // self.print();
                    panic!("Max bid is greater than min ask.");
                }
            }
        }
    }


    /// Update the order book with one line string data.
    /// 
    /// # String example
    /// 
    /// 
    /// # Panics
    /// This function will panic if given string is malformed, e.g. does not contain necessary data about quote or the instrument is mismatched.
    pub fn update(&mut self, line: &Tick, step_iter: &mut i32, file_l3: &mut BufWriter<File>, file_l2_ask: &mut BufWriter<File>, file_l2_bid: &mut BufWriter<File>)
    {
        // переписать всю функцию на итератор
        if self.instrument == line.SECCODE
        {
            // println!("len {}", line.TIME.as_str().chars().count());
            if line.TIME.as_str().chars().count() == 11
            {
                let zero: &str = "0";
                let time_without_zero: &str =  line.TIME.as_str().clone();

                let time_corrected = format!("{zero}{time_without_zero}");

                self.date = match NaiveTime::parse_from_str(&time_corrected, "%H%M%S%f") {
                    Err(why) => panic!("Couldn't parse {} and {}", &time_corrected, why),
                    Ok(time) => time,
                };
                // println!("{}", format!("{zero}{time_without_zero}"));
            }
            else 
            {
                self.date = match NaiveTime::parse_from_str(line.TIME.as_str().clone(), "%H%M%S%f") {
                    Err(why) => panic!("Couldn't parse {} and {}", line.TIME.as_str().clone(), why),
                    Ok(time) => time,
                };
            }

            *step_iter += 1;
            let side_type = if line.BUYSELL == String::from("B") { Side::BID } else { Side::ASK };

            match line.ACTION {
                0 => self.remove(&side_type, line.ORDERNO, line.VOLUME),
                1 => self.add(&side_type, self.price_step_inv, line.PRICE, line.VOLUME, line.ORDERNO),
                2 => {
                    self.deal(&side_type, line.ORDERNO, line.VOLUME);
                    match side_type {
                        Side::BID => {                        
                            let result_ask: f64 = match self.ask.keys().next(){
                                Some(p) => (*p as f64) * self.price_step,
                                None => 0.0,
                            };
                            self.high_bid.push(*self.high_bid.last().unwrap());
                            self.low_ask.push(result_ask);
                        },
                        Side::ASK => {               
                            let result_bid: f64 = match self.bid.keys().last(){
                                Some(p) => (*p as f64) * self.price_step,
                                None => 0.0,
                            };
                            self.high_bid.push(result_bid);
                            self.low_ask.push(*self.low_ask.last().unwrap());
                        },
                    }
                },
                _ => panic!("\nFound incorrect state of ACTION\n"),
            }

            // оптимизировать момент когда цена 0.0 (те по лучшей рыночной ставим)
            // If values of best bid/low ask doesn't exist that we enter 0.0
            if line.ACTION != 2{
                let result_bid: f64 = match self.bid.keys().last(){
                    Some(p) => (*p as f64) * self.price_step,
                    None => 0.0,
                };

                let result_ask: f64 = match self.ask.keys().next(){
                    Some(p) => (*p as f64) * self.price_step,
                    None => 0.0,
                };
                
                self.high_bid.push(result_bid);
                self.low_ask.push(result_ask);
            }

            // self.l3_print_to_file(file_l3);

            // write!(file_l2_bid, "{} ", line.ACTION).expect("TODO: panic message");
            // write!(file_l2_ask, "{} ", line.ACTION).expect("TODO: panic message");

            // write!(file_l2_bid, "{} Price {} Vol {} \n", line.BUYSELL, line.PRICE, line.VOLUME).expect("TODO: panic message");
            // write!(file_l2_ask, "{} Price {} Vol {} \n", line.BUYSELL, line.PRICE, line.VOLUME).expect("TODO: panic message");

            if line.ACTION == 2 {
                if line.BUYSELL == "B" {
                    self.l2_print_to_file(file_l2_bid, file_l2_ask);
                    write!(file_l2_bid, "Price {} Vol {} \n", line.PRICE, line.VOLUME).expect("TODO: panic message");
                    write!(file_l2_ask, "Price {} Vol {} \n", line.PRICE, line.VOLUME).expect("TODO: panic message");
                }
                if line.BUYSELL == "S" {
                    self.l2_print_to_file(file_l2_bid, file_l2_ask);
                    // write!(file_l2_bid, "Price {} Vol {} \n", line.PRICE, line.VOLUME).expect("TODO: panic message");
                    // write!(file_l2_ask, "Price {} Vol {} \n", line.PRICE, line.VOLUME).expect("TODO: panic message");
                }
            }

        }
        else
        {
            if !self.instruments.contains(&line.SECCODE){
                self.instruments.insert(line.SECCODE.clone());
            }
            
        }

    }

    /// Returns new order book created from file.
    /// # Panics
    /// This function will panic if file could not be opened (e.g. it does not exist) or contains malformed quotes.
    pub fn from_file(filename: &str, instrument: String, price_step: f64,) -> OrderBook
    {
        let mut orderbook = OrderBook::new(instrument.clone(), price_step);

        let mut reader = match csv::Reader::from_path(filename){
            Err(why) => panic!("Couldn't open {}: {}", filename, csv::Error::to_string(&why)),
            Ok(file) => file,
        };
        
        let mut l3_order_book_file: BufWriter<File> = __make_file(format!("{}_l3_orderbook.txt", instrument.clone().as_str()));
        let mut l2_order_book_ask_file: BufWriter<File> = __make_file(format!("{}_l2_ask.txt", instrument.clone().as_str()));
        let mut l2_order_book_bid_file: BufWriter<File> = __make_file(format!("{}_l2_bid.txt", instrument.clone().as_str()));

        let mut step_iter: i32 = 0;
        for result in reader.deserialize()
        {
            let record: Tick = match result {
                Err(why) => panic!("Couldn't parse string by serde {}: {}", filename, csv::Error::to_string(&why)),
                Ok(file) => file,
            };
            orderbook.update(&record, &mut step_iter, 
                            &mut l3_order_book_file, &mut l2_order_book_ask_file, &mut l2_order_book_bid_file);


        }
        orderbook.clean();
        l3_order_book_file.flush().expect("Error with flush");
        l2_order_book_ask_file.flush().expect("Error with flush");
        l2_order_book_bid_file.flush().expect("Error with flush");

        orderbook
    }

    pub fn update_time_window(&mut self, line: &Tick, step_iter: &mut i32, file_l3: &mut BufWriter<File>, file_l2_ask: &mut BufWriter<File>, 
        file_l2_bid: &mut BufWriter<File>, window_start: &String, window_end: &String)
    {
        // переписать всю функцию на итератор

        // println!("{}", window_start);
        // println!("{}", window_end);

        let window_start_NT: NaiveTime = match NaiveTime::parse_from_str(window_start.as_str().clone(), "%H%M%S%f") {
            Err(why) => panic!("Couldn't parse {} and {}", window_start.as_str().clone(), why),
            Ok(time) => time,
        };

        let window_end_NT: NaiveTime = match NaiveTime::parse_from_str(window_end.as_str().clone(), "%H%M%S%f") {
            Err(why) => panic!("Couldn't parse {} and {}", window_end.as_str().clone(), why),
            Ok(time) => time,
        };

        // println!("{}", window_start_NT);
        // println!("{}", window_end_NT);

        if self.instrument == line.SECCODE
        {
            self.date = match NaiveTime::parse_from_str(line.TIME.as_str().clone(), "%H%M%S%f") {
                Err(why) => panic!("Couldn't parse {} and {}", line.TIME.as_str().clone(), why),
                Ok(time) => time,
            };
            *step_iter += 1;
            let side_type = if line.BUYSELL == String::from("B") { Side::BID } else { Side::ASK };

            match line.ACTION {
                0 => self.remove(&side_type, line.ORDERNO, line.VOLUME),
                1 => self.add(&side_type, self.price_step_inv, line.PRICE, line.VOLUME, line.ORDERNO),
                2 => {
                    self.deal(&side_type, line.ORDERNO, line.VOLUME);
                    match side_type {
                        Side::BID => {                        
                            let result_ask: f64 = match self.ask.keys().next(){
                                Some(p) => (*p as f64) * self.price_step,
                                None => 0.0,
                            };
                            self.high_bid.push(*self.high_bid.last().unwrap());
                            self.low_ask.push(result_ask);
                        },
                        Side::ASK => {               
                            let result_bid: f64 = match self.bid.keys().last(){
                                Some(p) => (*p as f64) * self.price_step,
                                None => 0.0,
                            };
                            self.high_bid.push(result_bid);
                            self.low_ask.push(*self.low_ask.last().unwrap());
                        },
                    }
                },
                _ => panic!("\nFound incorrect state of ACTION\n"),
            }

            // оптимизировать момент когда цена 0.0 (те по лучшей рыночной ставим)
            // If values of best bid/low ask doesn't exist that we enter 0.0
            if line.ACTION != 2{
                let result_bid: f64 = match self.bid.keys().last(){
                    Some(p) => (*p as f64) * self.price_step,
                    None => 0.0,
                };

                let result_ask: f64 = match self.ask.keys().next(){
                    Some(p) => (*p as f64) * self.price_step,
                    None => 0.0,
                };
                
                self.high_bid.push(result_bid);
                self.low_ask.push(result_ask);
            }


                println!("window_start_NT {}", window_start_NT);

                println!("date {}", self.date);

                println!("date > window_start_NT: {}", self.date > window_start_NT);

                println!("window_end_NT {}", window_end_NT);

                println!("date < window_end_NT: {}", self.date < window_end_NT);

                if self.date > window_start_NT && self.date < window_end_NT
                {
                    self.l3_print_to_file(file_l3);
                    self.l2_print_to_file(file_l2_bid, file_l2_ask);
                }
        }
        else
        {
            if !self.instruments.contains(&line.SECCODE){
                self.instruments.insert(line.SECCODE.clone());
            }
            
        }

    }

    pub fn from_file_time_window(filename: &str, instrument: String, price_step: f64, window_start: &String, window_end: &String) -> OrderBook
    {                                                                                                                                                                                                                                       
        let mut orderbook = OrderBook::new(instrument.clone(), price_step);                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     

        let mut reader = match csv::Reader::from_path(filename){                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        
            Err(why) => panic!("Couldn't open {}: {}", filename, csv::Error::to_string(&why)),
            Ok(file) => file,
        };
        
        let mut l3_order_book_file: BufWriter<File> = __make_file(format!("{}_l3_orderbook.txt", instrument.clone().as_str()));
        let mut l2_order_book_ask_file: BufWriter<File> = __make_file(format!("{}_l2_ask.txt", instrument.clone().as_str()));
        let mut l2_order_book_bid_file: BufWriter<File> = __make_file(format!("{}_l2_bid.txt", instrument.clone().as_str()));

        let mut step_iter: i32 = 0;
        for result in reader.deserialize()
        {
            let record: Tick = match result {
                Err(why) => panic!("Couldn't parse string by serde {}: {}", filename, csv::Error::to_string(&why)),
                Ok(file) => file,
            };
            orderbook.update_time_window(&record, &mut step_iter, 
                            &mut l3_order_book_file, &mut l2_order_book_ask_file, &mut l2_order_book_bid_file, window_start, window_end);


        }
        orderbook.clean();
        l3_order_book_file.flush().expect("Error with flush");
        l2_order_book_ask_file.flush().expect("Error with flush");
        l2_order_book_bid_file.flush().expect("Error with flush");

        orderbook
    }
    
    /// Returns converted from L3 to L2 matched side.
    pub fn to_l2(&self, side: Side) -> Vec<(f64, f64)>
    {
        let book_side = match side {
            Side::ASK => &self.ask,
            Side::BID => &self.bid
        };
        let mut l2book_side: Vec<(f64, f64)> = book_side.iter().map(|(&k, v)| ((k as f64) * self.price_step, v.iter().fold(0., |acc, x| acc + (x.size as f64)))).collect();
        if side == Side::BID
        {
            l2book_side.reverse();
        }
        l2book_side
    }

    


    /// Prints full order book state to the file for current date.
    pub fn l3_print_to_file(&self, file : &mut BufWriter<File>)
    {
        write!(file, "Order book for {} at {}\n", self.instrument, self.date).expect("TODO: panic message");
        self.l3_print_side_to_file(Side::BID, file);
        self.l3_print_side_to_file(Side::ASK, file);
        writeln!(file, "================================\n").expect("TODO: panic message");

    }

    /// Prints l2 order book state to the file for current date.
    pub fn l2_print_to_file(&self, file_bid : &mut BufWriter<File>, file_ask : &mut BufWriter<File>)
    {
        self.l2_print_side_to_file(Side::BID, file_bid);
        self.l2_print_side_to_file(Side::ASK, file_ask);
    }



    pub fn l2_print_side_to_file(&self, _side: Side, file: &mut BufWriter<File>)
    {
        let  l2_book = match _side {
            Side::ASK => self.to_l2(Side::ASK),
            Side::BID => self.to_l2(Side::BID),
        };

        write!(file, "{} ", self.date).expect("TODO: panic message");

        write!(file, "[").expect("TODO: panic message");

        for (key, value) in l2_book
        {
            write!(file, "[ {:.3}, {} ], ", key, value).expect("TODO: panic message");   
        }
        
        write!(file, "[0., 0.]").expect("TODO: panic message");
        writeln!(file, "]").expect("TODO: panic message");
    }

    /// Prints current matched side state to the file.
    pub fn l3_print_side_to_file(&self, _side: Side, file: &mut BufWriter<File>)
    {
        let (side, text) = match _side {
            Side::ASK => (&self.ask, "Ask"),
            Side::BID => (&self.bid, "Bid"),
        };
        writeln!(file, "{}", text).expect("TODO: panic message");

        for (key, value) in side
        {
            let price_level: f64 = (*key as f64) * self.price_step;
            write!(file, "RUB {:.3}: ", price_level).expect("TODO: panic message");        
            for quote in value {
                write!(file, "(id: {}, size: {}), ", quote.id, quote.size).expect("TODO: panic message");        
                
            }
            writeln!(file).expect("TODO: panic message");        
        }
        writeln!(file).expect("TODO: panic message");        
    }


    /// Prints current matched side state.
    pub fn print_side(&self, _side: Side)
    {
        let (side, text) = match _side {
            Side::ASK => (&self.ask, "Ask"),
            Side::BID => (&self.bid, "Bid"),
        };
        println!("{text}");
        for (key, value) in side
        {
            let price_level: f64 = (*key as f64) * self.price_step;
            print!("RUB {:.4}: ", price_level);
            for quote in value {
                print!("(id: {}, size: {}), ", quote.id, quote.size);
            }
            print!("\n");
        }
        print!("\n");
    }

    /// Prints full order book state for current date.
    pub fn print(&self)
    {
        println!("Order book for {} at {}", self.instrument, self.date);
        self.print_side(Side::BID);
        self.print_side(Side::ASK);
    }

    // Update the order book with string data, separated with new line character '\n'.
    //
    // 
    // * `lines` - string data.
    // pub fn update_from_string(&mut self, lines: String)
    // {
    //     let lines: Vec<String> = lines.split("\n").map(|x| x.to_string()).collect();
    //     for line in &lines
    //     {
    //         self.update(line);
    //     }
    //     self.clean();
    // }


    // Returns new order book created from JSON.
    // 
    // # Panics
    // This function will panic if data contains malformed quotes.
    // pub fn from_str(json: &str, instrument: String, price_step: f64) -> OrderBook
    // {
    //     let mut orderbook = OrderBook::new(instrument, price_step);
    //     let lines: Vec<String> = json.split("\n").map(|x| x.to_string()).collect();

    //     for line in &lines
    //     {
    //         orderbook.update(line);
    //     }
    //     orderbook.clean();
    //     orderbook
    // }


    // Writes features in the given file.
    // fn feature_println(features: Vec<String>, file: &mut File)
    // {
    //     file.write_all(features.join("\t").as_bytes()).expect("TODO: panic message");
    //     file.write_all("\n".as_bytes()).expect("TODO: panic message");
    // }

    // // Returns vector of calculated (on L2 data) features (currently calculates only price by volume P(Q)).
    // fn calculator(l2side: Vec<(i64, f64)>) -> Vec<(f64, i64)>
    // {
    //     let mut features_pq: Vec<(f64, i64)> = Vec::new();
    //     if !l2side.is_empty()
    //     {
    //         // Calculate P = P(Q)
    //         let mut total_volume: f64 = 0.0;
    //         for (price, volume) in &l2side
    //         {
    //             total_volume += volume;
    //             features_pq.push((total_volume, *price));
    //         }
    //     }
    //     features_pq
    // }

    // Calculates features and outputs them in .npy files (currently calculates only price by volume P(Q)).
    // pub fn calculate_features(filename: &str, instrument: String, price_step: f64)
    // {
    //     let mut orderbook = OrderBook::new(instrument, price_step);
    //     let path = Path::new(filename);
    //     let display = path.display();
    //     let file = match File::open(&path) {
    //         Err(why) => panic!("Couldn't open {}: {}", display, std::Error::to_string(&why)),
    //         Ok(file) => file,
    //     };
    //     let reader = BufReader::new(file);
    //     let lines = reader.lines();

    //     let mut pq_bid_file = File::create(format!("{}_PQ_bid.tsv", filename)).unwrap();
    //     let mut pq_ask_file = File::create(format!("{}_PQ_ask.tsv", filename)).unwrap();

    //     let mut i: i64 = 0;
    //     for result in lines
    //     {
    //         if let Ok(line) = result
    //         {
    //             orderbook.update(&line);
    //             orderbook.clean();
    //         }
    //         if i % 2 == 1
    //         {
    //             let l2_bid: Vec<(i64, f64)> = orderbook.to_l2(Side::BID);
    //             let l2_ask: Vec<(i64, f64)> = orderbook.to_l2(Side::ASK);

    //             let (features_pq_bid, features_pq_ask) = (OrderBook::calculator(l2_bid), OrderBook::calculator(l2_ask));
    //             if i % 100 == -1
    //             {
    //                 OrderBook::feature_println(features_pq_bid.iter().map(|x| String::from(format!("{:?}", x))).collect(), &mut pq_ask_file);
    //                 OrderBook::feature_println(features_pq_ask.iter().map(|x| String::from(format!("{:?}", x))).collect(), &mut pq_bid_file);
    //             }
    //         }
    //         i+=1;
    //     }
    // }
}