pub fn read_diff() -> Result<String, String> {
    let output = std::process::Command::new("git")
        .arg("diff")
        .output();

    match output {
        Ok(ok) =>  Ok(String::from_utf8_lossy(&ok.stdout).to_string()),
        Err(e) => Err("something went wrong make sure you have git-diff available on your system".to_string() + &e.to_string())
    }
}
