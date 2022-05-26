use crate::utility;

pub struct Borg {}

impl utility::Utility for Borg {
    fn backup(&self, _backup_path: &String, _backup_name: &String, _backup_options: &String) -> bool {
        unimplemented!("The Borg utility backend is not yet implemented!");
    }
}
