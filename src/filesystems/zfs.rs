use crate::filesystem;

pub struct ZFS {}

impl filesystem::Filesystem for ZFS {
    fn create_snapshot(&self, subvolume: &String) -> (bool, String, String) {
        (true, String::from(""), String::from(""))
    }
    fn delete_snapshot(&self, subvolume: &String, snapshot_name: &String) -> bool {
        true
    }
}
