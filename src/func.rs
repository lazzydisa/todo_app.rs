// checks for existing of .config/apps directory (if not then directory will be created)
pub fn check_dir(path: &String) -> Result<(), String> {
    use std::fs;

    match fs::exists(&path) {
        Ok(true) => Ok(()),

        Ok(false) => {
            // if else if else darkness :/
            if let Ok(test) = fs::create_dir_all(path) {
                if test != () {
                    Err(String::from("Can't create ~/.config/apps directory"))
                } else {
                    Ok(())
                }
            } else {
                Ok(())
            }
        },

        Err(_) => Err(String::from("Can't read ~/.config directory")),
    }
}
