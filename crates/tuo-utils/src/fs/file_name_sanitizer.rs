use sanitize_filename::{is_sanitized, sanitize};

/// # Details
/// sanitize-filename removes the following:
///
///  Control characters (0x00–0x1f and 0x80–0x9f)
/// Reserved characters (/, ?, <, >, \, :, *, |, and ")
/// Unix reserved filenames (. and ..)
/// Trailing periods and spaces (for Windows)
/// Windows reserved filenames (CON, PRN, AUX, NUL, COM1, COM2, COM3, COM4, COM5, COM6, COM7, COM8, COM9, LPT1, LPT2, LPT3, LPT4, LPT5, LPT6, LPT7, LPT8, and LPT9)
// /The resulting string is truncated to 255 bytes in length. The string will not contain any directory paths and will be safe to use as a filename.
pub fn sanitize_file_name(unsanitized_path: &str) -> String {
    sanitize(unsanitized_path)
}

/// Check if file is sanitized, i.e. it does not contain any illegal characters.
pub fn is_file_name_valid(file_name: &str) -> bool {
    is_sanitized(file_name)
}
