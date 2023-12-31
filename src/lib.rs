// The `lib.rs` file serves as the main entry point for the library. It declares
// modules and makes them available for use in other parts of the library or by
// other crates that depend on this library.

// The `utils` module contains utility functions used across the library.
// It is not public, meaning it's intended for internal use within this crate.
mod utils;

// The `mel` module provides the core functionality for the shifting cipher.
// It's kept private as its functions are accessed through the public interfaces
// provided by `mel_912`, `mel_presets`, and `mel_mdp`.
mod mel;

// The `mel_912` module is publicly exposed and provides functionality for
// encrypting or decrypting text with a variable shifting cipher.
pub mod mel_912;

// The `mel_presets` module offers a set of preset shift values for encryption
// and decryption, simplifying the use of the `mel` function with common shifts.
pub mod mel_presets;

// The `mel_mdp` (mel with password) module provides a public interface for
// encrypting or decrypting text using a password-based shifting mechanism.
pub mod mel_mdp;
