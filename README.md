# Linux
### build
cargo build --release --features openssl-sys
### or
cargo build --target x86_64-unknown-linux-musl --release  --features openssl-sys
### run
./npm-login-cli.exe -u user -p user -n https://npm-url/repository/npm-public/
### Help
./npm-login-cli --help


# Windows
### build
cargo build --release
### run
.\npm-login-cli.exe -u user -p user -n https://npm-url/repository/npm-public/
### Help
.\npm-login-cli.exe --help
