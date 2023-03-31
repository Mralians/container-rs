use std::fs::{self, File};
use std::io;
use std::io::Write;
use std::path::Path;

const CGROUP_BASE_DIR: &str = "/sys/fs/cgroup";
const CGROUP_PID_DIR: &str = "pids";

#[derive(Debug)]
pub struct ControlGroupConfig {
    pub name: String,
    pub pids_max: u32,
    pub cgroup_proc: u32,
    pub memery_max: String,
}
impl ControlGroupConfig {
    pub fn new(name: String, pids_max: u32, cgroup_proc: u32, memery_max: String) -> Self {
        Self {
            name,
            pids_max,
            cgroup_proc,
            memery_max,
        }
    }
    #[inline]
    pub fn cgroup_proc(&self) -> u32 {
        self.cgroup_proc
    }
    #[inline]
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }
    #[inline]
    pub fn pids_max(&self) -> u32 {
        self.pids_max
    }
}
pub fn create_control_group(config: &ControlGroupConfig) -> io::Result<()> {
    let cgroups = Path::new("/sys/fs/cgroup");
    let pids = cgroups.join("pids");
    let cgroup_dir = pids.join(&config.name);
    fs::create_dir_all(&cgroup_dir)?;

    let mut pid_max = fs::OpenOptions::new()
        .write(true)
        .open(pids.join("pids.max"))?;

    let mut cgroup_procs = fs::OpenOptions::new()
        .write(true)
        .open(cgroup_dir.join("cgroup.procs"))?;

    let mut memory_max = fs::OpenOptions::new()
        .write(true)
        .open(pids.join("memory.max"))?;

    pid_max.write(config.pids_max.to_string().as_bytes())?;
    cgroup_procs.write(config.cgroup_proc.to_string().as_bytes())?;
    memory_max.write(config.memery_max.as_bytes())?;

    Ok(())
}
