use err_tools::*;
use serde_derive::*;
use std::collections::BTreeMap;
//use std::io::Read;

/// Required for Parsing the Kubectl json output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KubeOut {
    pub items: Vec<RoleItem>,
}

/// Required for Parsing the Kubectl json output
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct RoleItem {
    pub kind: String,
    pub metadata: RoleMeta,
    pub roleRef: RoleRef,
    pub subjects: Option<Vec<RoleSubject>>,
}

/// Required for Parsing the Kubectl json output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleMeta {
    name: String,
}

/// Required for Parsing the Kubectl json output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleRef {
    kind: String,
    name: String,
}

/// Required for Parsing the Kubectl json output
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct RoleSubject {
    pub apiGroup: Option<String>,
    pub kind: String,
    pub name: String,
}

use std::process::Command;

/// for both clusterrolebinding and rolebinding,
/// get the json data from kubectl and convert it to a list of subjects
pub fn get_subjects() -> anyhow::Result<Vec<SubjectItem>> {
    let mut res = get_roles(["get", "clusterrolebinding", "-o", "json"]).map(transpose)?;

    //Didn't have data to test assuming same as clusterrolebindings
    let res2 = get_roles(["get", "rolebinding", "-o", "json"]).map(transpose)?;
    res.extend(res2);
    Ok(res)
}

/// Run kubectl with the given arguments to return the json content as a KubeOut structure.
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

/// The structure for sorting that the API Caller will see in Json/Yaml format.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub struct SubjectItem {
    /// Name of the subject
    pub name: String,
    /// User/Group/ServiceApplication
    pub kind: String,
    /// List of roles this sbject is associated with
    pub roles: Vec<String>,
}

/// The key for the map while transposing
#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
struct SubjectKey {
    name: String,
    kind: String,
}

/// The Value for the map while transposing
#[derive(Debug)]
pub struct SubjectVal {
    roles: Vec<String>,
}

/// Turn a list of roles and associated subjects to a list of subjects with associated roles.
pub fn transpose(ko: KubeOut) -> Vec<SubjectItem> {
    // Loop through the items, and their subjects,
    // For every subject add it to the map, with its role as the value.
    // If the subject is in the map allready add the new role to the list.
    let mut map: BTreeMap<SubjectKey, SubjectVal> = BTreeMap::new();
    for k in ko.items {
        if let Some(sub) = k.subjects {
            for s in sub {
                let sk = SubjectKey {
                    name: s.name,
                    kind: s.kind,
                };
                match map.get_mut(&sk) {
                    //To Consider : Using RefCount for some of the strings to cheapen the cloning
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
    // Pull the data from the tree into the Vec result
    map.into_iter()
        .map(|(k, v)| SubjectItem {
            name: k.name,
            kind: k.kind,
            roles: v.roles,
        })
        .collect()
}
