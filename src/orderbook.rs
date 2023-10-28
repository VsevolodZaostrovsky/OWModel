use std::{io::BufReader, path::Path, fs::File, io::BufRead, io::Error, collections::{BTreeMap, VecDeque, HashMap}};
use std::io::Write;
use serde::{Deserialize, Serialize};

#[allow(dead_code)]

const BAD_STRING:          &str = "Bad string found. Check the inputs.";
const INSTRUMENT_MISMATCH: &str = "Instrument mismatch. Check the inputs.";

#[derive(PartialEq)]
pub enum Side
{
    BID,
    ASK,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum QuotesEnum {
    INCREMENT(Increment),
    SNAPSHOT(Vec<L3Quote>)
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct L3Quote
{
    pub id: i64,
    pub price: f64,
    pub size: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Increment
{
    pub added: Vec<L3Quote>,
    pub changed: Vec<L3Quote>,
    pub deleted: Vec<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tick
{
    pub date: String,
    pub instrument: String,
    pub r#type: String,
    pub side: String,
    pub quotes: QuotesEnum,
}

#[derive(Debug, PartialEq)]
pub struct OrderBook
{
    pub instrument: String,
    pub date: String,

    pub bid: BTreeMap<i64, VecDeque<L3Quote>>,
    pub ask: BTreeMap<i64, VecDeque<L3Quote>>,

    price_step: f64,
    price_step_inv: f64,

    bid_ids: HashMap<i64, i64>,
    ask_ids: HashMap<i64, i64>,
}

impl OrderBook
{
    fn new_side() -> BTreeMap<i64, VecDeque<L3Quote>>
    {
        BTreeMap::new()
    }

    pub fn new(instrument: String, price_step: f64) -> OrderBook
    {
        OrderBook{ instrument, date: "".to_string(), bid: OrderBook::new_side(), ask: OrderBook::new_side(), price_step, price_step_inv: 1./price_step, bid_ids: HashMap::new(), ask_ids: HashMap::new() }
    }

    fn match_side_type(&mut self, side_type: &Side) -> (&mut BTreeMap<i64, VecDeque<L3Quote>>, &mut HashMap<i64, i64>, &'static str)
    {
        match side_type {
            Side::ASK => (&mut self.ask, &mut self.ask_ids, "ASK"),
            Side::BID => (&mut self.bid, &mut self.bid_ids, "BID")
        }
    }

    fn add(&mut self, side_type: &Side, price_step_inv: f64, price: f64, size: f64, id: i64)
    {
        let price_key = (price * price_step_inv) as i64;

        let (tree, map, text) = self.match_side_type(side_type);
        if map.get(&id) == None
        {
            tree.entry(price_key).or_insert(VecDeque::new()).push_back(L3Quote {
                id,
                price,
                size,
            });
            map.insert(id, price_key);
        }
        else
        {
            panic!("Id exists in {}.", text);
        }
    }

    fn remove(&mut self, side_type: &Side, id: i64)
    {
        let (tree, map, text) = self.match_side_type(side_type);
        if let Some(price_level) = map.get(&id)
        {
            tree.entry(*price_level).and_modify(|x| x.retain(|q| q.id != id));
            map.remove(&id);
        }
        else
        {
            panic!("No such id in {}.\nTried to remove id {}", text, id);
        }
    }

    fn clean(&mut self)
    {
        self.ask.retain(|_k,v| v.len() != 0);
        self.bid.retain(|_k,v| v.len() != 0);
        self.check_bid_ask_overlap();
    }

    fn check_bid_ask_overlap(&self)
    {
        if let Some(max_bid) = self.bid.keys().max()
        {
            if let Some(min_ask) = self.ask.keys().min()
            {
                if max_bid > min_ask
                {
                    self.print();
                    panic!("Max bid is greater than min ask.");
                }
            }
        }
    }

    fn on_increment(&mut self, side_type: Side, increment: Increment)
    {
        for id in increment.deleted
        {
            self.remove(&side_type, id);
        }

        for quote in increment.changed
        {
            self.remove(&side_type, quote.id);
            self.add(&side_type, self.price_step_inv, quote.price, quote.size, quote.id);
        }

        for quote in increment.added
        {
            self.add(&side_type, self.price_step_inv, quote.price, quote.size, quote.id);
        }
    }

    fn on_snapshot(&mut self, side_type: Side, snapshot: Vec<L3Quote>)
    {
        match side_type
        {
            Side::ASK => {
                self.ask.clear();
                self.ask_ids.clear()
            }
            Side::BID => {
                self.bid.clear();
                self.bid_ids.clear()
            }
        };

        for quote in snapshot
        {
            self.add(&side_type, self.price_step_inv, quote.price, quote.size, quote.id);
        }
    }

    pub fn update(&mut self, line: &String)
    {
        let result: Result<Tick, serde_json::Error> = serde_json::from_str(line);
        if let Ok(json_line) = result
        {
            let side_type = if json_line.side == String::from("BID") { Side::BID } else { Side::ASK };

            if self.instrument == json_line.instrument
            {
                self.date = json_line.date;

                match json_line.quotes
                {
                    QuotesEnum::INCREMENT(quotes) => self.on_increment(side_type,quotes),
                    QuotesEnum::SNAPSHOT(quotes) => self.on_snapshot(side_type, quotes)
                };
            }
            else
            {
                panic!("{}\n\tFound instrument:    {}\n\tInstrument required: {}\n", INSTRUMENT_MISMATCH, json_line.instrument, self.instrument);
            }
        }
        else
        {
            panic!("{}\nFound line:\n=================================\n{}\n=================================\n", BAD_STRING, line);
        }
    }

    pub fn update_from_string(&mut self, lines: String)
    {
        let lines: Vec<String> = lines.split("\n").map(|x| x.to_string()).collect();
        for line in &lines
        {
            self.update(line);
        }
        self.clean();
    }

    pub fn from_file(filename: &str, instrument: String, price_step: f64) -> OrderBook
    {
        let mut orderbook = OrderBook::new(instrument, price_step);
        let path = Path::new(filename);
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
                orderbook.update(&line);
            }
        }
        orderbook.clean();
        orderbook
    }

    pub fn from_str(json: &str, instrument: String, price_step: f64) -> OrderBook
    {
        let mut orderbook = OrderBook::new(instrument, price_step);
        let lines: Vec<String> = json.split("\n").map(|x| x.to_string()).collect();

        for line in &lines
        {
            orderbook.update(line);
        }
        orderbook.clean();
        orderbook
    }

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

    pub fn print(&self)
    {
        println!("Order book for {} at {}", self.instrument, self.date);
        self.print_side(Side::BID);
        self.print_side(Side::ASK);
        print!("\n");
    }

    pub fn to_l2(&self, side: Side) -> Vec<(i64, f64)>
    {
        let book_side = match side {
            Side::ASK => &self.ask,
            Side::BID => &self.bid
        };
        let mut l2book_side: Vec<(i64, f64)> = book_side.iter().map(|(&k, v)| (k, v.iter().fold(0., |acc, x| acc + (x.size)))).collect();
        if side == Side::BID
        {
            l2book_side.reverse();
        }
        l2book_side
    }

    fn feature_println(features: Vec<String>, file: &mut File)
    {
        file.write_all(features.join("\t").as_bytes()).expect("TODO: panic message");
        file.write_all("\n".as_bytes()).expect("TODO: panic message");
    }

    fn calculator(l2side: Vec<(i64, f64)>) -> Vec<(f64, i64)>
    {
        let mut features_pq: Vec<(f64, i64)> = Vec::new();
        if !l2side.is_empty()
        {
            // Calculate P = P(Q)
            let mut total_volume: f64 = 0.0;
            for (price, volume) in &l2side
            {
                total_volume += volume;
                features_pq.push((total_volume, *price));
            }
        }
        features_pq
    }

    pub fn calculate_features(filename: &str, instrument: String, price_step: f64)
    {
        /*
            * This function calculates features for the orderbook and outputs them in multiple .npy files.
            * 1. Price by volume P(Q);
         */
        let mut orderbook = OrderBook::new(instrument, price_step);
        let path = Path::new(filename);
        let display = path.display();

        let file = match File::open(&path) {
            Err(why) => panic!("Couldn't open {}: {}", display, Error::to_string(&why)),
            Ok(file) => file,
        };
        let reader = BufReader::new(file);
        let lines = reader.lines();

        let mut pq_bid_file = File::create(format!("{}_PQ_bid.tsv", filename)).unwrap();
        let mut pq_ask_file = File::create(format!("{}_PQ_ask.tsv", filename)).unwrap();

        let mut i: i64 = 0;
        for result in lines
        {
            if let Ok(line) = result
            {
                orderbook.update(&line);
                orderbook.clean();
            }
            if i % 2 == 1
            {
                let l2_bid: Vec<(i64, f64)> = orderbook.to_l2(Side::BID);
                let l2_ask: Vec<(i64, f64)> = orderbook.to_l2(Side::ASK);

                let (features_pq_bid, features_pq_ask) = (OrderBook::calculator(l2_bid), OrderBook::calculator(l2_ask));
                if i % 100 == -1
                {
                    OrderBook::feature_println(features_pq_bid.iter().map(|x| String::from(format!("{:?}", x))).collect(), &mut pq_ask_file);
                    OrderBook::feature_println(features_pq_ask.iter().map(|x| String::from(format!("{:?}", x))).collect(), &mut pq_bid_file);
                }
            }
            i+=1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn orderbook_create_from_str()
    {
        /*
         * This test checks that the orderbook is created correctly
         */
        let price_step: f64 = 0.0025;
        let price_step_inv: f64 = 400.0;

        let info: String = format!("{}\n{}", r#"{"instrument":"BTCUSD","type":"INCREMENT","date":"2019-01-01T00:00:00.000Z","side":"BID","quotes":{"added":[{"id":1,"price":4000.0,"size":1.05}],"changed":[],"deleted":[]}}"#, r#"{"instrument":"BTCUSD","type":"INCREMENT","date":"2019-01-01T00:00:00.000Z","side":"ASK","quotes":{"added":[{"id":2,"price":5000.0,"size":1.10}],"changed":[],"deleted":[]}}"#);
        let info: &str = &info;

        let mut bid_expected: BTreeMap<i64, VecDeque<L3Quote>> = BTreeMap::new();
        let mut ask_expected: BTreeMap<i64, VecDeque<L3Quote>> = BTreeMap::new();

        bid_expected.insert((4000.0*price_step_inv) as i64, vec![L3Quote {
            id: 1,
            price: 4000.0,
            size: 1.05,
        }].into_iter().collect());

        ask_expected.insert((5000.0*price_step_inv) as i64, vec![L3Quote {
            id: 2,
            price: 5000.0,
            size: 1.10,
        }].into_iter().collect());

        let orderbook = OrderBook::from_str(info, "BTCUSD".to_string(), price_step);

        assert_eq!(orderbook.instrument, "BTCUSD".to_string());
        assert_eq!(orderbook.price_step, price_step);
        assert_eq!(orderbook.price_step_inv, price_step_inv);
        assert_eq!(orderbook.date, "2019-01-01T00:00:00.000Z".to_string());
        assert_eq!(orderbook.bid, bid_expected);
        assert_eq!(orderbook.ask, ask_expected);
    }

    #[test]
    fn orderbook_update_from_str()
    {
        /*
         * This test checks that the orderbook is updated correctly
         */
        let price_step: f64 = 0.0025;
        let price_step_inv: f64 = 400.0;

        let mut bid_expected: BTreeMap<i64, VecDeque<L3Quote>> = BTreeMap::new();
        let mut ask_expected: BTreeMap<i64, VecDeque<L3Quote>> = BTreeMap::new();
        let mut orderbook = OrderBook::new("BTCUSD".to_string(), price_step);

        let info1: &str = r#"{"instrument":"BTCUSD","type":"INCREMENT","date":"2019-01-01T00:00:00.000Z","side":"BID","quotes":{"added":[{"id":1,"price":4001.0,"size":1.05}],"changed":[],"deleted":[]}}"#;
        let info2: &str = r#"{"instrument":"BTCUSD","type":"INCREMENT","date":"2019-01-01T00:00:00.000Z","side":"ASK","quotes":{"added":[{"id":2,"price":5002.0,"size":1.10}],"changed":[],"deleted":[]}}"#;

        bid_expected.insert((4001.0*price_step_inv) as i64, VecDeque::from([L3Quote{id: 1, price: 4001.0, size: 1.05}]));
        ask_expected.insert((5002.0*price_step_inv) as i64, VecDeque::from([L3Quote{id: 2, price: 5002.0, size: 1.10}]));

        orderbook.update_from_string(info1.to_string());
        orderbook.update_from_string(info2.to_string());

        assert_eq!(orderbook.instrument, "BTCUSD".to_string());
        assert_eq!(orderbook.price_step, price_step);
        assert_eq!(orderbook.price_step_inv, price_step_inv);
        assert_eq!(orderbook.date, "2019-01-01T00:00:00.000Z".to_string());
        assert_eq!(orderbook.bid, bid_expected);
        assert_eq!(orderbook.ask, ask_expected);

        let info3: &str = r#"{"instrument":"BTCUSD","type":"INCREMENT","date":"2019-01-01T00:00:00.000Z","side":"BID","quotes":{"added":[{"id":3,"price":4003.0,"size":1.05}],"changed":[],"deleted":[]}}"#;
        let info4: &str = r#"{"instrument":"BTCUSD","type":"INCREMENT","date":"2019-01-01T00:00:00.000Z","side":"ASK","quotes":{"added":[{"id":4,"price":5004.0,"size":1.10}],"changed":[],"deleted":[]}}"#;

        bid_expected.insert((4003.0*price_step_inv) as i64, VecDeque::from([L3Quote{id: 3, price: 4003.0, size: 1.05}]));
        ask_expected.insert((5004.0*price_step_inv) as i64, VecDeque::from([L3Quote{id: 4, price: 5004.0, size: 1.10}]));

        orderbook.update_from_string(info3.to_string());
        orderbook.update_from_string(info4.to_string());

        assert_eq!(orderbook.instrument, "BTCUSD".to_string());
        assert_eq!(orderbook.price_step, price_step);
        assert_eq!(orderbook.price_step_inv, price_step_inv);
        assert_eq!(orderbook.date, "2019-01-01T00:00:00.000Z".to_string());
        assert_eq!(orderbook.bid, bid_expected);
        assert_eq!(orderbook.ask, ask_expected);

        let info5: &str = r#"{"instrument":"BTCUSD","type":"INCREMENT","date":"2019-01-01T00:00:00.000Z","side":"BID","quotes":{"added":[],"changed":[{"id":3,"price":4003.0,"size":1.06}],"deleted":[]}}"#;
        let info6: &str = r#"{"instrument":"BTCUSD","type":"INCREMENT","date":"2019-01-01T00:00:00.000Z","side":"ASK","quotes":{"added":[],"changed":[{"id":4,"price":5004.0,"size":1.11}],"deleted":[]}}"#;

        bid_expected.entry((4003.0*price_step_inv) as i64).and_modify(|v| v[0].size = 1.06);
        ask_expected.entry((5004.0*price_step_inv) as i64).and_modify(|v| v[0].size = 1.11);

        orderbook.update_from_string(info5.to_string());
        orderbook.update_from_string(info6.to_string());

        assert_eq!(orderbook.instrument, "BTCUSD".to_string());
        assert_eq!(orderbook.price_step, price_step);
        assert_eq!(orderbook.price_step_inv, price_step_inv);
        assert_eq!(orderbook.date, "2019-01-01T00:00:00.000Z".to_string());
        assert_eq!(orderbook.bid, bid_expected);
        assert_eq!(orderbook.ask, ask_expected);

        orderbook.print();
    }

    #[test]
    fn orderbook_create_snapshot()
    {
        /*
         * This test checks that the orderbook is created correctly from a non-incremental message (SNAPSHOT)
         */
        let info: &str = r#"{"instrument":"BTCUSD","type":"SNAPSHOT","date":"2019-01-01T00:00:00.000Z","side":"BID","quotes":[{"id":1,"price":4000.0,"size":1.05},{"id":3,"price":4003.0,"size":1.05}]}"#;
        let _ = OrderBook::from_str(info, "BTCUSD".to_string(), 0.0025);
    }

    #[test]
    fn orderbook_features()
    {
        let info: &str = r#"{"instrument":"BTCUSD","type":"SNAPSHOT","date":"2019-01-01T00:00:00.000Z","side":"BID","quotes":[{"id":1,"price":4000.0,"size":1.00},{"id":3,"price":4003.0,"size":1.05}]}"#;
        let orderbook = OrderBook::from_str(info, "BTCUSD".to_string(), 0.0025);
        let l2_bid: Vec<(i64, f64)> = orderbook.to_l2(Side::BID);
        let features_pq = OrderBook::calculator(l2_bid);
        let features_pq_expected = vec![(1.05, 1601200i64), (2.05, 1600000i64)];

        for i in 0..2
        {
            assert_eq!(features_pq_expected[i], features_pq[i]);
        }
    }

    #[test]
    #[should_panic]
    fn orderbook_create_from_incorrect_json()
    {
        /*
         * This test should panic since 'instrument' is misspelled as 'instrumeent'
         */
        let info: &str = r#"{"instrumeent":"BTCUSD","type":"INCREMENT","date":"2019-01-01T00:00:00.000Z","side":"BID","quotes":{"added":[{"id":1,"price":4000.0,"size":1.05}],"changed":[],"deleted":[]}}"#;
        let _ = OrderBook::from_str(info, "BTCUSD".to_string(), 0.0025);
    }

    #[test]
    #[should_panic]
    fn orderbook_snapshot_from_overlapped_bid_ask()
    {
        /*
         * This test checks that the orderbook is not created from the overlapped bid/ask
         */
        let info = concat!(r#"{"instrument":"BTCUSD","type":"SNAPSHOT","date":"2019-01-01T00:00:00.000Z","side":"BID","quotes":[{"id":1,"price":5000.0,"size":1.05},{"id":3,"price":4003.0,"size":1.05}]}"#,
        "\n",
        r#"{"instrument":"BTCUSD","type":"SNAPSHOT","date":"2019-01-01T00:00:00.000Z","side":"ASK","quotes":[{"id":2,"price":5002.0,"size":1.10},{"id":3,"price":4003.0,"size":1.10}]}"#);
        let ob = OrderBook::from_str(info, "BTCUSD".to_string(), 0.0025);
        ob.check_bid_ask_overlap();
    }
}
