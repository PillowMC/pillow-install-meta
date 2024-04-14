use std::{fs::File, io::{Read, Write}};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{library::get_addes_librarys, CmdArgs, Error};

#[derive(Deserialize, Serialize)]
struct VersionJsonArgs {
    game: Vec<String>,
    jvm: Vec<String>,
}

pub(crate) fn create_version_json(reader: impl Read, writer: impl Write, pillow_ver: String, quilt_ver: String) -> Result<String, Error> {
    let mut input: Value = serde_json::from_reader(reader)?;
    let root_obj = input.as_object_mut().ok_or(Error("Huh? version.json isn't an object?".to_string()))?;
    let arguments = root_obj.get("arguments").ok_or(Error("Huh? No arguments in version.json?".to_string()))?;
    let mut arguments: VersionJsonArgs = serde_json::from_value(arguments.clone())?;
    let (fml_ver, target_pos, ignore_list_pos, ipv6_pos) = {
        let fmlver_pos = arguments.game.iter().position(|i| i == "--fml.fmlVersion").ok_or(Error("Huh? No --fml.fmlVersion in game arguments?".to_string()))? + 1;
        (
            arguments.game.get(fmlver_pos).ok_or(Error("Huh? --fml.fmlVersion is the last one in game arguments?".to_string()))?.clone(), 
            arguments.game.iter().position(|i| i == "--launchTarget").ok_or(Error("Huh? No --launchTarget in game arguments?".to_string()))? + 1,
            arguments.jvm.iter().position(|i| i.starts_with("-DignoreList=")).ok_or(Error("Huh? No -DignoreList= in jvm arguments?".to_string()))?,
            arguments.jvm.iter().position(|i| i == "-Djava.net.preferIPv6Addresses=system").ok_or(Error("Huh? No -DignoreList= in jvm arguments?".to_string()))?,
        )
    };
    arguments.game[target_pos] = "pillowclient".to_string();
    arguments.jvm[ignore_list_pos] = format!("{},datafixerupper-", arguments.jvm[ignore_list_pos]);
    arguments.jvm.remove(ipv6_pos);
    let version_id = format!("pillow-{pillow_ver}+fml-{fml_ver}+quilt-loader-{quilt_ver}");
    root_obj.insert("id".to_string(), Value::String(version_id.clone()));
    root_obj.insert("arguments".to_string(), serde_json::to_value(arguments)?);

    let game_version = root_obj.get("inheritsFrom")
        .ok_or(Error("Huh? No inheritsFrom in version.json?".to_string()))?.as_str()
        .ok_or(Error("Huh? inheritsFrom isn't a string?".to_string()))?;
    let added_libs = get_addes_librarys(game_version.to_string(), pillow_ver, quilt_ver, false, true)?;
    let added_libs = added_libs.iter()
        .map(|i|serde_json::to_value(i).unwrap());
    root_obj.get_mut("libraries").ok_or(Error("Huh? No libraries in version.json?".to_string()))?
        .as_array_mut().ok_or(Error("Huh? libraries in version.json isn't an array?".to_string()))?
        .extend(added_libs);

    serde_json::to_writer(writer, root_obj)?;
    Ok(version_id)
}

pub(crate) fn gen_version_json(args: CmdArgs) -> Result<(), Error> {
    println!("{}", create_version_json(File::open(args.files.input)?, File::create(args.files.output)?, args.pillow_ver, args.quilt_ver)?);
    Ok(())
}