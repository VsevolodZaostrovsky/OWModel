mod orderbook;

use crate::orderbook::{OrderBook};
use tqdm::tqdm;


fn main()
{
    let filenames_rub = vec![ "data/DataDays/USD_RUB_T+1__2022-10-03",
                                        "data/DataDays/USD_RUB_T+1__2022-10-04",
                                        "data/DataDays/USD_RUB_T+1__2022-10-05",
                                        "data/DataDays/USD_RUB_T+1__2022-10-06",
                                        "data/DataDays/USD_RUB_T+1__2022-10-07",
                                        "data/DataDays/USD_RUB_T+1__2022-10-10",
                                        "data/DataDays/USD_RUB_T+1__2022-10-11",
                                        "data/DataDays/USD_RUB_T+1__2022-10-12",
                                        "data/DataDays/USD_RUB_T+1__2022-10-13",
                                        "data/DataDays/USD_RUB_T+1__2022-10-14",
                                        "data/DataDays/USD_RUB_T+1__2022-10-17",
                                        "data/DataDays/USD_RUB_T+1__2022-10-18",
                                        "data/DataDays/USD_RUB_T+1__2022-10-19",
                                        "data/DataDays/USD_RUB_T+1__2022-10-20",
                                        "data/DataDays/USD_RUB_T+1__2022-10-21",
                                        "data/DataDays/USD_RUB_T+1__2022-10-24",
                                        "data/DataDays/USD_RUB_T+1__2022-10-25",
                                        "data/DataDays/USD_RUB_T+1__2022-10-26",
                                        "data/DataDays/USD_RUB_T+1__2022-10-27",
                                        "data/DataDays/USD_RUB_T+1__2022-10-28",
                                        "data/DataDays/USD_RUB_T+1__2022-10-31",
                                        "data/DataDays/USD_RUB_T+1__2022-11-01",
                                        "data/DataDays/USD_RUB_T+1__2022-11-02",
                                        "data/DataDays/USD_RUB_T+1__2022-11-03",
];

    OrderBook::calculate_features("data/USD_RUB_T+1__2022-10-03", "USD/RUB_T+1".to_string(), 0.0025);


    
    //for filename in tqdm(filenames_rub.iter())
    //{
    //    OrderBook::calculate_features(filename, "USD/RUB_T+1".to_string(), 0.0025);
    //}

}