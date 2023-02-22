
<h1 align="center">
  <br>
  <a href="http://www.ricardodarocha.com.br"><img src="https://styles.redditmedia.com/t5_2s7lj/styles/communityIcon_pjg3ktzyju771.png" alt="Rust" width="200"></a>
  <br>
  Rust
  <br>
</h1>

<h4 align="center">About  </p>
An example of using bitflags - boolean operations between bits, the variable as integer (usize)



<p align="center">
  <a href="#introdução">Introduction</a> •
  <a href="#como-usar">How to use</a> •
  <a href="#download">Download</a> •
  <a href="#credits">Credits</a> •
  <a href="#related">Related</a> •
  <a href="#license">License</a>
</p>

![screenshot](img/screenshot.gif)

## Introduction

You can have bit operations between two numbers (usize) using the native cast to binary {:0b0}

**Example**
```rust

//Marca o enésimo bit como true
fn fill(num: usize, offset: u8) -> usize {
  if offset > 8 {panic!("Passou do limite")};
    num | (1 << offset)
}

//Verifica se o enésimo bit é true
fn is_filled(num: usize, offset: u8) -> bool {
    (num & (1 << offset)) != 0
}

```

## How to use

Clone this repository [Git](https://github.com/ricardodarocha/rust-bitflags.git)   
Run `cargo check cargo run`

```bash
# Clone this repository
$ git clone https://github.com/ricardodarocha/rust-bitflags.git

# Go into the repository
$ cd rust-bitflags

# Open with VSCode
$ code .

# install and run
$ cargo check
$ cargo run 
```

## Download

No binary yet

## Credits

This software uses pure Rust language

## Resources

bit operations
Operator Overloading to implement BitOr for a struct
Operator Overloading to implement BitOr for usize


## Related

[Rust](https://www.rust-lang.org/pt-BR) - A linguagem mais querida 🦀

## Contato

> Linkedin [ricardo-da-rocha-vitor](https://www.linkedin.com/in/ricardo-da-rocha-vitor-a0983932/)
> Site [ricardodarocha.com.br](https://www.ricardodarocha.com.br) &nbsp;&middot;&nbsp;
> GitHub [@ricardodarocha](https://github.com/ricardodarocha) &nbsp;&middot;&nbsp;
> Twitter [@ricardorochadev](https://twitter.com/ricardorochadev)


## You may also like...

- [Actix-Web](https://actix.rs/) 
- [Axum](https://docs.rs/axum/latest/axum/)
- [Tokio](https://github.com/tokio-rs)
- [async-std](https://async.rs/)


## License

GNU


---


