use std::path::PathBuf;

#[derive(Debug, PartialEq, Clone)]
pub struct Profile {
    // Name of the profile
    profile_name: String,
    // Path for the profile
    profile_path: PathBuf,
    // Path for rotation xml file within profile
    rotation_path: PathBuf,
    // Path for abilities xml file within profile
    abilities_path: PathBuf,
}

impl Profile {
    /// Gets profile name
    pub fn get_profile_name(&self) -> String { self.profile_name.clone() }
    /// Gets profile path
    pub fn get_profile_path(&self) -> PathBuf { self.profile_path.clone() }
    /// Gets rotation path
    pub fn get_rotation_path(&self) -> PathBuf { self.rotation_path.clone() }
    /// Gets abilities path
    pub fn get_abilities_path(&self) -> PathBuf { self.abilities_path.clone() }
    /// Default constructor
    pub fn default() -> Self {
        Self {
            profile_name: String::default(),
            profile_path: PathBuf::default(),
            rotation_path: PathBuf::default(),
            abilities_path: PathBuf::default()
        }
    }
    /// Construct to input path to profile folder
    pub fn new(profile_path: PathBuf) -> Self {
        // Do basic dir checks
        if !profile_path.is_dir() {
            panic!("Directory input is not valid.");
        }
        // Setup local variables for return
        let mut profile_name = "".to_string();
        let mut rotation_path = PathBuf::default();
        let mut abilities_path = PathBuf::default();
        // Loop through files within child directory
        profile_path
            .read_dir()
            .unwrap()
            .for_each(|child_dir| {
                let curr_dir = child_dir.unwrap().path();
                // Find and extract relevant information
                if profile_name.is_empty() {
                    profile_name = String::from(profile_path.file_name().unwrap().to_str().unwrap());
                }
                // Check if child directory has rotation or abilities in the name
                if curr_dir.to_str().unwrap().contains("Abilities") {
                    abilities_path = curr_dir;
                }
                else if curr_dir.to_str().unwrap().contains("Rotations") {
                    rotation_path = curr_dir;
                }
            });
        // Ready constructor for return
        Self {
            profile_name,
            profile_path,
            rotation_path,
            abilities_path,
        }
    }
}
