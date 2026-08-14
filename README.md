-- --

<div align="center">
    <img src="https://socialify.git.ci/IwakiLeKiwi/rll/image?custom_language=Rust&language=1&name=1&pattern=Circuit+Board&theme=Dark" alt="rll" width="640" height="320" />
</div>

-- --

<div align="center">

<p>

![Status](https://img.shields.io/badge/Status-In%20Development-blue?style=for-the-badge&logo=github)
![Last commit](https://img.shields.io/github/last-commit/IwakiLeKiwi/rll?style=for-the-badge&logo=github&color=orange)
![Stars](https://img.shields.io/github/stars/IwakiLeKiwi/rll?style=for-the-badge&logo=github&color=yellow)

</p>

<p>

![Language](https://img.shields.io/badge/Made%20With-Rust-bf8563?style=for-the-badge&logo=rust)
![Compilation](https://img.shields.io/badge/Compiled%20With-Cargo-black?style=for-the-badge&logo=rust)
![Author](https://img.shields.io/badge/Author-Iwaki-red?style=for-the-badge&logo=opsgenie)
![License](https://img.shields.io/badge/Licence-MIT-green?style=for-the-badge&logo=gitbook)


</p>

</div>

## ⚠️ Important information
> [!NOTE]
> I am a Rust novice and this is my first project in the language. I am learning Rust while building this library, so the code may not always follow best practices. Feedback and suggestions are more than welcome!

## 📖 About - Rust Launcher Lib (rll)

**Rust Launcher Lib (`rll`)** is a Rust library designed to simplify the development of Minecraft launchers.

It aims to provide the core functionality needed to install and manage Minecraft, including version metadata, libraries, assets, Java installations, game launching, and authentication.

> [!WARNING]
>  README in progress, more information and how to use in the future.<br>
> `rll` is currently in early development

## 🦀 Project Status

### Minecraft Installation
- [ ] Vanilla Minecraft installation
  - [ ] Minecraft libraries download
  - [ ] Minecraft assets download
  - [ ] Minecraft game files download

### Java

- [ ] Java version detection
- [ ] Java download & installation
  
### Game Launching & Authentication
- [ ] Game launch
  - [ ] JVM arguments 
- [ ] Microsoft authentication
  
### Mod Loaders

- [ ] Mod loader installation :
  - [ ] Forge
  - [ ] Fabric
  - [ ] Neoforge

## 🚀 Usage

### Install files (Vanilla)

First, to launch the game, we need an `Updater` object.
```rust
let mut updater = Updater::new("1.20.1");
```
Then, specify the location of the installation directory.<br>
On Windows, the default location is `%APPDATA%`, while on Linux it is usually `$HOME`.
```rust
updater.set_relative_local_dir_path(".rll");
```
Finally, we can install the game files:
```rust
updater.install_files();
```

### Launch

> [!IMPORTANT]
> Todo

## 📦 Dependencies

Here are the main Rust crates used in this project:
* [`reqwest`](https://crates.io/crates/reqwest)
* [`serde`](https://crates.io/crates/serde)
* [`tokio`](https://crates.io/crates/tokio)

See [`Cargo.toml`](./Cargo.toml) for the complete list of dependencies.
  
## 💡 Inspiration

This project is inspired by [`FlowUpdater`](https://github.com/FlowArg/FlowUpdater), a great library used to make launchers in Java.

## 📃 License

See the [`LICENSE`](./LICENSE) file for the complete license text.