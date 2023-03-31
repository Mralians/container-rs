use anyhow::{Context, Result};
use nix::mount::MsFlags;
use nix::sched::CloneFlags;
use std::env;
use std::error::Error;
use std::fs;
use std::io::{self, Write};
use std::os::unix::process::CommandExt;
use std::path::Path;
use std::process::{Command, Stdio};

pub mod container;
pub mod control_group;
fn main() -> Result<(), Box<dyn Error>> {
    match env::args().nth(1) {
        Some(arg) if arg == "child" => child()?,
        Some(arg) if arg == "run" => run()?,
        _ => panic!("help"),
    }
    Ok(())
}

fn run() -> Result<()> {
    let args = env::args().skip(2).collect::<Vec<_>>();
    // let stack = &mut [0; STACK_SIZE];
    println!("Running {args:?}");

    let mut cmd = Command::new("/proc/self/exe");
    cmd.arg("child").args(args);
    cmd.stdin(Stdio::inherit());
    cmd.stdout(Stdio::inherit());
    cmd.stdin(Stdio::inherit());

    unsafe {
        cmd.pre_exec(|| {
            let clone_flags =
                CloneFlags::CLONE_NEWNS | CloneFlags::CLONE_NEWUTS | CloneFlags::CLONE_NEWPID;

            Ok(nix::sched::unshare(clone_flags).unwrap())
        });
    }
    let mut child = cmd.spawn().context("cannot spawn child process")?;
    child.wait().context("cannot wait for child process")?;
    Ok(())
}
fn child() -> io::Result<()> {
    let args = env::args().skip(2).collect::<Vec<_>>();
    println!("Running {args:?}");
    let config = control_group::ControlGroupConfigBuilder::new()
        .name("mralians".to_string())
        .max_processes(20)
        .build();
    control_group::create_control_group(&config)?;

    let mut cmd = Command::new(&args[0]);
    cmd.args(&args[1..]);

    cmd.stdin(Stdio::inherit());
    cmd.stdout(Stdio::inherit());
    cmd.stderr(Stdio::inherit());

    nix::unistd::sethostname("container")?;
    nix::unistd::chroot("/home/mralians/ubuntu-jammy")?;
    nix::unistd::chdir("/")?;
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
    let process = cmd.spawn()?;
    process.wait_with_output()?;

    Ok(())
}
