# Blockchain App

This is a simple blockchain application implemented in Rust. It demonstrates the basic concepts of a blockchain, including creating blocks, adding them to a chain, and validating the integrity of the blockchain.

## Project Structure

- **Cargo.toml**: Project configuration file.
- **src/main.rs**: Main Rust file containing the blockchain logic.

## Features

- **Block Creation**: Create new blocks with a timestamp, previous hash, and data.
- **Blockchain Initialization**: Initialize a blockchain with a genesis block.
- **Adding Blocks**: Add new blocks to the blockchain.
- **Blockchain Validation**: Validate the integrity of the blockchain.

## Getting Started

### Prerequisites

- Rust: Install Rust from [here](https://www.rust-lang.org/tools/install).

### Installation

1. Clone the repository:
    ```sh
    git clone https://github.com/your-username/blockchain_app.git
    cd blockchain_app
    ```

2. Build and run the project:
    ```sh
    cargo run
    ```

### Usage

The application initializes a blockchain with a genesis block and adds two more blocks with sample data. It then validates the blockchain and prints the entire chain.

Example output:
```
Blockchain valid: true
Block { index: 0, timestamp: ..., previous_hash: "0", hash: "...", data: "Genesis Block" }
Block { index: 1, timestamp: ..., previous_hash: "...", hash: "...", data: "Block 1 Data" }
Block { index: 2, timestamp: ..., previous_hash: "...", hash: "...", data: "Block 2 Data" }
```


### Project Structure

- **Block Struct**: Represents a single block in the blockchain.
    - `index`: The position of the block in the chain.
    - `timestamp`: The time when the block was created.
    - `previous_hash`: The hash of the previous block.
    - `hash`: The current block's hash.
    - `data`: The data stored in the block.

- **Blockchain Struct**: Represents the entire blockchain.
    - `new()`: Initializes a new blockchain with a genesis block.
    - `add_block(data: String)`: Adds a new block to the blockchain.
    - `is_valid()`: Validates the integrity of the blockchain.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Rust Programming Language: [https://www.rust-lang.org/](https://www.rust-lang.org/)
- Serde: [https://serde.rs/](https://serde.rs/)
- SHA-2: [https://docs.rs/sha2/](https://docs.rs/sha2/)
- Chrono: [https://docs.rs/chrono/](https://docs.rs/chrono/)


