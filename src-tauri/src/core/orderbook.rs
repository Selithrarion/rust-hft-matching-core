use serde::{Deserialize, Serialize};
use std::cmp::{min, Reverse};
use std::collections::VecDeque;
use std::time::{SystemTime, UNIX_EPOCH};

const MAX_TRADE_HISTORY: usize = 10000;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Debug, Clone)]
pub struct Order {
    pub id: u64,
    pub price: u64,
    pub amount: u64,
    pub side: Side,
    pub timestamp_received: u128,
}

#[derive(Debug, Clone, Serialize)]
pub struct Trade {
    // pub taker_order_id: u128,
    // pub maker_order_id: u128,
    pub price: u64,
    pub amount: u64,
    pub timestamp: u128,
    #[serde(rename = "side")]
    pub taker_side: Side,
}

#[derive(Debug, Clone, Serialize)]
pub struct OrderBookLevelSnapshot {
    price: u64,
    amount: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct OrderBookSnapshot {
    bids: Vec<OrderBookLevelSnapshot>,
    asks: Vec<OrderBookLevelSnapshot>,
    timestamp: u128,
}

#[derive(Debug, Clone)]
struct OrderBookLevel {
    price: u64,
    orders: Vec<Order>,
}

#[derive(Debug, Clone)]
pub struct OrderBook {
    // highest to lowest
    bids: Vec<OrderBookLevel>,
    // lowest to highest
    asks: Vec<OrderBookLevel>,
    trades: VecDeque<Trade>,
}

impl OrderBook {
    pub fn new() -> Self {
        OrderBook {
            bids: Vec::new(),
            asks: Vec::new(),
            trades: VecDeque::new(),
        }
    }

    pub fn get_latest_trades(&self, limit: usize) -> Vec<Trade> {
         self.trades.iter().rev().take(limit).cloned().collect()
     }

    pub fn add_trades_to_history(&mut self, trades: impl IntoIterator<Item = Trade>) {
        self.trades.extend(trades);
        if self.trades.len() > MAX_TRADE_HISTORY {
            let to_remove = self.trades.len() - MAX_TRADE_HISTORY;
            self.trades.drain(..to_remove);
        }
    }

    pub fn get_snapshot(&self, depth: usize) -> OrderBookSnapshot {
        let bids = self
            .bids
            .iter()
            .map(|level| OrderBookLevelSnapshot {
                price: level.price,
                amount: level.orders.iter().map(|o| o.amount).sum(),
            })
            .take(depth)
            .collect();

        let asks = self
            .asks
            .iter()
            .map(|level| OrderBookLevelSnapshot {
                price: level.price,
                amount: level.orders.iter().map(|o| o.amount).sum(),
            })
            .take(depth)
            .collect();

        OrderBookSnapshot {
            bids,
            asks,
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis(),
        }
    }

    pub fn order_count(&self) -> usize {
        self.bids.iter().map(|l| l.orders.len()).sum::<usize>() + self.asks.iter().map(|l| l.orders.len()).sum::<usize>()
    }

    pub fn add_limit_order(&mut self, mut order: Order, out_trades: &mut Vec<Trade>) {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();

        match order.side {
            Side::Buy => {
                while let Some(best_ask_level) = self.asks.get_mut(0) {
                    if best_ask_level.price > order.price {
                        break;
                    }

                    while let Some(maker_order) = best_ask_level.orders.get_mut(0) {
                        let trade_amount = min(order.amount, maker_order.amount);
                        let trade = Trade {
                            amount: trade_amount,
                            price: best_ask_level.price,
                            taker_side: Side::Buy,
                            timestamp
                        };
                        out_trades.push(trade);

                        maker_order.amount -= trade_amount;
                        order.amount -= trade_amount;

                        if maker_order.amount == 0 {
                            best_ask_level.orders.remove(0);
                        }

                        if order.amount == 0 {
                            break;
                        }
                    }

                    if best_ask_level.orders.is_empty() {
                        self.asks.remove(0);
                    }

                    if order.amount == 0 {
                        break;
                    }
                }
            }
            Side::Sell => {
                while let Some(best_bid_level) = self.bids.get_mut(0) {
                    if best_bid_level.price < order.price {
                        break;
                    }

                    while let Some(maker_order) = best_bid_level.orders.get_mut(0) {
                        let trade_amount = min(order.amount, maker_order.amount);
                        let trade = Trade {
                            amount: trade_amount,
                            price: best_bid_level.price,
                            taker_side: Side::Sell,
                            timestamp
                        };
                        out_trades.push(trade);

                        maker_order.amount -= trade_amount;
                        order.amount -= trade_amount;

                        if maker_order.amount == 0 {
                            best_bid_level.orders.remove(0);
                        }

                        if order.amount == 0 {
                            break;
                        }
                    }

                    if best_bid_level.orders.is_empty() {
                        self.bids.remove(0);
                    }

                    if order.amount == 0 {
                        break;
                    }
                }
            }
        }

        if order.amount > 0 {
            match order.side {
                Side::Buy => {
                    match self.bids.binary_search_by_key(&Reverse(order.price), |l| Reverse(l.price)) {
                        Ok(index) => self.bids[index].orders.push(order),
                        Err(index) => self.bids.insert(index, OrderBookLevel {
                            price: order.price,
                            orders: vec![order],
                        }),
                    }
                }
                Side::Sell => {
                    match self.asks.binary_search_by_key(&order.price, |l| l.price) {
                        Ok(index) => self.asks[index].orders.push(order),
                        Err(index) => self.asks.insert(index, OrderBookLevel {
                            price: order.price,
                            orders: vec![order],
                        }),
                    }
                }
            }
        }
    }
}

impl Default for OrderBook {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_and_match_orders() {
        let mut book = OrderBook::new();
        let ts = 0;

        let sell_order = Order { id: 1, price: 100, amount: 10, side: Side::Sell, timestamp_received: ts };
        let mut trades_buffer = Vec::new();
        book.add_limit_order(sell_order, &mut trades_buffer);
        assert!(trades_buffer.is_empty());
        assert_eq!(book.asks[0].orders.len(), 1);

        let buy_order = Order { id: 2, price: 101, amount: 5, side: Side::Buy, timestamp_received: ts};
        book.add_limit_order(buy_order, &mut trades_buffer);
        let trades = &trades_buffer;
        assert_eq!(trades.len(), 1);
        let trade = &trades[0];
        assert_eq!(trade.price, 100);
        assert_eq!(trade.amount, 5);

        let remaining_sell = &book.asks[0].orders[0];
        assert_eq!(remaining_sell.amount, 5);
        assert!(book.bids.is_empty());

        let buy_order_2 = Order { id: 3, price: 100, amount: 10, side: Side::Buy, timestamp_received: ts};
        book.add_limit_order(buy_order_2, &mut trades_buffer);
        let trades_2 = &trades_buffer;
        assert_eq!(trades_2.len(), 1);
        assert_eq!(trades_2[0].amount, 5);
        assert!(book.asks.is_empty());

        let remaining_buy = &book.bids[0].orders[0];
        assert_eq!(remaining_buy.amount, 5);
    }
}