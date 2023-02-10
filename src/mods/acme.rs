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
        Command::new("rm").arg(cert_path).spawn()?;
    }
    if Path::new(key_path).exists() {
        Command::new("rm").arg(key_path).spawn()?;
    }
    fs::create_dir_all("/opt/BiliRoaming-Rust-Server/web").unwrap_or_default();
    fs::create_dir_all("/opt/BiliRoaming-Rust-Server/certificates").unwrap_or_default();
    Command::new("bash").arg("-c").arg("curl https://get.acme.sh | sh -s email=my@example.com").spawn()?;
    Command::new("bash").arg("-c").arg(format!("source ~/.bashrc && ~/.acme.sh/acme.sh --set-default-ca --server letsencrypt && ~/.acme.sh/acme.sh --issue -d {} -w /opt/BiliRoaming-Rust-Server/web",website_name)).spawn()?;
    Command::new("bash").arg("-c").arg(format!("source ~/.bashrc && ~/.acme.sh/acme.sh --install-cert -d {} --key-file {} --fullchain-file {}",website_name,AsRef::<Path>::as_ref(key_path).to_str().unwrap(),AsRef::<Path>::as_ref(cert_path).to_str().unwrap())).spawn()?;
    
    Ok(())
}
