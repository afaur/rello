# Rello - A hello world webapp in Pencil/Rust

Your typical curl / -> "200 Hello World" app, but in Rust using the Pencil framework. Deploys easily on Heroku.

## Build

    $ cargo build

## Run

    $ PORT=8000 cargo run

## Deploy to Heroku

[![Deploy](https://www.herokucdn.com/deploy/button.svg)](https://heroku.com/deploy)

or

    $ git clone https://github.com/archaelus/rello.git
    $ cd rello
    $ heroku create --buildpack https://github.com/Hoverbear/heroku-buildpack-rust.git
    $ git push heroku master
