# Counter Program (Solana + Rust)

A simple Solana smart contract written in Rust that demonstrates how to store and update on-chain state using a counter.  
The program allows users to initialize a counter account and increment the stored value.

---

## Features

- Initialize a counter account on Solana
- Increment the counter value
- Store state on-chain
- Simple example of Solana program structure
- Beginner-friendly Rust + Solana project

---

## Built With

- Rust
- Solana Program Library
- Solana CLI
- Cargo

---

## Installation & Setup

### 1. Install Solana CLI

Follow the official installation guide:

https://docs.solana.com/cli/install-solana-cli-tools

Verify installation:

```bash
solana --version
```

---

### 2. Clone the repository

```bash
git clone https://github.com/your-username/counter-program-solana-using-rust.git
cd counter-program-solana-using-rust
```

---

### 3. Build the program

```bash
cargo build-bpf
```

---

### 4. Deploy the program

```bash
solana program deploy target/deploy/counter_program.so
```

---

## How It Works

1. A counter account is created on-chain.
2. The program initializes the counter value.
3. Each time the increment instruction is called, the counter value increases.
4. The updated value is stored permanently on the Solana blockchain.

---

## 📂 Project Structure

```
counter-program-solana-using-rust/
│
├── src/
│   ├── lib.rs
│   ├── instruction.rs
│   ├── processor.rs
│   └── state.rs
│
├── Cargo.toml
└── README.md
```

---

## Concepts Used

- Solana Accounts
- Program Instructions
- On-chain State Management
- Rust Structs and Enums
- Serialization / Deserialization
- Solana Program Entry Point

---

## Future Improvements

- Add decrement functionality
- Add reset counter instruction
- Write integration tests
- Build a frontend to interact with the program
- Add Anchor version of the program

---

## Contributing

Pull requests are welcome.  
Feel free to fork the project and improve it.
