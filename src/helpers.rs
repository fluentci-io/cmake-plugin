use anyhow::Error;
use fluentci_pdk::dag;

pub fn setup_cmake(version: &str) -> Result<String, Error> {
    dag()
        .pkgx()?
        .with_exec(vec!["mkdir", "-p", "build"])?
        .stdout()?;

    let stdout = dag()
        .devbox()?
        .with_workdir("build")?
        .with_exec(vec!["devbox", "add", &format!("cmake@{}", version)])?
        .with_exec(vec!["devbox", "install"])?
        .stdout()?;
    Ok(stdout)
}
