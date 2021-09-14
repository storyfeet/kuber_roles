use err_tools::*;
use serde_derive::*;
//use std::io::Read;

#[derive(Debug, Serialize, Deserialize)]
pub struct KubeOut {
    items: Vec<RoleItem>,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct RoleItem {
    kind: String,
    metadata: RoleMeta,
    roleRef: RoleRef,
    subjects: Option<Vec<RoleSubject>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoleMeta {
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoleRef {}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct RoleSubject {
    apiGroup: Option<String>,
    kind: String,
    name: String,
}

use std::process::Command;
pub fn get_roles() -> anyhow::Result<KubeOut> {
    let output = Command::new("kubectl")
        .args(["get", "clusterrolebinding", "-o", "json"])
        .output()
        .e_str("Could not run kubectl")?;

    if !output.status.success() {
        return e_str("Kubectl exited with error :");
    }

    let s = String::from_utf8(output.stdout).e_str("Could not read output string")?;

    println!("roles == {}", s);

    let ko: KubeOut = serde_json::from_str(&s)?;

    Ok(ko)
}
