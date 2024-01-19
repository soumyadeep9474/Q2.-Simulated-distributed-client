# 2. Simulated distributed client
 
Extend the solution to Q1 by instantiating 5 client processes and one aggregator process.


a. All client processes start at the same tick of the time, say 10:01:01 AM.


b. Client process read values from the websocket for 10 seconds and computes the average and
sends it to the aggregator process.


c. Aggregator process waits for the average values from all the 5 processes. Upon getting all the
values it computes yet another average and displays on the screen.

--------------------------------------------------------------------------------------






To run the Rust program with the given code, you need to use the cargo run command in the terminal. Here is an example command line assuming you have the Rust toolchain installed:

```
cargo run -- --clients 5
```
## Explanation:

cargo run: This command is used to compile and run a Rust project.
--: Separates the cargo command-line arguments from the binary's command-line arguments.
--clients 5: This is an argument passed to your program. It specifies the number of clients to run. You can adjust the value (in this case, 5) based on your requirements.
Make sure to run this command in the directory where your main.rs file is located and where your Cargo.toml file is present.

This assumes you have the necessary dependencies installed, such as tokio, tokio-tungstenite, serde, serde_json, clap, and futures-util. If you don't have them installed, you can do so by running:

```
cargo build
```
This command will download and compile the dependencies specified in your Cargo.toml file. After that, you can use cargo run to execute your program.
