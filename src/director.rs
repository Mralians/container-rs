use crate::builders::control_group::ControlGroupConfigBuilder;
use std::process;
pub struct Director;

impl Director {
    pub fn construct_control_group(builder: &mut ControlGroupConfigBuilder) {
        builder.set_name("mralians".to_string());
        builder.set_cgroup_proc(process::id());
        builder.set_pids_max(10);
        // builder.set_memory_max("100m".to_string());
    }
}
