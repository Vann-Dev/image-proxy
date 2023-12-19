![banner](https://image-proxy.vannapps.com/537671423169614552553465736634633877496759545053396571706c544b61794a4644726338337a6630746c574b4d6a35522b7361665476436338615742595a7a654c2f656270467643636e674d6b2f386f6464413d3d)

# Rust Image Proxy

What is this?, this is repository used for encrypting or hiding real source for image, for example in banner here, the real source is from `https://content.vannapps.com/public/banner-image-proxy.png` then using this repository we can encrypt it to hex with secret key 🤫, then we can use it like this `https://image-proxy.vannapps.com/537671423169614552553465736634633877496759545053396571706c544b61794a4644726338337a6630746c574b4d6a35522b7361665476436338615742595a7a654c2f656270467643636e674d6b2f386f6464413d3d`.

## Features
 - Hide real source URL using aes-256-cbc
 - Encrypt using api
 - Written in Rust
 - Easy to setup

## Requirement
 - Cargo & Rust https://doc.rust-lang.org/cargo/getting-started/installation.html

## Cargo & Rust Instalation
 - Windows
 ```
 Open this link https://win.rustup.rs/
 ```
 - macOS & Linux
 ```
 curl https://sh.rustup.rs -sSf | sh
 ```

## How to use? [git clone]
 - Copy this repository `git clone https://github.com/Vann-Dev/image-proxy`
 - Write .env file, the example are in `.env.example`
 - Build the Rust file using `cargo build --release` (for production build) or `cargo build` (for development build)
 - To run the project, run the file inside target folder `/target/release/image-proxy` (for production) or `cargo r` (for development)

## How to use? [Docker]
 - You can pull the image from https://github.com/Vann-Dev/image-proxy/pkgs/container/image-proxy
 - Or just build it by yourself by `git clone` this repository and run `docker build -t image-proxy ./`

## Endpoint
 - GET `/[encrypted url]` Return image with encrypted source
 - POST `/encrypt` Return encrypted hash used for getting image
 ```
 body {
    "url": "https://content.vannapps.com/public/banner-image-proxy.png"
 }
 ```
## Product Example
`https://image-proxy.vannapps.com/`
use this url if you want to encrypt or get example image

`https://image-proxy.vannapps.com/537671423169614552553465736634633877496759545053396571706c544b61794a4644726338337a6630746c574b4d6a35522b7361665476436338615742595a7a654c2f656270467643636e674d6b2f386f6464413d3d`
