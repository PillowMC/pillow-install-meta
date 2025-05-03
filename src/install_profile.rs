use std::{fs::File, io::{Read, Write}};

use serde_json::{json, Value};

use crate::{library::{get_added_librarys, FabricStyleLibrary, VanillaStyleLibrary}, Error, InstallProfileArgs};

const ICON: &str = include_str!("icon.txt");

pub(crate) fn create_install_profile(reader: impl Read, writer: impl Write, version_id: String, pillow_ver: String, quilt_ver: String) -> Result<(), Error> {
    let mut input: Value = serde_json::from_reader(reader)?;
    let root_obj = input
        .as_object_mut()
        .ok_or(Error("Huh? install_profile.json isn't an object?".to_string()))?;
    root_obj.insert("profile".to_string(), Value::String("Pillow".to_string()));
    root_obj.insert("icon".to_string(), Value::String(ICON.to_string()));
    root_obj.insert(
        "welcome".to_string(),
        Value::String("Welcome to the simple Pillow installer".to_string()),
    );
    root_obj.insert(
        "version".to_string(),
        Value::String(version_id),
    );
    root_obj.remove("mirrorList");
    root_obj["data"].as_object_mut().ok_or(Error("Huh? data in install_profile.json isn't an object?".to_string()))?.remove("BINPATCH");

    let mc_ver = {
        root_obj.get("minecraft")
            .ok_or(Error("Huh? No minecraft in install_profile.json?".to_string()))?.as_str()
            .ok_or(Error("Huh? minecraft in install_profile.json isn't a string?".to_string()))?.to_string()
    };

    let libraries = root_obj.get_mut("libraries")
        .ok_or(Error("Huh? No libraries in install_profile.json?".to_string()))?.as_array_mut()
        .ok_or(Error("Huh? libraries in install_profile.json isn't an array?".to_string()))?;

    libraries.push(json!({
        "name": "net.fabricmc:mapping-io:0.5.1@jar",
        "downloads": {
            "artifact": {
                "sha1": "bc93c07f23c01aa65ef9bd42e4d33d1c361ca122",
                "size": 158891,
                "url": "https://maven.fabricmc.net/net/fabricmc/mapping-io/0.5.1/mapping-io-0.5.1.jar",
                "path": "net/fabricmc/mapping-io/0.5.1/mapping-io-0.5.1.jar"
            }
        }
    }));
    libraries.push(json!({
        "name": "net.pillowmc:mappinggen:0.1.1@jar",
        "downloads": {
            "artifact": {
                "sha1": "6c926290d502d9f681fcf905e871f6b6fbb3bd78",
                "size": 1694,
                "url": "https://codeberg.org/PillowMC/mappinggen/releases/download/0.1.1/mappinggen-0.1.1.jar",
                "path": "net/pillowmc/mappinggen/0.1.1/mappinggen-0.1.1.jar"
            }
        }
    }));

    let added_libs = get_added_librarys(mc_ver.clone(), pillow_ver, quilt_ver, false, false)?;
    let intermediary = &FabricStyleLibrary {
        name: format!("net.fabricmc:intermediary:{mc_ver}:v2@jar"),
        url: Some("https://maven.fabricmc.net/".to_string())
    };
    let added_libs = added_libs.iter()
        .chain(std::iter::once(intermediary))
        .map(|i|<FabricStyleLibrary as TryInto<VanillaStyleLibrary>>::try_into(i.clone()).unwrap())
        .map(|i|serde_json::to_value(i).unwrap());

    libraries.extend(added_libs);

    let processors = root_obj.get_mut("processors")
        .ok_or(Error("Huh? No processors in install_profile.json?".to_string()))?.as_array_mut()
        .ok_or(Error("Huh? processors in install_profile.json isn't an array?".to_string()))?;

    let binary_patcher_index = processors.iter().position(|i|i["jar"].is_string() && i["jar"].as_str().unwrap().starts_with("net.neoforged.installertools:binarypatcher")).ok_or(Error("No binarypatcher in preprocessers".to_string()))?;
    processors.remove(binary_patcher_index);
    processors.push(json!({
        "jar": "net.neoforged.installertools:installertools:2.1.2",
        "classpath": [
            "org.ow2.asm:asm:9.3@jar",
            "net.md-5:SpecialSource:1.11.0@jar",
            "net.sf.jopt-simple:jopt-simple:5.0.4@jar",
            "net.neoforged.installertools:installertools:2.1.2@jar",
            "com.google.code.gson:gson:2.8.9@jar",
            "org.ow2.asm:asm-tree:9.3@jar",
            "com.opencsv:opencsv:4.4@jar",
            "net.neoforged:srgutils:1.0.0@jar",
            "org.apache.commons:commons-text:1.3@jar",
            "de.siegmar:fastcsv:2.0.0@jar",
            "org.apache.commons:commons-lang3:3.8.1@jar",
            "org.apache.commons:commons-collections4:4.2@jar",
            "org.ow2.asm:asm-commons:9.3@jar",
            "net.neoforged.installertools:cli-utils:2.1.2@jar",
            "com.google.guava:guava:20.0@jar",
            "commons-beanutils:commons-beanutils:1.9.3@jar",
            "org.ow2.asm:asm-analysis:9.3@jar",
            "commons-collections:commons-collections:3.2.2@jar",
            "commons-logging:commons-logging:1.2@jar"
        ],
        "args": [
            "--task",
            "EXTRACT_FILES",
            "--archive",
            format!("[net.fabricmc:intermediary:{mc_ver}:v2@jar]"),
            "--from",
            "mappings/mappings.tiny",
            "--to",
            format!("[net.fabricmc:intermediary:{mc_ver}:v2@tiny]")
        ]
    }));
    processors.push(json!({
        "jar": "net.neoforged.installertools:installertools:2.1.2",
        "classpath": [
            "org.ow2.asm:asm:9.3@jar",
            "net.md-5:SpecialSource:1.11.0@jar",
            "net.sf.jopt-simple:jopt-simple:5.0.4@jar",
            "net.neoforged.installertools:installertools:2.1.2@jar",
            "com.google.code.gson:gson:2.8.9@jar",
            "org.ow2.asm:asm-tree:9.3@jar",
            "com.opencsv:opencsv:4.4@jar",
            "net.neoforged:srgutils:1.0.0@jar",
            "org.apache.commons:commons-text:1.3@jar",
            "de.siegmar:fastcsv:2.0.0@jar",
            "org.apache.commons:commons-lang3:3.8.1@jar",
            "org.apache.commons:commons-collections4:4.2@jar",
            "org.ow2.asm:asm-commons:9.3@jar",
            "net.neoforged.installertools:cli-utils:2.1.2@jar",
            "com.google.guava:guava:20.0@jar",
            "commons-beanutils:commons-beanutils:1.9.3@jar",
            "org.ow2.asm:asm-analysis:9.3@jar",
            "commons-collections:commons-collections:3.2.2@jar",
            "commons-logging:commons-logging:1.2@jar"
        ],
        "args": [
            "--task",
            "CREATE_PARENTS",
            "--target",
            format!("[net.pillowmc:intermediary2srg:{mc_ver}@jar]")
        ]
    }));
    processors.push(json!({
        "jar": "net.pillowmc:mappinggen:0.1.1",
        "classpath": [
            "net.fabricmc:mapping-io:0.5.1@jar",
            "net.pillowmc:mappinggen:0.1.1@jar"
        ],
        "args": [
            "{MOJMAPS}",
            format!("[net.fabricmc:intermediary:{mc_ver}:v2@tiny]"),
            format!("[net.pillowmc:intermediary2srg:{mc_ver}@jar]")
        ]
    }));

    serde_json::to_writer(writer, root_obj)?;
    Ok(())
}

pub(crate) fn gen_install_profile(args: InstallProfileArgs) -> Result<(), Error> {
    create_install_profile(File::open(args.cmd.files.input)?, File::create(args.cmd.files.output)?, args.version_id, args.cmd.pillow_ver, args.cmd.quilt_ver)
}
