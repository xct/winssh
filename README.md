# winssh

Spawns a ssh server on windows. No installation required.
Based on https://github.com/PowerShell/Win32-OpenSSH/

## usage

```
winssh.exe --port <port>
```

On every build new keys will be generated. After starting the server you can use the "key" from the files directory.

## compile

```
rustup target add x86_64-pc-windows-gnu
rustup toolchain install stable-x86_64-pc-windows-gnu
```

Windows:
```
cargo build --release --target x86_64-pc-windows-gnu
```

To reduce the filesize further, you can strip the binaries with `strip`.
