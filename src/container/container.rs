fn container() -> Result<(), std::io::Error> {
    println!("container id is {}", std::process::id());
    let args = env::args().skip(2).collect::<Vec<_>>();
    let mut control_group_builder = ControlGroupConfigBuilder::default();
    director::Director::construct_control_group(&mut control_group_builder);
    let config = control_group_builder.build();
    println!("{config:?}");
    // control_group::create_control_group(&config)?;

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
