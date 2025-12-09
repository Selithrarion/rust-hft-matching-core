use criterion::{criterion_group, criterion_main, Criterion};
use rand::Rng;
use rand::prelude::IndexedRandom;
use app_lib::core::orderbook::{Order, OrderBook, Side};

fn generate_random_orders(count: usize) -> Vec<Order> {
    let mut rng = rand::rng();
    let sides = [Side::Buy, Side::Sell];
    let mut orders = Vec::with_capacity(count);

    for i in 0..count {
        orders.push(Order {
            id: i as u128,
            price: rng.random_range(90_000..110_000),
            amount: rng.random_range(1..100),
            side: *sides.choose(&mut rng).unwrap(),
        });
    }

    orders
}

fn bench_add_1m_orders(c: &mut Criterion) {
    let orders = generate_random_orders(1_000_000);

    c.bench_function("add_1m_limit_orders", |b| {
        b.iter(|| {
            let mut orderbook = OrderBook::new();
            for order in std::hint::black_box(&orders) {
                orderbook.add_limit_order(order.clone());
            }
        })
    });
}

criterion_group!(benches, bench_add_1m_orders);
criterion_main!(benches);