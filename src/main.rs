use std::io::Read;

use clap::{Command, Arg, arg, ArgAction};
use clap_complete::{Shell, Generator, generate};
use storage::Error;

use crate::storage::Storage;

mod storage;

fn build_command() -> clap::Command {
    Command::new("kv")
        .author("Tino Rusch <tino.rusch@gmail.com>")
        .about("encrypted and versioned command line storage")
        .version("0.1.0")
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .arg(arg!(--root <VALUE>).default_value("~/.kv").env("KV_ROOT"))
        .arg(arg!(--gpg <VALUE>).required(false).env("KV_GPG_ID"))
        .subcommand(
            Command::new("set")
                .about("set a key value pair")
                .alias("put")
                .arg(
                    Arg::new("key")
                        .help("key")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::new("value")
                        .help("value")
                        .required(false)
                        .index(2),
                ),
        )
        .subcommand(
            Command::new("get")
                .about("get a value")
                .alias("cat")
                .arg(
                    Arg::new("key")
                        .help("key")
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(
            Command::new("list")
                .about("list keys")
                .alias("ls")
                .arg(
                    Arg::new("dir")
                        .help("directory")
                        .index(1),
                ),
        )
        .subcommand(
            Command::new("delete")
                .about("delete a key")
                .alias("rm")
                .arg(Arg::new("recursive")
                    .short('r')
                    .long("recursive")
                    .action(ArgAction::SetTrue)
                )
                .arg(
                    Arg::new("key")
                        .help("key")
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(Command::new("push")
            .about("Push changes to remote origin")
        )
        .subcommand(Command::new("pull")
            .about("Pull changes from remote origin")
        )
        .subcommand(
            Command::new("generate-shell-completion")
                .hide(true)
                .about("generate shell completion script")
                .arg(
                    Arg::new("shell")
                        .help("shell to generate completion for")
                        .value_parser(clap::value_parser!(Shell))
                        .required(true)
                )
                .arg(
                    Arg::new("output")
                        .help("output file")
                        .required(false)
                        .value_hint(clap::ValueHint::FilePath)
                ),
        )
}

fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut std::io::stdout());
}

fn main() -> Result<(), Error>{
    // get command line options
    let matches = build_command().get_matches();
       
    // setup pretty env logger
    pretty_env_logger::init();

    let root = matches.get_one::<String>("root").unwrap();
    let root = shellexpand::tilde(root).to_string();
    if !std::path::Path::new(&root).exists() {
        std::fs::create_dir_all(&root)?;
    }

    let gpg_id = matches.get_one::<String>("gpg").map(|id|id.to_owned());
    
    let gpg_storage = storage::GpgStorage::new(&root, gpg_id);
    let mut git_storage = storage::GitStorage::new(gpg_storage, &root);

    match matches.subcommand() {
        Some(("set", matches)) => {
            let key = matches.get_one::<String>("key").unwrap();
            let value = match matches.get_one::<String>("value"){
                Some(value) => value.to_owned(),
                None => {
                    let mut buffer = String::new();
                    std::io::stdin().read_to_string(&mut buffer).unwrap();
                    buffer
                }
            };
            git_storage.set(key, &value)?;
        },
        Some(("get", matches)) => {
            let key = matches.get_one::<String>("key").unwrap();
            let value = git_storage.get(key).unwrap();
            print!("{}", value);
        },
        Some(("list", matches)) => {
            let dir = matches.get_one::<String>("dir")
                .map(|e|e.to_owned())
                .unwrap_or("".into());
            let keys = git_storage.list(&dir)?;
            for key in keys {
                println!("{}", key);
            }
        },
        Some(("delete", matches)) => {
            let key = matches.get_one::<String>("key").unwrap();
            let recursive = matches.get_flag("recursive");
            git_storage.remove(key, recursive)?;
        },
        Some(("push", _)) => {
            git_storage.push()?;
        },
        Some(("pull", _)) => {
            git_storage.pull()?;
        },
        Some(("generate-shell-completion", matches)) => {
            let mut cmd = build_command();
            let shell = matches.get_one::<Shell>("shell").unwrap().to_owned();
            print_completions(shell, &mut cmd);
        },
        _ => unreachable!(),
    }

    Ok(())
}
