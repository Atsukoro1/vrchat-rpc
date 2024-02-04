use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use serde::{Serialize, Deserialize};

pub struct FileSystemClient;

pub const ROOT_FOLDER: &'static str = r"\VRChat-RPC-Client";
pub const ROOT_LOCATION: &'static str = r"C:\Users\Jakub\Desktop";

impl FileSystemClient {
    pub fn new() -> Self {
        FileSystemClient
    }

    pub fn ensure_folder_exists(&self, folder_path: &Path) -> io::Result<()> {
        if !folder_path.exists() {
            fs::create_dir_all(folder_path)?;
        }
        Ok(())
    }

    pub fn get_file_path(&self, file_name: &str, subfolder: Option<&str>) -> PathBuf {
        let root_folder = PathBuf::from(ROOT_LOCATION).join(ROOT_FOLDER);
        match subfolder {
            Some(sub) => root_folder.join(sub).join(file_name),
            None => root_folder.join(file_name),
        }
    }

    pub fn write_json<T: Serialize>(
        &self,
        file_name: &str,
        data: &T,
        subfolder: Option<&str>,
    ) -> io::Result<()> {
        let file_path = self.get_file_path(file_name, subfolder);
        self.ensure_folder_exists(&file_path.parent().unwrap())?;

        let json_data = serde_json::to_string(data)?;

        let mut file = File::create(&file_path)?;
        file.write_all(json_data.as_bytes())?;

        Ok(())
    }

    pub fn read_json<T: for<'a> Deserialize<'a>>(&self, file_name: &str, subfolder: Option<&str>) -> io::Result<T> {
        let file_path = self.get_file_path(file_name, subfolder);
        let mut file = File::open(&file_path)?;

        let mut json_data = String::new();

        file.read_to_string(&mut json_data)?;
        let result: T = serde_json::from_str(&json_data)?;
        
        Ok(result)
    }
}