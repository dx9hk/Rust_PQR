use std::path::PathBuf;

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
    pub fn new(profile_path: PathBuf) -> Self {
        // Do basic dir checks
        if !profile_path.is_dir() {
            panic!("Directory input is not valid.");
        }
        // Setup local variables for return
        let mut profile_name = "".to_string();
        let mut rotation_path = PathBuf::default();
        let mut abilities_path = PathBuf::default();
        // Get rotation and abilities file
        profile_path
            .read_dir()
            .unwrap()
            .for_each(|child_dir| {
            // Check if file is a directory
            if child_dir.unwrap().file_type().unwrap().is_dir() {
                // Loop through files within child directory
                let path_child_dir = PathBuf::from(child_dir.unwrap().path());
                path_child_dir
                    .read_dir()
                    .unwrap()
                    .for_each(|child_dir| {
                        // Find and extract relevant information
                        if profile_name.is_empty() {
                            profile_name = String::from(path_child_dir.file_name().unwrap().to_str().unwrap());
                        }
                        // Check if child directory has rotation or abilities in the name
                        if child_dir.unwrap().path().to_str().unwrap().contains("Abilities") {
                            abilities_path = child_dir.unwrap().path();
                        }
                        else if child_dir.unwrap().path().to_str().unwrap().contains("Rotations") {
                            rotation_path = child_dir.unwrap().path();
                        }
                });



            }
        });
        // Get profile name from folder name
        Self {
            profile_name: "".to_string(),
            profile_path,
            rotation_path: Default::default(),
            abilities_path: Default::default(),
        }
    }
}
