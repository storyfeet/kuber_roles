use err_tools::*;
//use std::io::Read;
use std::process::Command;
pub fn get_roles() -> anyhow::Result<String> {
    let output = Command::new("kubectl")
        .args(["get", "clusterrolebinding", "-o", "json"])
        .output()
        .e_str("Could not run kubectl")?;

    if !output.status.success() {
        return e_str("Kubectl exited with error :");
    }

    let s = String::from_utf8(output.stdout).e_str("Could not read output string")?;

    Ok(s)
}
