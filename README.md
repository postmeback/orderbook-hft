# 📚 OrderBook-HFT

A basic **Order Book** engine implementation for a **High-Frequency Trading (HFT)** system using **Rust**.  
Supports **limit orders** (buy/sell), **trade matching**, and **trade recording**.

---

## 🚀 Build and Run Instructions

1. **Clone the repository**
   ```bash
   git clone https://github.com/postmeback/orderbook-hft.git
   cd orderbook-hft
   ```

2. **Build the project**
   ```bash
   cargo build --release
   ```

3. **Run the project**
   ```bash
   cargo run
   ```

4. **Run tests**
   - Run all tests:
     ```bash
     cargo test
     ```
   - Run a specific test file (e.g., `order_book_test.rs`):
     ```bash
     cargo test --test order_book_test
     ```

---

## 📂 Project Structure

```text
orderbook-hft/
├── src/
│   ├── main.rs          # Entry point (example usage)
│   ├── lib.rs           # Exposes modules for external access
│   └── models/
│       ├── order.rs     # Order struct
│       └── order_book.rs# OrderBook struct and matching logic
|       ├── trade.rs #after successfull trade
├── tests/
│   └── order_book_test.rs # Integration tests
├── Cargo.toml
└── README.md
```

---

## 📋 Assumptions Made

- **Single-threaded** design for simplicity (no concurrency or locking).
- **Price-Time Priority** matching: best price first, earliest order first.
- **Price stored as `f64`** wrapped with `OrderedFloat<f64>`.
- **Timestamp** captured using `SystemTime::now()`.
- **No partial fills** — full order quantity matching only.
- **No cancellation or modification** — basic add & match system.

---

## 🛠 Dependencies & Environment

| Dependency      | Purpose                          | Version |
|-----------------|----------------------------------|---------|
| `ordered-float` | To safely store floats in maps   | `3.9`   |
| `chrono`        | (Optional) For datetime handling | `0.4`   |
| `Rust Compiler` | Language compiler                | `>=1.78`|

> Install Rust: [https://rustup.rs](https://rustup.rs)

---

## ⚙️ How Matching Works

- **Buy Orders**: Matched with the lowest priced sell orders.
- **Sell Orders**: Matched with the highest priced buy orders.
- **Data Structure**:
  - Prices are stored in `BTreeMap` for automatic sorting.
  - Orders at the same price level are stored in a `VecDeque` (FIFO queue).

---

## 📌 Design Decisions Explained

- **BTreeMap**: Automatically maintains sorted prices needed for fast matching.
- **VecDeque**: Efficient front removal and back insertion (queue behavior).
- **OrderedFloat**: Needed because `f64` by itself cannot be compared or sorted in Rust.
- **lib.rs**: Exposes internal modules (`models`) to external integration tests cleanly.
- **Separate model files**: `order.rs` and `order_book.rs` for clean separation of concerns and easier testing.

---

## 📈 Future Enhancements

- Support **partial fills** (match smaller quantities).
- Implement **order cancellation** and **modification**.
- Introduce **concurrency** using `tokio` for performance.


---


## 📣 Contact

If you have any questions or suggestions, feel free to reach out!  
Happy trading! 🚀

---

