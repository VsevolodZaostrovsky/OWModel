mod orderbook;

use crate::orderbook::{OrderBook};
use tqdm::tqdm;


fn main()
{
    let filenames_rub = vec![ "data/USD_RUB_T+1__2022-10-03",
                                        "data/USD_RUB_T+1__2022-10-04",
                                        "data/USD_RUB_T+1__2022-10-05",
                                        "data/USD_RUB_T+1__2022-10-06",
                                        "data/USD_RUB_T+1__2022-10-07",
                                        "data/USD_RUB_T+1__2022-10-10",
                                        "data/USD_RUB_T+1__2022-10-11",
                                        "data/USD_RUB_T+1__2022-10-12",
                                        "data/USD_RUB_T+1__2022-10-13",
                                        "data/USD_RUB_T+1__2022-10-14",
                                        "data/USD_RUB_T+1__2022-10-17",
                                        "data/USD_RUB_T+1__2022-10-18",
                                        "data/USD_RUB_T+1__2022-10-19",
                                        "data/USD_RUB_T+1__2022-10-20",
                                        "data/USD_RUB_T+1__2022-10-21",
                                        "data/USD_RUB_T+1__2022-10-24",
                                        "data/USD_RUB_T+1__2022-10-25",
                                        "data/USD_RUB_T+1__2022-10-26",
                                        "data/USD_RUB_T+1__2022-10-27",
                                        "data/USD_RUB_T+1__2022-10-28",
                                        "data/USD_RUB_T+1__2022-10-31",
                                        "data/USD_RUB_T+1__2022-11-01",
                                        "data/USD_RUB_T+1__2022-11-02",
                                        "data/USD_RUB_T+1__2022-11-03",
                                        "data/USD_RUB_T+1__2022-11-07",
                                        "data/USD_RUB_T+1__2022-11-08",
                                        "data/USD_RUB_T+1__2022-11-09",
                                        "data/USD_RUB_T+1__2022-11-10",
                                        "data/USD_RUB_T+1__2022-11-11",];
    let filenames_chn = vec!["data/USD_CNH_T+1__2022-10-04", "data/USD_CNH_T+1__2022-10-20", "data/USD_CNH_T+1__2022-11-01", "data/USD_CNH_T+1__2022-11-15"];

    for filename in tqdm(filenames_chn.iter())
    {
        OrderBook::calculate_features(filename, "USD/CNH_T+1".to_string(), 0.0025);
    }

    for filename in tqdm(filenames_rub.iter())
    {
        OrderBook::calculate_features(filename, "USD/RUB_T+1".to_string(), 0.0025);
    }
}