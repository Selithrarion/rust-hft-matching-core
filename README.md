# Rust HFT Matching Engine

![Terminal UI](/screenshots/main.webp)

**initial stats**: crossbeam 2 channels, btree for order storage, 100ms for ui update, default allocator. (cargo debug, not release)  
Round-trip Latency: ~2-10ms  
Core Latency:  
50us for Orders/sec: 700, Trades/sec: 700  
50us for Orders/sec: 7000, Trades/sec: 7000  
150us for Orders/sec: 70000, Trades/sec: 70000  

✅after replacing btree with sorted vec + binary search to improve cache locality  
Core Latency:  
~20-30us with 70us spikes for O/s: 70000 (same for 700, 7000)  
~300us for O/s: 350000

❌an attempt to reduce lock contention  
separate book copy for ui increased latency to 1000us

❌an attempt to pin the order processor thread to a single CPU core  
regression ~150us for 700 O/s

❌an attempt to replace `Vec<Order>` with `VecDeque<Order>`  
lower stability, higher spikes

❌an attempt to use an `arena allocator (bumpalo)`  
didn't help or regressed. seems default allocations already extremely optimized and we have enough memory in user-space  
or maybe we have to allocate a lot of memory manually?

❌an attempt to replace `orders.remove(0)` with virtual `head_index`  
regression 300us -> 1000us with spikes up to 5000  
seems `.drain()` is less optimized or smth

✅replaced `SystemTime::now()` for id generation with `AtomicU64`  
~100-200us with spikes up to 500us  
(so it was ~300us before)  

✅ replaced `Vec::new()` with reusable global mut `trade_buffer` to reduce allocations in the hot path  
still ~100-200us but now there are lots of dips to 30-60us (tho still spikes up to 500us)  

**FINAL**  
✅UI update interval set to 5s to test lock contention hypothesis  
~15-50µs with occasional spikes up to ~300µs for O/s: 350,000.  
have to try again a separate ui channel ig but lazy  
maximum O/s ~600k - maybe have to optimize structures further

but a production build ~4000us ???  
TODO: maybe have to replace `SystemTime::now`

soooo... next level would be programming FPGA (wanted to buy 20$ Tang Nano 9K or smth to test it)  
really interested in it woah!!!

---

![Chart UI](/screenshots/chart.webp)

## dev
if you wanna compile it yourself or add more features:  
you need `rust`, `nodejs` and `pnpm` (also c++ build tools and webview support for tauri)  

**stack:** vuejs + rust + tauri

```bash
pnpm dev  # run everything
pnpm build # or prod build
