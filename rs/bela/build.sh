NAME="glicol"

export PATH=$PATH:`pwd`/arm-bela-linux-gnueabihf/bin
cargo build --target=armv7-unknown-linux-gnueabihf --example $NAME
scp target/armv7-unknown-linux-gnueabihf/debug/examples/$NAME root@bela.local:~
ssh root@bela.local