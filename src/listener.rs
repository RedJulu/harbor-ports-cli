use listeners::Protocol;

#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub enum ProtocolFilter {
    Tcp,
    Udp,
    All,
}

pub struct PortInfo {
    pub address: String,
    pub port: u16,
    pub protocol: String,
    pub process: String,
    pub pid: u32,
}

pub fn get_active_listeners(filter: ProtocolFilter) -> Vec<PortInfo> {
    let Ok(listeners) = listeners::get_all() else {
        return Vec::new();
    };

    listeners
        .into_iter()
        .filter(|l| match filter {
            ProtocolFilter::Tcp => l.protocol == Protocol::TCP,
            ProtocolFilter::Udp => l.protocol == Protocol::UDP,
            ProtocolFilter::All => true,
        })
        .map(|l| PortInfo {
            address: l.socket.ip().to_canonical().to_string(),
            port: l.socket.port(),
            protocol: l.protocol.to_string(),
            process: l.process.name,
            pid: l.process.pid,
        })
        .collect()
}

pub fn filter_for_port(port: u16, filter: ProtocolFilter) -> Vec<PortInfo> {
    get_active_listeners(filter)
        .into_iter()
        .filter(|p| p.port == port)
        .collect()
}

pub fn terminate_port(port: u16, force: bool, filter: ProtocolFilter) -> bool {
    if let Some(info) = get_active_listeners(filter)
        .into_iter()
        .find(|l| l.port == port)
    {
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
