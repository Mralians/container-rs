pub mod builders;
pub mod control_group;
pub mod director;

use anyhow::{Context, Result};
use nix::mount::MsFlags;
use nix::sched::CloneFlags;
use std::env;
use std::error::Error;
use std::io;
use std::os::unix::process::CommandExt;
use std::path::Path;
use std::process::{exit, Command, Stdio};

use builders::control_group::ControlGroupConfigBuilder;
use control_group::{cpu, memory};

fn main() -> Result<(), Box<dyn Error>> {
    // match env::args().nth(1) {
    //     Some(arg) if arg == "container" => container()?,
    //     Some(arg) if arg == "run" => run()?,
    //     _ => panic!("help"),
    // }
    let mut cgroup = ControlGroupConfigBuilder::default();
    let n = cgroup.build();
    println!("{n:?}");
    Ok(())
}

fn run() -> Result<()> {
    let args = env::args().skip(2).collect::<Vec<_>>();
    let mut cmd = Command::new("/proc/self/exe");
    cmd.arg("container").args(args);
    cmd.stdin(Stdio::inherit());
    cmd.stdout(Stdio::inherit());
    cmd.stdin(Stdio::inherit());

    unsafe {
        cmd.pre_exec(|| {
            let clone_flags =
                CloneFlags::CLONE_NEWNS | CloneFlags::CLONE_NEWUTS | CloneFlags::CLONE_NEWPID;

            nix::sched::unshare(clone_flags).unwrap();
            Ok(())
        });
    }
    let mut child = cmd.spawn().context("cannot spawn child process")?;
    child.wait().context("cannot wait for child process")?;
    Ok(())
}
