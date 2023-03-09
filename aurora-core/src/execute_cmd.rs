pub fn execute_cmd(cmd: &str) -> std::process::Output {
    if cfg!(target_os = "windows") {
        println!("windows execute_cmd: {cmd}");
        let output = std::process::Command::new("cmd")
            .args(["/C", cmd])
            .output()
            .expect("failed to execute process");
        println!("windows exe ok");
        output
    } else {
        println!("unix execute_cmd: {cmd}");
        let output = std::process::Command::new("sh")
            .args(["-c", cmd])
            .output()
            .expect("failed to execute process");
        println!("unix exe ok");
        output
    }
}
