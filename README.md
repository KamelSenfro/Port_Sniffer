my 1st try mit ***Rust*** 

A port sniffer CLI made with pure STD rust.
Use `cargo run` `cargo build` to either run or build


A multi-threaded port scanner written in Rust. This tool allows you to scan a range of ports on a specified IP address to check if they are open. It supports various features such as verbose output, saving results to a file, and specifying the number of concurrent threads.

## Features

- **Multi-threaded Scanning**: Specify the number of concurrent threads to speed up the scanning process.
- **Customizable Port Range**: Define the start and end ports for scanning.
- **Verbose Output**: Get detailed information about each port scanned.
- **Timeout Configuration**: Set a timeout for each connection attempt.
- **Save Results to a File**: Option to save the scan results to a file.
- **Protocol Specification**: (Currently only supports TCP)

## Usage

### Installation

1. Ensure you have Rust installed. If not, you can install it from [rust-lang.org](https://www.rust-lang.org/tools/install).
2. Clone this repository:
    ```sh
    git clone https://github.com/yourusername/rust-port-sniffer.git
    ```
3. Change into the project directory:
    ```sh
    cd rust-port-sniffer
    ```
4. Build the project:
    ```sh
    cargo build --release
    ```

### Running the Port Sniffer

You can run the port sniffer with various options:

```sh
./target/release/rust-port-sniffer [OPTIONS]
