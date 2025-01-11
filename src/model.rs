use reqwest::blocking::get;
use std::error::Error;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use xdg::BaseDirectories;

#[derive(Debug)]
pub struct Model {
    pub lang: String,
    pub download_url: String,
}

impl Model {
    // Method to download and install the model
    pub fn install(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_installed() {
            base_directories()
                .create_cache_directory("downloads")
                .unwrap();

            if !Path::exists(Path::new(&self.get_download_path())) {
                self.download().unwrap();
            }

            self.extract().unwrap();

            println!("Model for {} installed successfully!", self.lang);
        } else {
            println!("Model for {} already exists.", self.lang);
        }
        Ok(())
    }

    fn download(&self) -> Result<(), Box<dyn Error>> {
        println!("Downloading model for language: {}", self.lang);

        let response = get(&self.download_url)?;

        let mut file = fs::File::create(self.get_download_path())?;
        let bytes = response.bytes()?;
        file.write_all(&bytes)?;

        println!("Finished downloading model for language: {}", self.lang);

        Ok(())
    }

    fn extract(&self) -> Result<(), Box<dyn Error>> {
        println!("Extracting model for language: {}", self.lang);

        base_directories().create_data_directory("models").unwrap();

        let file = fs::File::open(self.get_download_path()).unwrap();

        let mut archive = zip::ZipArchive::new(file).unwrap();

        archive
            .extract(format!(
                "{}models",
                base_directories().get_data_home().to_str().unwrap()
            ))
            .unwrap();

        let prefix = "https://alphacephei.com/vosk/models/";

        if let Some(suffix) = self.download_url.strip_prefix(prefix) {
            println!("Suffix: {}", suffix);
            let model_name = suffix.strip_suffix(".zip").unwrap();
            let parent = format!(
                "{}models",
                base_directories().get_data_home().to_str().unwrap()
            );
            let old_path = format!("{}/{}", parent, model_name);
            let new_path = format!("{}/{}", parent, self.lang);
            fs::rename(&Path::new(&old_path), Path::new(&new_path))?;
        } else {
            println!("The URL does not start with the specified prefix.");
        }

        println!("Finished extracting model for language: {}", self.lang);

        Ok(())
    }

    pub fn get_local_path(&self) -> String {
        format!(
            "{}models/{}",
            base_directories().get_data_home().to_str().unwrap(),
            self.lang,
        )
    }

    pub fn get_download_path(&self) -> String {
        format!(
            "{}downloads/model_{}.zip",
            base_directories().get_cache_home().to_str().unwrap(),
            self.lang,
        )
    }

    pub fn is_installed(&self) -> bool {
        Path::new(&self.get_local_path()).exists()
    }
}

pub fn available_models() -> Vec<Model> {
    vec![
        Model {
            lang: "en".to_string(),
            download_url: "https://alphacephei.com/vosk/models/vosk-model-small-en-us-0.15.zip"
                .to_string(),
        },
        Model {
            lang: "es".to_string(),
            download_url: "https://alphacephei.com/vosk/models/vosk-model-small-es-0.42.zip"
                .to_string(),
        },
        Model {
            lang: "fr".to_string(),
            download_url: "https://alphacephei.com/vosk/models/vosk-model-small-fr-0.22.zip"
                .to_string(),
        },
    ]
}

// Function to select a model based on language
pub fn get_model_by_lang(lang: &str) -> Option<Model> {
    let models = available_models();
    models.into_iter().find(|model| model.lang == lang)
}

fn base_directories() -> BaseDirectories {
    BaseDirectories::with_prefix("stenograph").unwrap()
}
