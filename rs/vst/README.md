## Introduction

Glicol should now be able to run on every DAW:
https://youtu.be/yFKH9ou_XyQ

Modified on top of:
https://github.com/DGriffin91/egui_baseview_test_vst2

## Todo

- [ ] Support input, so that you can live coding an effect
- [ ] Work on MIDI i/o

## Usage: macOS (Tested on M1; need to test on previous models)
Run `sudo zsh scripts/macos-build-and-install.sh`
> For M1 users, run `sudo zsh scripts/m1.sh`
Start your DAW, test the plugin

## Usage: Windows (Untested)
Run `cargo build`
Copy `target/debug/glicol_vst.dll` to your VST plugin folder
Start your DAW, test the plugin

## Usage: Linux (Untested)
Run `cargo build`
Copy `target/debug/glicol_vst.so` to your VST plugin folder
Start your DAW, test the plugin