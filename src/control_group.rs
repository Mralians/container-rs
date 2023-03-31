use std::fs::{self, File};
use std::io;
use std::io::Write;
use std::path::Path;

const CGROUP_BASE_DIR: &str = "/sys/fs/cgroup";
const CGROUP_PID_DIR: &str = "pids";

#[derive(Debug)]
pub struct ControlGroupConfig {
    name: String,
    max_processes: u64,
}

impl ControlGroupConfig {
    pub fn builder() -> ControlGroupConfigBuilder {
        ControlGroupConfigBuilder::new()
    }
}

#[derive(Debug)]
pub struct ControlGroupConfigBuilder {
    name: String,
    max_processes: u64,
}

impl ControlGroupConfigBuilder {
    pub fn new() -> Self {
        Self {
            name: "mralians".to_owned(),
            max_processes: 20,
        }
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    pub fn max_processes(mut self, max_processes: u64) -> Self {
        self.max_processes = max_processes;
        self
    }

    pub fn build(self) -> ControlGroupConfig {
        ControlGroupConfig {
            name: self.name,
            max_processes: self.max_processes,
        }
    }
}

pub fn create_control_group(config: &ControlGroupConfig) -> io::Result<()> {
    let cgroups = Path::new(CGROUP_BASE_DIR);
    let pids = cgroups.join("pids");

    let cgroup_dir = pids.join(&config.name);
    fs::create_dir_all(&cgroup_dir)?;

    let mut pid_max = File::create(cgroup_dir.join("pids.max"))?;
    let mut cgroup_procs = File::create(cgroup_dir.join("cgroup.procs"))?;
    pid_max.write(config.max_processes.to_string().as_bytes())?;
    cgroup_procs.write(format!("{}", std::process::id()).as_bytes())?;

    Ok(())
}
