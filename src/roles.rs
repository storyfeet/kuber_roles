use err_tools::*;
use serde_derive::*;
use std::collections::BTreeMap;
//use std::io::Read;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KubeOut {
    pub items: Vec<RoleItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct RoleItem {
    pub kind: String,
    pub metadata: RoleMeta,
    pub roleRef: RoleRef,
    pub subjects: Option<Vec<RoleSubject>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleMeta {
    name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleRef {
    kind: String,
    name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct RoleSubject {
    pub apiGroup: Option<String>,
    pub kind: String,
    pub name: String,
}

use std::process::Command;

pub fn get_subjects() -> anyhow::Result<Vec<SubjectItem>> {
    let mut res = get_roles(["get", "clusterrolebinding", "-o", "json"]).map(transpose)?;

    //Didn't have data to test assuming same as clusterrolebindings
    let res2 = get_roles(["get", "rolebinding", "-o", "json"]).map(transpose)?;
    res.extend(res2);
    Ok(res)
}

pub fn get_roles<I, S>(args: I) -> anyhow::Result<KubeOut>
where
    I: IntoIterator<Item = S>,
    S: AsRef<std::ffi::OsStr>,
{
    let output = Command::new("kubectl")
        .args(args)
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

//Transpose to

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub struct SubjectItem {
    pub name: String,
    pub kind: String,
    pub roles: Vec<String>,
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct SubjectKey {
    name: String,
    kind: String,
}

#[derive(Debug)]
pub struct SubjectVal {
    roles: Vec<String>,
}

pub fn transpose(ko: KubeOut) -> Vec<SubjectItem> {
    let mut map: BTreeMap<SubjectKey, SubjectVal> = BTreeMap::new();
    for k in ko.items {
        if let Some(sub) = k.subjects {
            for s in sub {
                let sk = SubjectKey {
                    name: s.name,
                    kind: s.kind,
                };
                match map.get_mut(&sk) {
                    Some(v) => v.roles.push(k.metadata.name.clone()),
                    _ => {
                        map.insert(
                            sk,
                            SubjectVal {
                                roles: vec![k.metadata.name.clone()],
                            },
                        );
                    }
                }
            }
        }
    }
    map.into_iter()
        .map(|(k, v)| SubjectItem {
            name: k.name,
            kind: k.kind,
            roles: v.roles,
        })
        .collect()
}
