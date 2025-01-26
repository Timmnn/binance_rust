use binance_rust::{
    make_request,
    rest_endpoints::{Endpoint, MarketDataKlineReqBody},
};

#[tokio::main]
async fn main() {
    let _ = make_request(Endpoint::Ping).await;

    match make_request(Endpoint::MarketDataKline(
        MarketDataKlineReqBody {
            symbol: "TRUMPUSDC".to_string(),
            interval: "1m".to_string(),
            ..MarketDataKlineReqBody::default()
        },
        0,
    ))
    .await
    {
        Ok(()) => println!("All good"),
        Err(e) => println!("ERR {}", e),
    };
}
