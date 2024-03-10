# 95Gen 🔑

This is a Windows 95 + Office 95 Keygen based on public methods, all displayed in a simple UI made in Rust.

## Advantages

- Low CPU Usage
- Generates unlimited keys
- Verification

## How does it work?

Sample key: AAABB-OEM-0CCCCCC-DDDDD  
The first 3 digits are any number from 1-366, signifying a day of the year.  
The next 2 are a year, anything from 95-03  
The C section is any number when added up, is divisible by zero, its last digit cannot be 0 or >= 8.  
The D section is random numbers.  

## Run

```bash
cargo run
```

## Build

```bash
cargo build
```

## Disclaimer

This software, referred to as 95Gen, is a project based on public methods and a simple algorithm designed for educational and recreational purposes only. The primary goal is to explore and understand basic key generation concepts. Windows 95 is essentially abandonware, as there are no future updates planned for the OS.

## License

[License](LICENSE)
