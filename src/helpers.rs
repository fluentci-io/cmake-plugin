use anyhow::Error;
use fluentci_pdk::dag;

pub fn setup_cmake(version: &str) -> Result<String, Error> {
    let stdout = dag()
        .devbox()?
        .with_exec(vec!["mkdir", "-p", "build"])?
        .with_exec(vec!["cp", "devbox.*", "build/"])?
        .with_workdir("build")?
        .with_exec(vec!["devbox", "add", &format!("cmake@{}", version)])?
        .with_exec(vec!["devbox", "install"])?
        .stdout()?;
    Ok(stdout)
}
