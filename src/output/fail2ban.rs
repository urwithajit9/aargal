use std::io::Write;
use std::os::unix::net::UnixStream;

use crate::config::schema::Fail2BanConfig;
use crate::output::executor::ExecutorError;

pub fn format_command(jail: &str, ip: &str) -> String {
    format!("set {} banip {}", jail, ip)
}


pub fn ban_ip(ip: &str, cfg: &Fail2BanConfig) -> Result<(), ExecutorError> {
    let mut stream = UnixStream::connect(&cfg.socket)?;
    let cmd = format!("set {} banip {}\n", cfg.jail, ip);
    stream.write_all(cmd.as_bytes())?;
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_fail2ban_command() {
        let cmd = format_command("aargal-auto", "1.2.3.4");
        assert_eq!(cmd, "set aargal-auto banip 1.2.3.4");
    }
}

