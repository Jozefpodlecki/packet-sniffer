# Packet Sniffer

A simple Rust tool to capture and analyze network packets with flexible output handlers.

## Build

```sh
cargo build --release
```

## Examples

Dumping packets to bin

```sh
./target/release/packet-sniffer.exe --source windivert --handler raw
```

Reading from bin

```sh
./target/release/packet-sniffer.exe --source file --handler tracker
```