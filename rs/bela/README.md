# Glicol on Bela

This crate helps Glicol engine to run on [Bela board](https://bela.io).

It is based on:

https://github.com/andrewcsmith/bela-rs

https://github.com/padenot/bela-sys

Thus, this version only works for OSX and Linux.
> I have only tested on M1 Mac.

## Setup

### Step 1

```sh
rustup target add armv7-unknown-linux-gnueabihf
rustup toolchain install stable-armv7-unknown-linux-gnueabihf
```
> For non-Rust programmers, you should have [Rust](https://www.rust-lang.org/) installed on your computer!

### Step 2

`git clone` this whole repo, not just this folder.

I suggest you fork it first in case you wanna contribute.

### Step 3

With a bela board plugged in and accessible at `http://bela.local`, run:

```sh
./setup.sh
```

> On Mac, you may need to run `sudo zsh setup.sh`

This downloads the right linker, pulls in some required files from the board,
and sets up the `$PATH` environment variable. This MUST be called in each
terminal session that will be used to call `cargo`, but will only download the
files once.

> You can change the `setup.sh` file mannualy when there is any missing files in this process. This may be due to some updates on the Bela dependancies.

## Testing

```sh
./build.sh
```
> On Mac, you may need to run `sudo zsh build.sh`

This will:
- build the `glicol.rs` in the examples
- copy the binary file to Bela board
- `ssh` into the Bela board

> If you see that linker cannot be found in building, try to run the command in `linker.sh` manually in terminal. Then call the `build.sh` again.

Then, on the bela board, there are three ways to get sound:

### Usage 1: no param, thus a hello tone
```sh
./glicol
```

### Usage 2: Input glicol code
This will play a sawtooth osc whose freq is modulated by adc3:
```sh
./glicol "o: saw ~mod; ~mod: ~adc3 >> mul 110 >> add 220"
```

### Usage 3: Read a .glicol file
The content of `hello.glicol` is identical to the second manual input.
```sh
./glicol -- hello.glicol
```

> These are just POC, and will be changed soon.

## TODO

- [x] Support ADC
- [ ] More params for `./glicol` command such as num_analog_in 
- [ ] Live coding?
- [ ] Optimise file size