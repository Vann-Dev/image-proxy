![banner](https://image-proxy.vannapps.com/594648694b4c34633737437966616e453063366874756c6e7858397a47676a536967305666322b314e34594848776175566569466c49345946433759336335576b414d526f6839546c61616230735671546974612f413d3d)

# Rust Image Proxy

What is this?, this is repository used for encrypting or hiding real source for image, for example in banner here, the real source is from `https://content.vannapps.com/public/banner-image-proxy.png` then using this repository we can encrypt it to hex with secret key 🤫, then we can use it like this `https://image-proxy.vannapps.com/594648694b4c34633737437966616e453063366874756c6e7858397a47676a536967305666322b314e34594848776175566569466c49345946433759336335576b414d526f6839546c61616230735671546974612f413d3d`.

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

`https://image-proxy.vannapps.com/55587938484333356f4d50384851584857426a416e37524d514f6454456769624849532b6838675279632b6e31497339387a7a3072473953357a5a6847695339`