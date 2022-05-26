use std::path::PathBuf;
use clap::Parser;

#[path = "./filesystems.rs"]
mod filesystems;
#[path = "./filesystems/filesystem.rs"]
mod filesystem;
#[path = "./filesystems/bcachefs.rs"]
mod bcachefs;
#[path = "./filesystems/btrfs.rs"]
mod btrfs;
#[path = "./filesystems/zfs.rs"]
mod zfs;

#[path = "./utilities.rs"]
mod utilities;
#[path = "./utilities/utility.rs"]
mod utility;
#[path = "./utilities/borg.rs"]
mod borg;
#[path = "./utilities/restic.rs"]
mod restic;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// The file system to target: zfs, btrfs, bcachefs
    filesystem: String,

    /// The backup utility to use: restic, borg
    utility: String,

    /// The filesystem subvolume
    subvolume: String,

    /// Config file??
    #[clap(short, long, parse(from_os_str), value_name = "FILE")]
    config: Option<PathBuf>,

    /// Turn debugging information on
    #[clap(short, long, parse(from_occurrences))]
    debug: usize,
}

fn main() {
    let cli = Cli::parse();

    let valid_filesystems = vec!["zfs", "btrfs", "bcachefs"];
    let valid_utilities = vec!["restic", "borg"];

    let mut cli_error = false;
    if !valid_filesystems.iter().any(|e| cli.filesystem.to_lowercase().contains(e)) {
        println!("Invalid filesystem {:?}. Valid filesystems: {:?}", cli.filesystem, valid_filesystems.join(", "));
        cli_error = true;
    }
    if !valid_utilities.iter().any(|e| cli.utility.to_ascii_lowercase().contains(e)) {
        println!("Invalid backup utility {:?}. Valid utilities: {:?}", cli.utility, valid_utilities.join(", "));
        cli_error = true;
    }

    if cli_error {
        std::process::exit(1);
    }

    let filesystem = filesystems::filesystem(cli.filesystem.to_string().to_lowercase());
    //let utility = utilities::utility(cli.utility.to_string());

    let subvolumes = filesystem.list_subvolumes();

    let mut subvolume = filesystem::Subvolume {
        name: String::from(""),
        used_space: 0,
        mountpoint: String::from(""),
    };
    for sv in subvolumes {
        if sv.name == cli.subvolume {
            subvolume = sv;
        }
    }
    if subvolume.name == "" {
        println!("Subvolume {:?} not found in filesystem {:?}", cli.subvolume, cli.filesystem);
        std::process::exit(1);
    }

    let snapshot = filesystem.create_snapshot(&subvolume);
    println!("Created snapshot {} {} {}", snapshot.name, snapshot.used_space, snapshot.mountpoint);

    //let success = utility.backup(&snapshot_path, &cli.subvolume, &String::from(""));

    // let success = filesystem.delete_snapshot(&cli.subvolume.to_string(), &snapshot_name);
    // if !success {
    //     println!("Failed to delete snapshot");
    //     std::process::exit(1);
    // }

}
