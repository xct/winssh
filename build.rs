use std::process::Command;
use std::fs;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    Command::new("sh").arg("-c")
        .arg("rm files/h* files/k*")
        .status()
        .unwrap();
    Command::new("sh").arg("-c")
        .arg("yes 'y' 2>/dev/null | ssh-keygen -t ed25519 -f files/key -q -N \"\"")
        .status()
        .unwrap();
    Command::new("sh").arg("-c")
        .arg("yes 'y' 2>/dev/null | ssh-keygen -t ed25519 -f files/key_reverse -q -N \"\"")
        .status()
        .unwrap();
     Command::new("sh").arg("-c")
        .arg("cp files/key.pub files/authorized_keys")
        .status()
        .unwrap();
    Command::new("sh").arg("-c")
        .arg("yes 'y' 2>/dev/null | ssh-keygen -f host_dsa -N '' -t dsa -f files/host_dsa -q -N \"\"")
        .status()
        .unwrap();
    Command::new("sh").arg("-c")
        .arg("yes 'y' 2>/dev/null | ssh-keygen -f host_rsa -N '' -t rsa -f files/host_rsa -q -N \"\"")
        .status()
        .unwrap();
}
