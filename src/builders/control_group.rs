use crate::control_group::ControlGroupConfig;

#[derive(Debug, Default)]
pub struct ControlGroupConfigBuilder {
    pub name: Option<String>,
    pub pids_max: Option<u32>,
    pub cgroup_proc: Option<u32>,
    pub memory_max: Option<String>,
}
impl ControlGroupConfigBuilder {
    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }
    pub fn set_pids_max(&mut self, pids_max: u32) {
        self.pids_max = Some(pids_max);
    }
    pub fn set_cgroup_proc(&mut self, cgroup_proc: u32) {
        self.cgroup_proc = Some(cgroup_proc);
    }
    pub fn build(self) -> ControlGroupConfig {
        ControlGroupConfig::new(
            self.name.expect("please set a name for cgroup"),
            self.pids_max.expect("please set max pid"),
            self.cgroup_proc.expect("please set a cgroup process"),
            self.memory_max.expect("please set memory max"),
        )
    }

    pub fn set_memory_max(&mut self, memory_max: String) {
        self.memory_max = Some(memory_max);
    }
}
