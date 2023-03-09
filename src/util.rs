use std::path::{Path, PathBuf};

pub trait ExpandHome {
    fn expand(&self) -> Option<PathBuf>;
    fn unexpand(&self) -> Option<PathBuf>;
}

impl<P: AsRef<Path>> ExpandHome for P {
    // https://stackoverflow.com/a/54306906
    fn expand(&self) -> Option<PathBuf> {
        let p = self.as_ref();
        if !p.starts_with("~") {
            return Some(p.to_path_buf());
        }
        if p == Path::new("~") {
            return dirs::home_dir();
        }
        dirs::home_dir().map(|mut h| {
            if h == Path::new("/") {
                // Corner case: `h` root directory;
                // don't prepend extra `/`, just drop the tilde.
                p.strip_prefix("~").unwrap().to_path_buf()
            } else {
                h.push(p.strip_prefix("~/").unwrap());
                h
            }
        })
    }

    fn unexpand(&self) -> Option<PathBuf> {
        let p = self.as_ref();
        let home = dirs::home_dir()?;
        if !p.starts_with(&home) {
            return Some(p.to_path_buf());
        }
        if p == dirs::home_dir()? {
            return Some(Path::new("~").to_path_buf());
        }
        Some(Path::new("~").join(p.strip_prefix(home).ok()?))
    }
}

#[test]
fn expand() {
    let path = PathBuf::from("~/projects/");
    assert_eq!(
        path.expand().unwrap(),
        PathBuf::from("/home/willothy/projects/")
    );
    assert_eq!(path.expand().unwrap().unexpand().unwrap(), path);
}
