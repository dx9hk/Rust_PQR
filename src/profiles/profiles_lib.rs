use std::path::PathBuf;
use crate::profiles::profile_lib::Profile;

#[derive(Debug, PartialEq, Clone)]
pub struct Profiles {
    list_of_profiles: Vec<Profile>
}

impl Profiles {
    /// Construct from profiles folder
    pub fn new(profiles_folder: PathBuf) -> Self {
        // Check if valid directory
        if !profiles_folder.is_dir() {
            panic!("Failed to read profiles directory");
        }
        // Setup return vector
        let mut return_vector = vec![];
        // Loop through all profiles and add to return vector
        profiles_folder
            .read_dir()
            .unwrap()
            .for_each(|child_dir| {
                let curr_dir = child_dir.unwrap().path();
                return_vector.push(Profile::new(curr_dir));
            });
        // Return data to constructor
        Self {
            list_of_profiles: return_vector
        }
    }
    /// Return profiles
    pub fn get_profiles(&self) -> &Vec<Profile> { &self.list_of_profiles }
}