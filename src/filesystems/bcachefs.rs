use crate::filesystem;

pub struct BCacheFS {}

impl filesystem::Filesystem for BCacheFS {
    fn create_snapshot(&self, subvolume: &filesystem::Subvolume) -> filesystem::Snapshot {
        filesystem::Snapshot {
            name: String::from(""),
            used_space: 0,
            mountpoint: String::from("")
        }
    }

    fn delete_snapshot(&self, snapshot: &filesystem::Snapshot) -> () {

    }

    fn list_snapshots(&self) -> Vec<filesystem::Snapshot> {
        vec![filesystem::Snapshot {
            name: String::from(""),
            used_space: 0,
            mountpoint: String::from("")
        }]
    }

    fn list_subvolumes(&self) -> Vec<filesystem::Subvolume> {
        vec![filesystem::Subvolume {
            name: String::from(""),
            mountpoint: String::from(""),
            used_space: 0,
        }]
    }
}
