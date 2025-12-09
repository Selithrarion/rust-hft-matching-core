#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod core;
mod error;

use std::sync::{Arc, Mutex};
use crate::core::orderbook::{Order, OrderBook, Side};
use std::thread;
use rand::prelude::IndexedRandom;
use rand::Rng;
use serde::{Serialize, Deserialize};
use tauri::{AppHandle, State, Emitter};
use crossbeam_channel::{unbounded, Receiver, Sender};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

static ORDER_ID_COUNTER: AtomicU64 = AtomicU64::new(1);

#[derive(Serialize, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum ConnectionStatus {
    Connected,
}

#[derive(Serialize, Clone, Deserialize, Debug)]
struct LatencyUpdate {
    avg_processing_time_ns: u128,
}


pub struct AppState {
    pub order_tx: Sender<Order>,
    pub book_handle: Arc<Mutex<OrderBook>>,
}

#[tauri::command]
fn ping() -> &'static str {
    "pong"
}

#[tauri::command]
fn get_connection_status() -> ConnectionStatus {
    ConnectionStatus::Connected
}

#[tauri::command]
fn place_order(
    price: f64,
    amount: f64,
    side: Side,
    state: State<AppState>,
) -> Result<(), String> {
    let order_id = ORDER_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
    let price_cents = (price * 100.0).round() as u64;
    let amount_base_units = (amount * 10000.0).round() as u64;
    let timestamp_received = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();

    let order = Order {
        id: order_id,
        price: price_cents,
        amount: amount_base_units,
        side,
        timestamp_received,
    };
    state.order_tx.send(order).map_err(|e| e.to_string())
}

fn run_market_simulation(order_tx: Sender<Order>) {
    thread::spawn(move || {
        let mut rng = rand::rng();
        let sides = [Side::Buy, Side::Sell];
        let mut last_price: f64 = 3000.0;

        loop {
            for _ in 0..rng.random_range(10000..20000) {
                let order_id = ORDER_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
                let price_f64 = last_price + rng.random_range(-0.5..0.51);
                let price_cents = (price_f64 * 100.0).round() as u64;
                let timestamp_received = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
                last_price = price_f64;

                let order = Order {
                    id: order_id,
                    price: price_cents,
                    amount: rng.random_range(1..500) * 100,
                    timestamp_received,
                    side: *sides.choose(&mut rng).unwrap(),
                };
                order_tx.send(order).unwrap();
            }
            thread::sleep(Duration::from_millis(20));
        }
    });
}

fn run_orderbook_processor(
    book_handle: Arc<Mutex<OrderBook>>,
    app_handle: AppHandle,
    order_rx: Receiver<Order>,
) {
    thread::spawn(move || {
        let metrics_interval = Duration::from_secs(1);
        let mut last_metrics_update = Instant::now();
        let mut orders_processed: u64 = 0;
        let mut trades_executed: u64 = 0;
        let mut total_processing_time_ns: u128 = 0;
        let mut latency_measurements: u64 = 0;
        let mut trade_buffer = Vec::with_capacity(200);

        while let Ok(first_order) = order_rx.recv() {
            let mut book = book_handle.lock().unwrap();

            let mut orders_batch = vec![first_order];
            orders_batch.extend(order_rx.try_iter());
            orders_processed += orders_batch.len() as u64;

            for order in orders_batch {
                let received_ts = order.timestamp_received;
                book.add_limit_order(order, &mut trade_buffer);
                let processing_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() - received_ts;
                total_processing_time_ns += processing_time;
                latency_measurements += 1;
            }

            trades_executed += trade_buffer.len() as u64;
            book.add_trades_to_history(trade_buffer.iter().cloned());
            trade_buffer.clear();

            if last_metrics_update.elapsed() >= metrics_interval {
                let avg_latency = if latency_measurements > 0 {
                    total_processing_time_ns / latency_measurements as u128
                } else {
                    0
                };

                app_handle.emit("latency_update", LatencyUpdate { avg_processing_time_ns: avg_latency }).unwrap();

                println!(
                    "[METRICS] Orders/sec: {}, Trades/sec: {}, Avg Latency: {} ns",
                    orders_processed, trades_executed, avg_latency
                );
                orders_processed = 0;
                trades_executed = 0;
                total_processing_time_ns = 0;
                latency_measurements = 0;
                last_metrics_update = Instant::now();
            }
        }

    });
}

fn run_ui_updater(app_handle: AppHandle, book_handle: Arc<Mutex<OrderBook>>) {
    thread::spawn(move || {
        let ui_update_interval = Duration::from_millis(500);

        loop {
            thread::sleep(ui_update_interval);

            let book = book_handle.lock().unwrap();
            let snapshot = book.get_snapshot(50);
            let new_trades = book.get_latest_trades(100);

            if !new_trades.is_empty() {
                app_handle.emit("new_trades_batch", &new_trades).unwrap();
            }
            app_handle.emit("book_update", snapshot).unwrap();
        }
    });
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let (order_tx, order_rx) = unbounded::<Order>();
    let book_handle = Arc::new(Mutex::new(OrderBook::new()));

    tauri::Builder::default()
        .manage(AppState {
            order_tx: order_tx.clone(),
            book_handle: book_handle.clone(),
        })
        .setup(move |app| {
            let app_handle = app.handle().clone();
            let processor_app_handle = app.handle().clone();
            run_market_simulation(order_tx);
            run_orderbook_processor(book_handle.clone(), processor_app_handle, order_rx);
            run_ui_updater(app_handle, book_handle);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![place_order, ping, get_connection_status])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}