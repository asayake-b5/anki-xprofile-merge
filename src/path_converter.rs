use std::{
    fs,
    path::{Path, PathBuf},
};

use regex::Regex;

pub struct PathConverter {
    pub bank_root: PathBuf,
    pub main_root: PathBuf,
    pub regex_sound: Regex,
}

impl PathConverter {
    pub fn new(bank_profile: &str, main_profile: &str) -> Self {
        let mut main_root = dirs::data_dir().unwrap();
        main_root.push("Anki2");
        main_root.push(main_profile);
        main_root.push("collection.media");
        if !main_root.as_path().exists() {
            eprintln!("Path {} does not exist, did you provide the right profile name, and are you using anki-morphs?", main_root.display());
            panic!("handle this idk");
        }

        let mut bank_root = dirs::data_dir().unwrap();
        bank_root.push("Anki2");
        bank_root.push(bank_profile);
        bank_root.push("collection.media");
        if !bank_root.as_path().exists() {
            eprintln!("Path {} does not exist, did you provide the right profile name, and are you using anki-morphs?", bank_root.display());
            panic!("handle this idk");
        }

        let regex_sound = Regex::new(r"\[sound:(.*)\]").unwrap();

        Self {
            bank_root,
            main_root,
            regex_sound,
        }
    }

    pub fn sound_to_path(&self, sound: &str) -> Option<PathBuf> {
        let filename = self.regex_sound.replace(sound, "$1").to_string();
        let mut r = self.bank_root.clone();
        r.push(filename.clone());
        let mut r_main = self.main_root.clone();
        r_main.push(filename);
        //TODO look into this better
        // if r_main.exists() {
        //     eprintln!(
        //         "path {} already exists, it would be overwritten",
        //         r_main.display()
        //     );
        //     Some(r)
        // } else {
        Some(r)
        // }
    }
}
