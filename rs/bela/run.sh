# make sure glicol is already on Bela!

set -e
NAME="glicol"
scp _main.glicol root@bela.local:~
ssh -t root@bela.local "./glicol -- _main.glicol"