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
    from_file("../data/OrderLog20200302.txt", "EUR000TODTOM".to_string(), 0.001); 
    from_file("../data/OrderLog20200302.txt", "EURUSDTODTOM".to_string(), 0.001);
    from_file("../data/OrderLog20200302.txt", "GBPRUBTODTOM".to_string(), 0.001);
    from_file("../data/OrderLog20200302.txt", "KZTRUB_TOM".to_string(), 0.001);
    from_file("../data/OrderLog20200302.txt", "CHFRUB_TOM".to_string(), 0.001);
    from_file("../data/OrderLog20200302.txt", "TRYRUB_TOM".to_string(), 0.001);
    from_file("../data/OrderLog20200302.txt", "EUR_RUB__TOD".to_string(), 0.001);
    from_file("../data/OrderLog20200302.txt", "CHFRUB_TOD".to_string(), 0.001);
    from_file("../data/OrderLog20200302.txt", "USDRUB_TMB".to_string(), 0.001);
    from_file("../data/OrderLog20200302.txt", "EURRUB_TDB".to_string(), 0.001);
    from_file("../data/OrderLog20200302.txt", "GLDRUB_TOM".to_string(), 0.001);
    from_file("../data/OrderLog20200302.txt", "CNYRUB_TOM".to_string(), 0.001);
    from_file("../data/OrderLog20200302.txt", "USDRUB_TOM9M".to_string(), 0.001);
    from_file("../data/OrderLog20200302.txt", "USDRUB_TOM1Y".to_string(), 0.001);
    from_file("../data/OrderLog20200302.txt", "USDRUB_TOM2M".to_string(), 0.001);
    from_file("../data/OrderLog20200302.txt", "GBPUSD_TOM".to_string(), 0.001);
    from_file("../data/OrderLog20200302.txt", "CNYRUB_TOM1M".to_string(), 0.001);
    from_file("../data/OrderLog20200302.txt", "EURRUB_TMB".to_string(), 0.001);
    from_file("../data/OrderLog20200302.txt", "USDRUB_TOM1D".to_string(), 0.001);
    from_file("../data/OrderLog20200302.txt", "USDCHF_TOM".to_string(), 0.001);
    from_file("../data/OrderLog20200302.txt", "BYNRUB_TOD".to_string(), 0.001);
    from_file("../data/OrderLog20200302.txt", "JPYRUBTODTOM".to_string(), 0.001);
    from_file("../data/OrderLog20200302.txt", "GLDRUB_TOM1W".to_string(), 0.001);
    from_file("../data/OrderLog20200302.txt", "CNYRUB_TOM1D".to_string(), 0.001);
    from_file("../data/OrderLog20200302.txt", "USDRUB_TDB".to_string(), 0.001);
    from_file("../data/OrderLog20200302.txt", "USDRUB_TOM6M".to_string(), 0.001);
    from_file("../data/OrderLog20200302.txt", "JPYRUB_TOM".to_string(), 0.001);
    from_file("../data/OrderLog20200302.txt", "GBPRUB_TOD".to_string(), 0.001);
    from_file("../data/OrderLog20200302.txt", "EURUSD000TOD".to_string(), 0.001);
    from_file("../data/OrderLog20200302.txt", "HKDRUB_TOM".to_string(), 0.001);
    from_file("../data/OrderLog20200302.txt", "USD000TODTOM".to_string(), 0.001);
    from_file("../data/OrderLog20200302.txt", "USDRUB_TOM1W".to_string(), 0.001);
    from_file("../data/OrderLog20200302.txt", "USD000UTSTOM".to_string(), 0.001);
    from_file("../data/OrderLog20200302.txt", "EUR_RUB__TOM".to_string(), 0.001);
    from_file("../data/OrderLog20200302.txt", "TRYRUB_TOD".to_string(), 0.001);
    from_file("../data/OrderLog20200302.txt", "USDRUB_TOM2W".to_string(), 0.001);
    from_file("../data/OrderLog20200302.txt", "CHFRUBTODTOM".to_string(), 0.001);
    from_file("../data/OrderLog20200302.txt", "KZTRUB_TOD".to_string(), 0.001);
    from_file("../data/OrderLog20200302.txt", "EURUSD000TOM".to_string(), 0.001);
    from_file("../data/OrderLog20200302.txt", "USDRUB_TOM1M".to_string(), 0.001);
    from_file("../data/OrderLog20200302.txt", "KZTRUBTODTOM".to_string(), 0.001);
    from_file("../data/OrderLog20200302.txt", "SLVRUB_TOM".to_string(), 0.001);
    from_file("../data/OrderLog20200302.txt", "USD000000TOD".to_string(), 0.001);
    from_file("../data/OrderLog20200302.txt", "JPYRUB_TOD".to_string(), 0.001);
    from_file("../data/OrderLog20200302.txt", "BYNRUB_TOM".to_string(), 0.001);
    from_file("../data/OrderLog20200302.txt", "GBPRUB_TOM".to_string(), 0.001);
    from_file("../data/OrderLog20200302.txt", "CNY000000TOD".to_string(), 0.001);
    from_file("../data/OrderLog20200302.txt", "USDRUB_TOM3M".to_string(), 0.001);
    from_file("../data/OrderLog20200302.txt", "HKDRUB_TOD".to_string(), 0.001);
    from_file("../data/OrderLog20200302.txt", "CNYRUBTODTOM".to_string(), 0.001);
    from_file("../data/OrderLog20200302.txt", "CNYRUB_TOM1W".to_string(), 0.001);
}
