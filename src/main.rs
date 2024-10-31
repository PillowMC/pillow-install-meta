mod version_json;
mod install_profile;
mod jvm_args;
mod library;

use clap::{Args, Parser};

#[derive(Debug)]
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
struct JvmArgsArgs {
    #[command(flatten)]
    cmd: CmdArgs,


    /// Use this when generating win_args.txt.
    #[arg(long)]
    windows: bool,
}

#[derive(Parser, Clone)]
struct InstallProfileArgs {
    #[command(flatten)]
    cmd: CmdArgs,


    /// Version ID
    #[arg(value_name = "VERSION ID")]
    version_id: String
}

#[derive(Parser, Clone)]
struct CmdArgs {
    #[command(flatten)]
    files: FileArgs,

    /// Pillow version
    #[arg(value_name = "PILLOW VERSION")]
    pillow_ver: String,

    /// Fabric Loader version
    #[arg(value_name = "FABRIC LOADER VERSION")]
    fabric_ver: String,
}

/// Pillow installer generator.
#[derive(Parser)]
#[command(version, about, long_about = None)]
enum Command {
    /// Generates version.json.
    VersionJson(CmdArgs),
    /// Generates {win,unix}_args.txt
    JvmArgs(JvmArgsArgs),
    /// Generates install_profile.json.
    InstallProfile(InstallProfileArgs)
}

fn main() {
    let args = Command::parse();
    let res = match args {
        Command::VersionJson(f) => version_json::gen_version_json(f),
        Command::InstallProfile(f) => install_profile::gen_install_profile(f),
        Command::JvmArgs(f) => jvm_args::gen_jvm_args(f),
    };
    if let Err(e) = res {
        eprintln!("{}", e.0);
    }
}
