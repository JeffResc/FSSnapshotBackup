use crate::filesystem;

pub struct ZFS {}

impl filesystem::Filesystem for ZFS {
    fn create_snapshot(&self, subvolume: &filesystem::Subvolume) -> filesystem::Snapshot {
        // run command: "zfs snapshot <subvolume>@<snapshot_name>"
        
        // generate snapshot name based on current time
        let now = std::time::SystemTime::now();
        let now_str = now.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs().to_string();
        let snapshot_name = format!("fssnapshotbackup_{}", now_str);
        let snapshot_full_name = format!("{}@{}", subvolume.name, snapshot_name);

        // run command
        std::process::Command::new("zfs")
            .arg("snapshot")
            .arg(&snapshot_full_name)
            .spawn()
            .expect("failed to execute process");

        // ensure the snapshot was created
        // this can take a few seconds, so we'll do a loop with a 5 second timeout
        let mut timeout = 0;
        while timeout < 5 {
            // list snapshots
            let snapshots = self.list_snapshots();

            // find snapshot
            for snapshot in snapshots {
                if snapshot.name == snapshot_full_name {
                    // test if snapshot mountpoint exists
                    let mountpoint = format!("{}/.zfs/snapshot/{}", subvolume.mountpoint, snapshot.name.split("@").collect::<Vec<&str>>()[1]);
                    if std::fs::metadata(&mountpoint).is_ok() {
                        // return snapshot
                        let s = filesystem::Snapshot {
                            name: snapshot.name,
                            used_space: snapshot.used_space,
                            mountpoint: mountpoint
                        };
                        return s;
                    } else {
                        self.delete_snapshot(&snapshot);
                        println!("Error: Unable to locate snapshot mountpoint, deleting snapshot...");
                        std::process::exit(1);
                    }
                }
            }
            timeout += 1;
            std::thread::sleep(std::time::Duration::from_secs(1));
        }

        // return empty snapshot
        return filesystem::Snapshot {
            name: String::from(""),
            used_space: 0,
            mountpoint: String::from("")
        };
    }

    fn delete_snapshot(&self, snapshot: &filesystem::Snapshot) -> () {
        // run comamnd: "zfs destroy <subvolume>@<snapshot_name>"
        std::process::Command::new("zfs")
            .arg("destroy")
            .arg(format!("{}@{}", snapshot.name, snapshot.name))
            .spawn()
            .expect("failed to execute process");
    }

    fn list_snapshots(&self) -> Vec<filesystem::Snapshot> {
        // run command: "zfs list -t snapshot -H"
        let zfs_list_snapshots = std::process::Command::new("zfs")
            .arg("list")
            .arg("-t")
            .arg("snapshot")
            .arg("-o")
            .arg("name,used,mountpoint")
            .arg("-p")
            .arg("-H")
            .output()
            .expect("failed to execute process");
        
        // parse output
        // tab seperated with columns: name used
        let zfs_list_snapshots_str = String::from_utf8_lossy(&zfs_list_snapshots.stdout);
        let mut zfs_list_snapshots_vec: Vec<&str> = zfs_list_snapshots_str.split("\n").collect();
        zfs_list_snapshots_vec.pop();

        let snapshots = zfs_list_snapshots_vec.iter().map(|line| {
            let line_vec: Vec<&str> = line.split("\t").collect();
            filesystem::Snapshot {
                name: String::from(line_vec[0]),
                used_space: u64::from_str_radix(line_vec[1], 10).unwrap(),
                mountpoint: String::from(line_vec[2])
            }
        }).collect();

        return snapshots;
    }

    fn list_subvolumes(&self) -> Vec<filesystem::Subvolume> {
        // run command: "zfs list -H"
        let zfs_list = std::process::Command::new("zfs")
            .arg("list")
            .arg("-t")
            .arg("filesystem")
            .arg("-o")
            .arg("name,used,mountpoint")
            .arg("-p")
            .arg("-H")
            .output()
            .expect("Failed to run zfs list command");

        // parse output
        // tab separated with columns: name used mountpoint
        let zfs_list_str = String::from_utf8_lossy(&zfs_list.stdout);
        let mut zfs_list_vec: Vec<&str> = zfs_list_str.split('\n').collect();
        zfs_list_vec.pop();

        let subvols = zfs_list_vec.iter().map(|line| {
            let line_vec: Vec<&str> = line.split('\t').collect();
            filesystem::Subvolume {
                name: String::from(line_vec[0]),
                used_space: u64::from_str_radix(line_vec[1], 10).unwrap(),
                mountpoint: String::from(line_vec[2]),
            }
        }).collect();

        return subvols;
    }
}
