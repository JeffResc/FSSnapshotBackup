pub trait Filesystem {
    fn create_snapshot(&self, subvolume: &String) -> (bool, String, String);
    fn delete_snapshot(&self, subvolume: &String, snapshot_name: &String) -> bool;
}
