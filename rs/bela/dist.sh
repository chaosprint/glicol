set -e
export PATH=$PATH:`pwd`/arm-bela-linux-gnueabihf/bin

NAME="glicol"
RUSTFLAGS="-C target-cpu=cortex-a8" cargo build --target=armv7-unknown-linux-gnueabihf --example $NAME
scp target/armv7-unknown-linux-gnueabihf/debug/examples/$NAME dist/

NAME="glicol32"
RUSTFLAGS="-C target-cpu=cortex-a8" cargo build --target=armv7-unknown-linux-gnueabihf --example $NAME
scp target/armv7-unknown-linux-gnueabihf/debug/examples/$NAME dist/

NAME="glicol64"
RUSTFLAGS="-C target-cpu=cortex-a8" cargo build --target=armv7-unknown-linux-gnueabihf --example $NAME
scp target/armv7-unknown-linux-gnueabihf/debug/examples/$NAME dist/

NAME="glicol16"
RUSTFLAGS="-C target-cpu=cortex-a8" cargo build --target=armv7-unknown-linux-gnueabihf --example $NAME
scp target/armv7-unknown-linux-gnueabihf/debug/examples/$NAME dist/