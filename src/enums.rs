/// Status of a single symbol (for example BTCUSD)
pub enum SymbolStatus {
    /// TODO: Document
    PreTrading,
    /// TODO: Document
    Trading,
    /// TODO: Document
    PostTrading,
    /// TODO: Document
    EndOfDay,
    /// TODO: Document
    Halt,
    /// TODO: Document
    AuctionMatch,
    /// Symbol's trading status that represents the symbol is not available for trading, which can happen during expected downtime. Market data is not generated during BREAK.
    /// <https://github.com/binance/binance-spot-api-docs/blob/master/faqs/spot_glossary.md>
    Break,
}

///
pub enum SymbolPermissions {
    /// Spot Trading
    Spot,
    /// Margin Trading
    Margin,
    /// Leveraged Trading
    Leveraged,
    //TODO: Contact Binance Support to ask, what the different values mean
    TrdGrp002,
    TrdGrp003,
    TrdGrp004,
    TrdGrp005,
    TrdGrp006,
    TrdGrp007,
    TrdGrp008,
    TrdGrp009,
    TrdGrp010,
    TrdGrp011,
    TrdGrp012,
    TrdGrp013,
    TrdGrp014,
    TrdGrp015,
    TrdGrp016,
    TrdGrp017,
    TrdGrp018,
    TrdGrp019,
    TrdGrp020,
    TrdGrp021,
    TrdGrp022,
    TrdGrp023,
    TrdGrp024,
    TrdGrp025,
}

/// Status of a single order
pub enum OrderStatus {
    New,
    PendingNew,
    PartiallyFilled,
    Filled,
    Canceled,
    PendingCancel,
    Rejected,
    Expired,
    ExpiredInMatch,
}

pub enum OrderListStatus {
    Response,
    ExecStarted,
    AllDone,
}

pub enum OrderListOrderStatus {
    Executing,
    AllDone,
    Reject,
}

pub enum ContingencyType {
    OCO,
    OTO,
}

pub enum AllocationType {
    SOR,
}

pub enum OrderType {
    Limit,
    Market,
    StopLoss,
    StopLossLimit,
    TakeProfit,
    TakeProfitLimit,
    LimitMaker,
}

pub enum OrderResponseType {
    Ack,
    Result,
    Full,
}

pub enum WorkingFloor {
    Exchange,
    Sor,
}

pub enum OrderSide {
    Buy,
    Sell,
}

pub enum TimeInForce {
    Gtc,
    Ioc,
    Fok,
}

// TODO: Should the rate limiter be implemented locally or should we just wait for the error
// response to know, when the limit is reached
pub enum RateLimiter {
    RequestWeight,
    Orders,
    RawRequests,
}

pub enum StpModes {
    None,
    ExpireMaker,
    ExpireTaker,
    ExpireBoth,
}
