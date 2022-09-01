set -e
NAME="glicol"

scp target/armv7-unknown-linux-gnueabihf/debug/examples/$NAME root@bela.local:~
scp _main.glicol root@bela.local:~
ssh root@bela.local