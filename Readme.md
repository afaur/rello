# Test example
The `pencil` dependency should be set to the `closures` branch.
```
cd deps
rm -rf pencil
git clone git@github.com:afaur/pencil.git
cd pencil
git checkout closures
cd ../..
cargo run
open http://localhost:5000/somename
```

# Rello - A hello world webapp in Pencil/Rust

Your typical curl / -> "200 Hello World" app, but in Rust using the Pencil framework. Deploys easily on Heroku.

## Build

If you have Rust and openssl installed properly

    $ cargo build

On my OS X 10.11 machine, I have to:

    $ brew install multirust openssl
    $ multirust default stable
    $ multirust update
    $ env OPENSSL_INCLUDE_DIR=$(brew --prefix openssl)"/include" OPENSSL_LIB_DIR=$(brew --prefix openssl)"/lib" cargo build

## Run

    $ PORT=8000 cargo run

## Deploy to Heroku

[![Deploy](https://www.herokucdn.com/deploy/button.svg)](https://heroku.com/deploy)

or

    $ git clone https://github.com/archaelus/rello.git
    $ cd rello
    $ heroku create --buildpack https://github.com/Hoverbear/heroku-buildpack-rust.git
    $ git push heroku master
