use std::{fs::{self, DirEntry}, io, path::Path};
use super::*;

pub fn call(init: &CliParams) -> () {
    match init.command {
        "create" => create::handle(init),
        "read" => read::handle(init),
        "delete" => delete::handle(init),
        _ => Err(io::Error::new(io::ErrorKind::Other, 
            "Invalid command: Please review command and start again ...")),
    };
    
}

mod create {
    use super::*;

    pub fn handle(init: &CliParams) -> Result<(), io::Error> {
        println!("you chose to create a dir");
        Ok(())
    }
}

mod read {
    use super::*;

    pub fn handle(init: &CliParams) -> Result<(), io::Error> {
        let pth = Path::new(init.name);
        if pth.is_dir() {
            visit_dir(pth);
        } else {
            println!("not a path or dir");
            println!("{:?}", pth);
        }
        Ok(())
    }

    fn visit_dir(path: &Path) -> Result<(), io::Error> {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                format::format_dir(&path);
                visit_dir(&path)?;
            } else if path.is_file() {
                format::format_file(&path);                
            }
        }
        Ok(())
    }
}

mod delete {
    use super::*;

    pub fn handle(init: &CliParams) -> Result<(), io::Error> {
        println!("you chose to delete a dir");
        Ok(())
    }
}

mod format {
    use std::path::PathBuf;

    pub fn format_dir(path: &PathBuf) {
        println!("\n->{:?}", path);
    }

    pub fn format_file(path: &PathBuf) {
        println!("| \n--{:?}", path);
    }
}


