# ShanksBot-RS

A rust implementation of the ShanksBot shown by [Matt Parker](https://standupmaths.com/) in [this video](https://www.youtube.com/watch?v=DmfxIhmGPP4).
It counts the number of decimals of a reciprocal of a given prime* number before they start repeating (watch the video for more details).

*it is supposed to be used on prime numbers, but any positive integer will work.

## How to use

You'll need to have a [rust toolchain installed](https://www.rust-lang.org/tools/install). If you don't need the source code you can simply run ```cargo install shanksbot-rs```. Then you can use it from the command line like ```shanks 60017```.
If you prefer running it straight from source code, clone the repository, ```cd``` into the project directory and run ```cargo run -- 60017```. The result will be printed to ```stdout```.

## What's the practical use of this?
There is none that I know of. It was just a fun little exercise. While watching the video.