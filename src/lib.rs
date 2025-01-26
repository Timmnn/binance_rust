pub mod enums;
pub mod rest_endpoints;
use std::collections::HashMap;

#[derive(Debug, serde::Deserialize)]
struct Kline {
    open_time: u64,                       // Timestamp of the open time
    open: String,                         // Opening price
    high: String,                         // Highest price
    low: String,                          // Lowest price
    close: String,                        // Closing price
    volume: String,                       // Volume of trades
    close_time: u64,                      // Timestamp of the close time
    quote_asset_volume: String,           // Volume of the quote asset
    number_of_trades: u32,                // Number of trades
    taker_buy_base_asset_volume: String,  // Taker buy base asset volume
    taker_buy_quote_asset_volume: String, // Taker buy quote asset volume
    ignore: String,                       // Unused field
}

pub async fn make_request(req: rest_endpoints::Endpoint) -> Result<(), Box<dyn std::error::Error>> {
    print!("");
    println!("Requesting {:?}", req);

    match &req {
        rest_endpoints::Endpoint::Ping => {
            let resp = reqwest::get("https://api.binance.com".to_owned() + req.as_str())
                .await?
                .json::<HashMap<String, String>>()
                .await?;
            println!("{resp:#?}");
            Ok(())
        }
        rest_endpoints::Endpoint::MarketDataKline(params, _) => {
            let query_string = serde_urlencoded::to_string(&params).unwrap();

            let url = "https://api.binance.com".to_owned() + req.as_str() + "?" + &query_string;

            println!("{}", url);

            let resp = reqwest::get(url)
                .await
                .unwrap()
                .json::<Vec<Kline>>()
                .await?;

            println!("{resp:#?}");

            Ok(())
        }
    }
}
