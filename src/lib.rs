use extism_pdk::*;
use fluentci_pdk::dag;

pub mod helpers;

#[plugin_fn]
pub fn setup(version: String) -> FnResult<String> {
    let version = if version.is_empty() {
        "latest".into()
    } else {
        format!("{}", version)
    };

    let stdout = helpers::setup_cmake(&version)?;

    Ok(stdout)
}

#[plugin_fn]
pub fn generate(src: String) -> FnResult<String> {
    let src = if src.is_empty() {
        "..".into()
    } else {
        format!("{}", src)
    };

    helpers::setup_cmake("latest")?;

    let stdout = dag()
        .devbox()?
        .with_workdir("build")?
        .with_exec(vec!["cmake", &src])?
        .stdout()?;

    Ok(stdout)
}

#[plugin_fn]
pub fn make(args: String) -> FnResult<String> {
    helpers::setup_cmake("latest")?;
    let stdout = dag()
        .devbox()?
        .with_workdir("build")?
        .with_exec(vec!["make", &args])?
        .stdout()?;
    Ok(stdout)
}
