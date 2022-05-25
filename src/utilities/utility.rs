pub trait Utility {
    fn backup(&self, backup_path: &String, backup_name: &String, backup_options: &String) -> bool;
}
