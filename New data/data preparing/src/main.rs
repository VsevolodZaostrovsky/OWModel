mod order_book;
use crate::order_book::OrderBook;


fn from_file(filename: &str, instrument: String, price_step: f64) {
    // let temp_orderbook = OrderBook::from_file_time_window(filename, instrument, price_step, &"100005000000".to_string(), &"100015062664".to_string());
    let temp_orderbook = OrderBook::from_file(filename, instrument, price_step);
    for i in temp_orderbook.instruments{
        // println!("{}", i);
    }

    println!("{} parsed", filename);
}

fn main()
{
    // from_file("../data/CU/OrderLog20210303.txt", "USD000UTSTOM".to_string(), 0.001); 

    // from_file("OrderLogEx.txt","EURUSD000TOM".to_string(), 0.001); 
    // from_file("../data/SE/OrderLog20210303.txt","LKOH".to_string(), 0.001); 
    // from_file("../data/SE/OrderLog20210303.txt","GAZP".to_string(), 0.001); 
    // from_file("../data/SE/OrderLog20210303.txt","VTBR".to_string(), 0.001); 
    // from_file("../data/SE/OrderLog20210303.txt","MTLR".to_string(), 0.001); 
    // from_file("../data/SE/OrderLog20210303.txt","MGNT".to_string(), 0.001); 
    // from_file("../data/SE/OrderLog20210303.txt","YNDX".to_string(), 0.001); 
    // from_file("../data/SE/OrderLog20210303.txt","PLZL".to_string(), 0.001); 
    // from_file("../data/SE/OrderLog20210303.txt","SNGSP".to_string(), 0.001); 
    // from_file("../data/SE/OrderLog20210303.txt","ROSN".to_string(), 0.001); 


    // from_file("../data/CU/OrderLog20210303.txt", "USD000000TOD".to_string(), 0.0001);
    // from_file("../data/CU/OrderLog20210303.txt", "USD000UTSTOM".to_string(), 0.0001);

    // from_file("../data/CU/OrderLog20210303.txt", "EUR_RUB__TOD".to_string(), 0.0001);
    // from_file("../data/CU/OrderLog20210303.txt", "EUR_RUB__TOM".to_string(), 0.0001);

    // from_file("../data/CU/OrderLog20210303.txt", "GBPRUB_TOM".to_string(), 0.0001);
    from_file("../data/CU/OrderLog20210303.txt", "GBPRUB_TOD".to_string(), 0.0001);

    from_file("../data/CU/OrderLog20210303.txt", "CNY000000TOD".to_string(), 0.00001);
    from_file("../data/CU/OrderLog20210303.txt", "CNYRUB_TOM".to_string(), 0.00001);

    // from_file("../data/CU_OrderLog20210303.txt", "CNYRUB_TOM".to_string(), 0.00001);
    // from_file("../data/CU_OrderLog20210303.txt", "GBPRUB_TOD".to_string(), 0.0001);
    // // from_file("../data/CU_OrderLog20210303.txt", "J
    // from_file("../data/CU/OrderLog20210303.txt", "USD000UTSTOM".to_string(), 0.0001);

    // from_file("../data/CU/OrderLog20210303.txt", "EUR_RUB__TOD".to_string(), 0.0001);
    // from_file("../data/CU/OrderLog20210303.txt", "EUR_RUB__TOM".to_string(), 0.0001);

    // from_file("../data/CU/OrderLog20210303.txt", "GBPRUB_TOM".to_string(), 0.0001);PYRUB_TOD".to_string(), 0.001);
    // from_file("../data/CU/OrderLog20210303.txt", "GBPRUB_TOM".to_string(), 0.0001);
    // from_file("../data/CU_OrderLog20210303.txt", "CNY000000TOD".to_string(), 0.00001);
    // from_file("../data/CU_OrderLog20210303.txt", "USDRUB_TOM3M".to_string(), 0.001);
    // from_file("../data/CU_OrderLog20210303.txt", "HKDRUB_TOD".to_string(), 0.001);
    // from_file("../data/CU_OrderLog20210303.txt", "CNYRUBTODTOM".to_string(), 0.001);
    // from_file("../data/CU/OrderLog20210303.txt", "CNYRUB_TOM1W".to_string(), 0.00001);   
}
