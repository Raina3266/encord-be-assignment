cd $(git rev-parse --show-toplevel)

cd tetris_code
cargo build --release
cd ..

cp tetris_code/target/release/tetris .