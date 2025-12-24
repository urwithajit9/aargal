#[derive(Debug)]
pub enum DoctorStatus {
    Ok,
    Warn,
    Error,
}

#[derive(Debug)]
pub struct DoctorEntry {
    pub status: DoctorStatus,
    pub message: String,
}

pub struct DoctorReport {
    entries: Vec<DoctorEntry>,
}

impl DoctorReport {
    pub fn new() -> Self {
        Self { entries: Vec::new() }
    }

    pub fn ok(&mut self, msg: impl Into<String>) {
        self.entries.push(DoctorEntry {
            status: DoctorStatus::Ok,
            message: msg.into(),
        });
    }

    pub fn warn(&mut self, msg: impl Into<String>) {
        self.entries.push(DoctorEntry {
            status: DoctorStatus::Warn,
            message: msg.into(),
        });
    }

    pub fn error(&mut self, msg: impl Into<String>) {
        self.entries.push(DoctorEntry {
            status: DoctorStatus::Error,
            message: msg.into(),
        });
    }

    pub fn print(&self) {
        println!("\nAargal Doctor Report\n====================");
        for e in &self.entries {
            let prefix = match e.status {
                DoctorStatus::Ok => "✔",
                DoctorStatus::Warn => "⚠",
                DoctorStatus::Error => "✖",
            };
            println!("{} {}", prefix, e.message);
        }
    }
}
