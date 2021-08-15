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
$ velas-keygen new -o /path/to/keypairs/player1.json
$ velas-keygen new -o /path/to/keypairs/player2.json
```
Note: you need fo fund these wallets with some coins in order to pay transactions fees

## Install dependencies
Install required npm dependencies using command
```bash
$ npm install
```

## Build on-chain program and deploy it
Build game with rust compiler using this command:
```bash
npm run build:program-rust
```

Deploy it:
```bash
velas program deploy dist/program/helloworld.so
```

## Initializing and playing the game
First you need to initialize new game with following command, signed by first player:
```
$ npm start game-reset <second_player> /path/to/keypairs/player1.json
```

Replace `<second_player>` with base58 encoded public key of second player before executing the command above. You can use following command to show base58 encoded pubkey from specified keypair file:
```bash
$ npm start show-key /path/to/keypairs/player2.json
```

After game initialization, take a look at this line at console output (pubkey will be different). Save this public key, we will need to pass it as argument of commands below.
```
Creating game account: 65FuAgjgSGKqZs6cWBLvKgqd3oD5DPbwvVhDRf7RZos5
```

After the game has been initialized, we can check its state (don't forget to replace game account pubkey with your own here and after):
```bash
$ npm start game-state 65FuAgjgSGKqZs6cWBLvKgqd3oD5DPbwvVhDRf7RZos5
```

Game field shoud be empty, pubkeys of players authorized to play should be set up (pubkeys of players will be different). The status says that it is now the turn of the first player:
```
Game field:
* * *
* * *
* * *
Player one: Bk4khFDudVzm58AffvxBvLN1ALefhYMqc3hdh1fST7hs
Player two: 5hdGEibS3vj3C8gLJh4b2inW12tzMWpGCjuBAjFKYqfr
Status: PlayerOneTurn
```

Let's make a move, signed by first player:
```bash
$ npm start make-turn 65FuAgjgSGKqZs6cWBLvKgqd3oD5DPbwvVhDRf7RZos5 0 1 /path/to/keypairs/player1.json
```
Here `0 1` means that we want to put a mark at point at row = 0, column = 1. The coordinate system starts from the top left corner.

After the first move, we could check the game state again:
```bash
$ npm start game-state 65FuAgjgSGKqZs6cWBLvKgqd3oD5DPbwvVhDRf7RZos5
```

```
Game field:
* X *
* * *
* * *
Player one: Bk4khFDudVzm58AffvxBvLN1ALefhYMqc3hdh1fST7hs
Player two: 5hdGEibS3vj3C8gLJh4b2inW12tzMWpGCjuBAjFKYqfr
Status: PlayerTwoTurn
```

Now, second player could make his turn. Note: currently both players should play from the same PC.
```bash
$ npm start make-turn 65FuAgjgSGKqZs6cWBLvKgqd3oD5DPbwvVhDRf7RZos5 1 1 /path/to/keypairs/player2.json
```

Check the game state again:
```bash
$ npm start game-state 65FuAgjgSGKqZs6cWBLvKgqd3oD5DPbwvVhDRf7RZos5
```

```
Game field:
* X *
* 0 *
* * *
Player one: Bk4khFDudVzm58AffvxBvLN1ALefhYMqc3hdh1fST7hs
Player two: 5hdGEibS3vj3C8gLJh4b2inW12tzMWpGCjuBAjFKYqfr
Status: PlayerOneTurn
```
