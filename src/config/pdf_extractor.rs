use pdf_extract::extract_text;
use std::collections::HashSet;
use std::io::Result;
use std::path::Path;

pub fn extract_text_from_pdf(pdf_path: &str) -> Result<String> {
    let path = Path::new(pdf_path);
    if !path.exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "File not found",
        ));
    }

    // Extract text directly from the file path
    match extract_text(path) {
        Ok(text) => Ok(text),
        Err(_) => Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Could not extract text",
        )),
    }
}

pub fn match_score(resume_text: &str, tags: Option<Vec<String>>) -> f32 {
    // Step 1: Preprocess the resume text
    let resume_words: HashSet<String> = resume_text
        .to_lowercase() // Convert to lowercase for case-insensitive matching
        .split_whitespace() // Split into individual words
        .map(|s| s.trim_matches(|c: char| !c.is_alphanumeric()).to_string()) // Remove punctuation
        .collect();

    // Step 2: Check if tags are available, and proceed if they are
    if let Some(tags) = tags {
        // Calculate matches
        let total_tags = tags.len();
        let matched_tags = tags
            .iter()
            .filter(|&tag| resume_words.contains(&tag.to_lowercase()))
            .count();

        // Step 3: Calculate the score as a percentage
        if total_tags == 0 {
            0.0 // Return 0 if no tags provided
        } else {
            (matched_tags as f32 / total_tags as f32) * 100.0 // Score in percentage
        }
    } else {
        // If no tags are provided (None), return 0
        0.0
    }
}
