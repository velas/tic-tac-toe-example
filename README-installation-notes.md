# Installation Notes
if you are a first-time user of Rust, the notes below may help you to install
some of the dependencies on a Mac or Linux workstation.

### Rust
We suggest that you install Rust using the 'rustup' tool. Rustup will install
the latest version of Rust, Cargo, and the other binaries used in Solana.

Follow the instructions at [Installing
Rust](https://www.rust-lang.org/tools/install).

For Mac users, Homebrew is also an option.  The Mac Homebrew command is `brew
install rustup` and then `rustup-init`. See [Mac
Setup](https://sourabhbajaj.com/mac-setup/Rust/) & [Installing
Rust](https://www.rust-lang.org/tools/install) for more details.

After installation, you should have `rustc`, `cargo`, & `rustup`. You should
also have `~/.cargo/bin` in your PATH environment variable.

### NodeJS/NPM
Fetch the `npm` dependencies, by running:
```bash
$ npm install
```

### Git Repository
Clone the 'tic-tac-toe-example' repository into your development machine:
```bash
$ cd /path/to/your/work/folder/
$ git clone https://github.com/velas/tic-tac-toe-example.git
$ cd tic-tac-toe-example
```
(If you plan to submit changes in a pull request, be sure to create a fork first
and then clone your fork.)
