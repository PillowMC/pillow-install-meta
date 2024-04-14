mod version_json;
mod install_profile;

use clap::{Args, Parser};

struct Error(String);

impl <E> From<E> for Error where E: std::error::Error {
    fn from(value: E) -> Self {
        Self(value.to_string())
    }
}

#[derive(Clone, Args)]
struct FileArgs {
    /// NeoForge's file
    #[arg(value_name = "INPUT")]
    input: std::path::PathBuf,

    /// Pillow's file
    #[arg(value_name = "OUTPUT")]
    output: std::path::PathBuf,
}

#[derive(Parser, Clone)]
struct InstallProfileArgs {
    #[command(flatten)]
    files: FileArgs,


    /// Version ID
    #[arg(value_name = "VERSION ID")]
    version_id: String,
}

#[derive(Parser, Clone)]
struct CmdArgs {
    #[command(flatten)]
    files: FileArgs,

    /// Pillow version
    #[arg(value_name = "PILLOW VERSION")]
    pillow_ver: String,

    /// Quilt Loader version
    #[arg(value_name = "QUILT LOADER VERSION")]
    quilt_ver: String,
}

/// Pillow installer generator.
#[derive(Parser)]
#[command(version, about, long_about = None)]
enum Command {
    /// Generates version.json.
    VersionJson(CmdArgs),
    /// Generates install_profile.json.
    InstallProfile(InstallProfileArgs)
}

fn main() {
    let args = Command::parse();
    let res = match args {
        Command::VersionJson(f) => version_json::gen_version_json(f),
        Command::InstallProfile(f) => install_profile::gen_install_profile(f)
    };
    if let Err(e) = res {
        eprintln!("{}", e.0);
    }
}
