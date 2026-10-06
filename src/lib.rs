//! # RustLauncherLib
//! 
//! A library for building Minecraft launchers in Rust.
//! 
//! It is organized around three steps:
//! 
//! - [`update`]: fetch version metadata from Mojang and download the game
//!   files (client, libraries and assets).
//! 
//! > **Status:** early development. Only `update` module is being
//! > implemented for now.

pub mod update;

#[cfg(test)]
mod test {

    #[test]
    pub fn test() {
        println!("Hello from RustLauncherLib!");
    }
}