<p align="center">
  <a href="https://velas.com">
    <img alt="Velas chain" src="https://i.imgur.com/1AHYxYP.png" width="250" />
  </a>
</p>

# Hello world on Velas
This project demonstrates simple example of "tic-tac-toe" game built on the Velas blockchain

The project comprises of:

* An on-chain tic-tac-toe program with game logic
* A client that can initialize new game and play it with other player

## Quick start
The following dependencies are required to build and run this example, depending
on your OS, they may already be installed:

- Install node (v14 recommended)
- Install npm
- Install the latest Rust stable from https://rustup.rs/
- Install Velas Tool Suite from
  https://docs.velas.com/cli/install-velas-cli-tools

If this is your first time using Rust, these [Installation
Notes](README-installation-notes.md) might be helpful.

## Create Keypair for each player
Every player should have a keypair for making move, you can generate keypair using following command

```bash
$ velas-keygen new
```

## Install dependencies
Install required npm dependencies using command
```bash
$ npm install
```

## Build on-chain program and deploy it
Build game with rust compiler using command
```bash
npm run build:program-rust
```

Deploy it
```bash
velas program deploy dist/program/helloworld.so
```

## Initializing and playing the game
TODO:
```
game-reset
make-turn
game state
```
