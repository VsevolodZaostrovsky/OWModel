mod order_book;
use crate::order_book::OrderBook;


fn from_file(filename: &str, instrument: String, price_step: f64) {
    let temp_orderbook = OrderBook::from_file(filename, instrument, price_step);
    for i in temp_orderbook.instruments{
        println!("{}", i);
    }
}

fn main()
{
    from_file("../data/OrderLog20200302.txt", "TRYRUB_TOM".to_string(), 0.001);
    // from_file("../data/OrderLog20200103.txt", "USD000UTSTOM".to_string(), 0.001);
    // from_file("../data/OrderLog20200302.txt", "SLVRUB_TOM".to_string(), 0.001);
    from_file("../data/OrderLog20200103.txt", "CHFRUB_TOM".to_string(), 0.001);
    from_file("../data/OrderLog20200103.txt", "HKDRUB_TOM".to_string(), 0.001);
    from_file("../data/OrderLog20200103.txt", "CNYRUB_TOM".to_string(), 0.001);
    from_file("../data/OrderLog20200103.txt", "EURUSD000TOM".to_string(), 0.001);
    from_file("../data/OrderLog20200103.txt", "USDRUB_TOM1D".to_string(), 0.001);
    from_file("../data/OrderLog20200103.txt", "EUR_RUB__TOM".to_string(), 0.001);
    from_file("../data/OrderLog20200103.txt", "JPYRUB_TOM".to_string(), 0.001);
}
