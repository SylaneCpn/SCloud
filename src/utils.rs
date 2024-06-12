/*Utility functions*/

//###########################################################################################//

//add slash to path if request doesn't ends whit "/"
pub fn slash_path(path: &str) -> String {
    let mut added = String::from(path);
    if !added.ends_with("/") {
        added += "/";
    }
    added
}

//trim path in full-path for repond_dir if request ends whit "/"
pub fn trim_path(path: &str) -> String {
    let mut trimmed = String::from(path);
    if trimmed.ends_with("/") {
        trimmed.pop();
    }
    trimmed
}
