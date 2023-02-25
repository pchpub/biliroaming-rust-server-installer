use super::http_server::simple_http_server;
use super::request::sync_getwebpage;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::process::Command;

// a function to automatically apply certificate
pub fn apply_cert<T>(cert_path: &T, key_path: &T, website_name: &str) -> Result<(), Box<dyn Error>>
where
    T: AsRef<Path> + AsRef<std::ffi::OsStr>,
{
    if Path::new(cert_path).exists() {
        Command::new("rm").arg(cert_path).status()?;
    }
    if Path::new(key_path).exists() {
        Command::new("rm").arg(key_path).status()?;
    }
    fs::create_dir_all("/opt/BiliRoaming-Rust-Server/web").unwrap_or_default();
    fs::create_dir_all("/opt/BiliRoaming-Rust-Server/certificates").unwrap_or_default();
    Command::new("bash")
        .arg("-c")
        .arg("curl https://get.acme.sh | sh")
        .status()?;
    std::thread::spawn(|| {
        if let Ok(_) = simple_http_server("/opt/BiliRoaming-Rust-Server/web") {
        } else {
            println!("Error: Failed to run http server");
        }
    });
    std::thread::sleep(std::time::Duration::from_secs(2));

    let cmd2 = format!("source ~/.bashrc && ~/.acme.sh/acme.sh --install-cert -d {} --key-file {} --fullchain-file {}",website_name,AsRef::<Path>::as_ref(key_path).to_str().unwrap(),AsRef::<Path>::as_ref(cert_path).to_str().unwrap());
    Command::new("bash").arg("-c").arg(format!(r#"source ~/.bashrc && ~/.acme.sh/acme.sh --set-default-ca --server letsencrypt && ~/.acme.sh/acme.sh --issue -d {website_name} -w /opt/BiliRoaming-Rust-Server/web --renew-hook "{cmd2} --reloadcmd \"systemctl restart biliroaming_rust_server\"""#)).status()?;
    Command::new("bash").arg("-c").arg(cmd2).status()?;

    sync_getwebpage("http://127.0.0.1/signal/stop", "", "", None).unwrap_or_default();

    Ok(())
}
