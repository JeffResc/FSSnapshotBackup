use crate::filesystem;

pub struct BCacheFS {}

impl filesystem::Filesystem for BCacheFS {
    fn create_snapshot(&self, subvolume: &String) -> (bool, String, String) {
        (true, String::from(""), String::from(""))
    }
    fn delete_snapshot(&self, subvolume: &String, snapshot_name: &String) -> bool {
        true
    }
}
