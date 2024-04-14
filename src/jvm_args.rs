use std::{fs::File, io::{Read, Write}};

use crate::{library::get_addes_librarys, Error, JvmArgsArgs};

const SEP_UNIX: &str = ":";
const SEP_WINDOWS: &str = ";";

pub(crate) fn create_jvm_args(mut reader: impl Read, mut writer: impl Write, pillow_ver: String, quilt_ver: String, windows: bool) -> Result<(), Error> {
    let sep = if windows {
        SEP_WINDOWS
    } else {
        SEP_UNIX
    };
    let mut str = String::new();
    reader.read_to_string(&mut str)?;
    let lines = str.lines();
    let game_version = lines.clone().find_map(|i|i.strip_prefix("--fml.mcVersion ")).unwrap().to_string();
    for i in lines {
        if i == "-Djava.net.preferIPv6Addresses=system" {
            continue;
        }
        if i.starts_with("-DignoreList=") {
            writeln!(writer, "{},datafixerupper-", i)?;
            continue;
        }
        if i.starts_with("--launchTarget") {
            writeln!(writer, "--launchTarget pillowserver")?;
            continue;
        }
        if i.starts_with("-DlegacyClassPath=") {
            let added: String = get_addes_librarys(game_version.clone(), pillow_ver.clone(), quilt_ver.clone(), true, true)?.iter()
                .map(|i|format!("{sep}libraries/{}", i.get_path())).collect();
            writeln!(writer, "{i}{added}")?;
            continue;
        }
        writeln!(writer, "{i}")?;
    }
    Ok(())
}

pub(crate) fn gen_jvm_args(args: JvmArgsArgs) -> Result<(), Error> {
    create_jvm_args(File::open(args.cmd.files.input)?, File::create(args.cmd.files.output)?, args.cmd.pillow_ver, args.cmd.quilt_ver, args.windows)
}
