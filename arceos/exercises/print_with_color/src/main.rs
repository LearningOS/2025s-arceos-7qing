#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]

#[cfg(feature = "axstd")]
use axstd::printlnwithcolor;

#[cfg_attr(feature = "axstd", no_mangle)]
fn main() {
    printlnwithcolor!("[WithColor]: Hello, Arceos!");
}
