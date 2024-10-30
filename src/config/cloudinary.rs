use serde_json::Value;
use std::env;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

pub struct CloudinaryConfig {
    pub cloud_name: String,
    #[allow(dead_code)]
    pub api_key: String,
    #[allow(dead_code)]
    pub api_secret: String,
}

impl CloudinaryConfig {
    pub fn new() -> Self {
        dotenv::dotenv().ok(); // Load environment variables from .env file

        CloudinaryConfig {
            cloud_name: env::var("CLOUDINARY_CLOUD_NAME").expect("Cloud name must be set"),
            api_key: env::var("CLOUDINARY_API_KEY").expect("API key must be set"),
            api_secret: env::var("CLOUDINARY_API_SECRET").expect("API secret must be set"),
        }
    }

    // Upload an image to Cloudinary
    pub async fn upload_image(
        &self,
        image_path: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let url = format!(
            "https://api.cloudinary.com/v1_1/{}/image/upload",
            self.cloud_name
        );

        let mut file = File::open(image_path).await.map_err(|e| {
            println!("Error opening file: {}", e); // Log or handle the IO error as needed
            e
        })?;

        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).await.map_err(|e| {
            println!("Error reading file: {}", e); // Log or handle the IO error as needed
            e
        })?;

        // Asynchronous network request with reqwest
        let client = reqwest::Client::new();
        let part = reqwest::multipart::Part::bytes(buffer).file_name("upload.png");

        let form = reqwest::multipart::Form::new()
            .part("file", part)
            .text("upload_preset", "my-preset");

        let response = client.post(url).multipart(form).send().await?;

        // let response_text = response.text().await?;

        // info!("{}", response_text);

        let response_json: Value = response.json().await?; // Parse response as JSON

        let image_url = response_json["secure_url"]
            .as_str()
            .ok_or("Failed to get secure URL")?;

        Ok(image_url.to_string())
    }
}
