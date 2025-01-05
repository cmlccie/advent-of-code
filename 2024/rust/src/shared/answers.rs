use std::path::Path;

/*-------------------------------------------------------------------------------------------------
  Answers
-------------------------------------------------------------------------------------------------*/

pub fn answer<P: AsRef<Path> + ?Sized>(file_path: &P) -> Option<String> {
    Some(std::fs::read_to_string(file_path).ok()?.trim().to_string())
}
