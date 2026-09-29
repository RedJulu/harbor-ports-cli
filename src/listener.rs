pub struct PortInfo {
    pub address: String,
    pub port: u16,
    pub process: String,
    pub pid: u32,
}

pub fn get_active_listeners() -> Vec<PortInfo> {
    let mut result = Vec::new();

    if let Ok(listeners) = listeners::get_all() {
        for l in listeners {
            result.push(PortInfo {
                address: l.socket.ip().to_string(),
                port: l.socket.port(),
                process: l.process.name,
                pid: l.process.pid,
            });
        }
    }

    result
}

pub fn filter_for_port(port: u16) -> Vec<PortInfo> {
    get_active_listeners()
        .into_iter()
        .filter(|p| p.port == port)
        .collect()
}

pub fn terminate_port(port: u16, force: bool) -> bool {
    if let Some(info) = get_active_listeners().into_iter().find(|l| l.port == port) {
        let signal = if force { 9 } else { 15 };
        #[cfg(unix)]
        {
            use std::process::Command;
            let status = Command::new("kill")
                .arg(format!("-{}", signal))
                .arg(info.pid.to_string())
                .status();
            return status.is_ok_and(|s| s.success());
        }

        #[cfg(not(unix))]
        {
            let _ = (signal, l);
            return false;
        }
    }
    false
}
