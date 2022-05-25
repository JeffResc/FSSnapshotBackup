use crate::filesystem;

pub struct BTRFS {}

impl filesystem::Filesystem for BTRFS {
    fn create_snapshot(&self, subvolume: &String) -> (bool, String, String) {
        (true, String::from(""), String::from(""))
    }
    fn delete_snapshot(&self, subvolume: &String, snapshot_name: &String) -> bool {
        true
    }
}
