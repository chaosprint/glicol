# Glicol on Bela

This crate helps Glicol engine to run on [Bela board](https://bela.io).

It is based on:
https://github.com/andrewcsmith/bela-rs
https://github.com/padenot/bela-sys

Thus, this version only works for OSX and Linux.
> I have only tested on M1 Mac.

## Setup

Install the right tool chain for the Beaglebone black:

```sh
rustup target add armv7-unknown-linux-gnueabihf
rustup toolchain install stable-armv7-unknown-linux-gnueabihf
```

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

Then, on the bela board
```sh
./glicol
```

## TODO

- [ ] Support ADC
- [ ] Provide `glicol_synth` example
- [ ] Provide high level wrapper for Glicol engine
- [ ] Optimise file size