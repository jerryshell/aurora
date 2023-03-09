pub fn execute_cmd(cmd: &str) -> std::process::Output {
    if cfg!(target_os = "windows") {
        tracing::info!("windows execute_cmd: {cmd}");
        let output = std::process::Command::new("cmd")
            .args(["/C", cmd])
            .output()
            .expect("failed to execute process");
        tracing::info!("windows exe ok");
        output
    } else {
        tracing::info!("unix execute_cmd: {cmd}");
        let output = std::process::Command::new("sh")
            .args(["-c", cmd])
            .output()
            .expect("failed to execute process");
        tracing::info!("unix exe ok");
        output
    }
}
