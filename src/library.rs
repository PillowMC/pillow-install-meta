use std::str::Split;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha1::{Sha1, Digest};

use crate::Error;

#[derive(Deserialize, Serialize, Clone)]
struct Artifact {
    sha1: String,
    size: usize,
    url: String,
    path: String
}

#[derive(Deserialize, Serialize, Clone)]
struct Download {
    artifact: Artifact
}

#[derive(Deserialize, Serialize, Clone)]
pub(crate) struct VanillaStyleLibrary {
    name: String,
    downloads: Download
}

#[derive(Deserialize, Serialize, Clone)]
pub(crate) struct FabricStyleLibrary {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl FabricStyleLibrary {
    pub fn get_path(&self) -> String {
        let mut splited = self.name.split(":");
        let mut ce: Split<&str>;
        let group = splited.next().unwrap().to_owned().replace(".", "/");
        let name = splited.next().unwrap().to_owned();
        let version = splited.next().unwrap().to_owned();
        let classifier = {
            let str = {
                ce = splited.next().unwrap_or("@jar").split("@");
                ce.clone()
            }.next().unwrap().to_string();
            if str.is_empty() {
                None
            } else {
                Some(str)
            }
        };
        let extension = {ce.next();ce}.next().unwrap_or("jar").to_string();
        if let Some(c) = classifier {
            format!("{group}/{name}/{version}/{name}-{version}-{c}.{extension}")
        } else {
            format!("{group}/{name}/{version}/{name}-{version}.{extension}")
        }
    }
}

impl TryInto<VanillaStyleLibrary> for FabricStyleLibrary {
    type Error = Error;
    fn try_into(self) -> Result<VanillaStyleLibrary, Error> {
        let path = self.get_path();
        let url = format!("{}{}", self.url.unwrap_or("".to_string()), path);
        eprintln!("{}: {}", self.name, url);
        let mut hash = Sha1::new();
        let mut req = reqwest::blocking::get(url.clone())?;
        req.copy_to(&mut hash)?;
        let hash = base16ct::lower::encode_string(&hash.finalize());
        eprintln!("{}: {}", self.name, hash);
        Ok(VanillaStyleLibrary {
            name: self.name,
            downloads: Download {
                artifact: Artifact {
                    sha1: hash,
                    size: req.content_length().unwrap().try_into()?,
                    url,
                    path,
                }
            }
        })
    }
}

pub(crate) fn get_added_librarys(game_version: String, pillow_ver: String, fabric_ver: String, server: bool, i2s: bool) -> Result<Vec<FabricStyleLibrary>, Error> {
    let type_ = if server {
        "server"
    } else {
        "profile"
    };
    let fabric_url = format!("https://meta.fabricmc.net/v2/versions/loader/{game_version}/{fabric_ver}/{type_}/json");
    let fabric_json: Value = reqwest::blocking::get(fabric_url)?.json()?;
    let fabric_json = fabric_json.as_object()
        .ok_or(Error("Huh? Fabric meta's profile json isn't an object?".to_string()))?;
    let libraries = serde_json::from_value::<Vec<FabricStyleLibrary>>(fabric_json.get("libraries")
        .ok_or(Error("Huh? No libraries in Fabric profile json?".to_string()))?.clone())?;
    let pillow_library = FabricStyleLibrary {
        name: format!("com.github.PillowMC:pillow:{pillow_ver}-fabric"), // Just because of the limitation of jitpack.io
        url: Some("https://jitpack.io/".to_string())
    };

    let intermediary2srg_library = FabricStyleLibrary {
        name: format!("net.pillowmc:intermediary2srg:{game_version}"), // Just because of the limitation of jitpack.io
        url: None
    };
    Ok(libraries
        .iter()
        .filter(|i|!(i.name.starts_with("org.ow2.asm")||i.name.contains(":intermediary:")))
        .cloned()
        .chain(if i2s {
            vec!(intermediary2srg_library, pillow_library)
        } else {
            vec!(pillow_library)
        })
        .collect())
}