use super::control_group::ControlGroupConfig;

#[derive(Debug)]
struct Container {
    chroot_path: String,
    hostname: String,
    cgroup: ControlGroupConfig,
}

impl Container {
    pub fn new() -> Self {}
}
