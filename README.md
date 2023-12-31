# My Encryption Library

## Overview

My Encryption Library is a Rust crate that provides various encryption and decryption algorithms. It's designed to be easy to use, yet flexible enough to handle a range of encryption needs.

## Features

- Shift-based encryption and decryption (similar to the Caesar cipher).
- Support for accented characters and special symbols.
- Different encryption presets for ease of use.
- Password-based encryption for enhanced security.

## Installation

Add this to your `Cargo.toml`:

```
[dependencies]
my_encryption_lib = "0.1.0"
```

## Usage

Here's a quick example to get you started:

```
use my_encryption_lib::mel_presets;

fn main() {
    let encrypted = mel_presets::mel_5("Hello, world!", true).unwrap();
    let decrypted = mel_presets::mel_5(&encrypted, false).unwrap();

    println!("Encrypted: {}", encrypted);
    println!("Decrypted: {}", decrypted);
}
```

Replace `mel_5` with any other function from the library to use a different encryption algorithm.

## Contributing

1. Fork the repository.
2. Create a new branch (`git checkout -b feature-branch`).
3. Make your changes.
4. Commit your changes (`git commit -am 'Add some feature'`).
5. Push to the branch (`git push origin feature-branch`).
6. Create a new Pull Request.

## License

This project is licensed under **MIT license**.

## Contact

For any queries, feel free to reach out on **GitHub**.
