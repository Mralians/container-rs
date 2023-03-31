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
use std::process::{Command, Stdio};

use builders::control_group::ControlGroupConfigBuilder;
fn main() -> Result<(), Box<dyn Error>> {
    match env::args().nth(1) {
        Some(arg) if arg == "container" => container()?,
        Some(arg) if arg == "run" => run()?,
        _ => panic!("help"),
    }

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
fn container() -> Result<(), std::io::Error> {
    println!("container id is {}", std::process::id());
    let args = env::args().skip(2).collect::<Vec<_>>();
    let mut control_group_builder = ControlGroupConfigBuilder::default();
    director::Director::construct_control_group(&mut control_group_builder);
    let config = control_group_builder.build();

    control_group::create_control_group(&config)?;

    let mut cmd = Command::new(&args[0]);
    cmd.args(&args[1..]);

    cmd.stdin(Stdio::inherit());
    cmd.stdout(Stdio::inherit());
    cmd.stderr(Stdio::inherit());

    nix::unistd::sethostname("container")?;
    nix::unistd::chroot("/home/mralians/ubuntu-jammy")?;
    nix::unistd::chdir("/")?;
    unsafe {
        cmd.pre_exec(|| {
            nix::mount::mount::<Path, Path, Path, Path>(
                Some(Path::new("none")),
                Path::new("/proc"),
                Some(Path::new("proc")),
                MsFlags::empty(),
                None,
            )?;
            nix::mount::mount::<Path, Path, Path, Path>(
                Some(Path::new("tmpfs")),
                Path::new("/tmp"),
                Some(Path::new("tmpfs")),
                MsFlags::empty(),
                None,
            )?;
            Ok(())
        });
    }
    let process = cmd.spawn()?;
    process.wait_with_output()?;

    nix::mount::umount("/proc")?;
    nix::mount::umount("/tmp")?;

    Ok(())
}
