use std::fs::read_to_string;

/*-------------------------------------------------------------------------------------------------
  Inputs
-------------------------------------------------------------------------------------------------*/

pub fn get_input<P: AsRef<std::path::Path> + ?Sized>(file_path: &P) -> String {
    read_to_string(file_path).unwrap()
}
