use super::OrderBook;

#[derive(Debug, Clone)]
pub struct MarketData {
    pub exchange: String,
    pub order_book: OrderBook,
}