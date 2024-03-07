use mime_guess2::Mime;

use tuo_shared::errors::utils::TuoUtilError;

/// Get the mime type of file
///
/// # Arguments
///
/// * `file_path` - A string slice that holds the path to the file
///
/// # Returns
///
/// * `Option<String>` - The string representation of the mime type of the file
pub fn get_mime_type(file_path: &str) -> Result<Mime, TuoUtilError> {
    // check if the file path is valid
    if !std::path::Path::new(file_path).exists() {
        return Err(TuoUtilError::InvalidFilePath(file_path.to_string()));
    }

    match mime_guess2::from_path(file_path).first() {
        None => Err(TuoUtilError::CannotDetermineFileType(file_path.to_string())),
        Some(mime) => Ok(mime),
    }
}

pub fn get_mime_type_str(file_path: &str) -> Result<String, TuoUtilError> {
    let mime = get_mime_type(file_path)?;
    Ok(mime.to_string())
}

pub fn get_mime_type_from_extension(extension: &str) -> Result<Mime, TuoUtilError> {
    match mime_guess2::MimeGuess::from_ext(extension).first() {
        None => Err(TuoUtilError::CannotDetermineFileType(extension.to_string())),
        Some(mime) => Ok(mime),
    }
}

pub fn get_mime_type_str_from_extension(extension: &str) -> Result<String, TuoUtilError> {
    let mime = get_mime_type_from_extension(extension)?;
    Ok(mime.to_string())
}
