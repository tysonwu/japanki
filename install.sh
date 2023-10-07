cargo build --release
mkdir -p $HOME/.japanki
scp ./target/release/japanki /usr/local/bin
scp ./.japanki/* ~/.japanki
