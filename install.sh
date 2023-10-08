cargo build --release
mkdir -p $HOME/.japanki
rm /usr/lcoal/bin/japanki 2> /dev/null
scp ./target/release/japanki /usr/local/bin
scp ./.japanki/* ~/.japanki
