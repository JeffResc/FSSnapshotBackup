pub trait Filesystem {
    fn list_subvolumes(&self) -> Vec<Subvolume>;
    fn list_snapshots(&self) -> Vec<Snapshot>;
    fn create_snapshot(&self, subvolume: &Subvolume) -> Snapshot;
    fn delete_snapshot(&self, snapshot: &Snapshot) -> ();
}

pub struct Subvolume {
    pub name: String,
    pub used_space: u64,
    pub mountpoint: String
}

pub struct Snapshot {
    pub name: String,
    pub used_space: u64,
    pub mountpoint: String
}
