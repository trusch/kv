use std::io::Write;
use std::process::{Command, Stdio};

pub trait Storage {
    fn get(&self, key: &str) -> Result<String, Error>;
    fn set(&mut self, key: &str, value: &str) -> Result<(), Error>;
    fn list(&self, prefix: &str) -> Result<Vec<String>, Error>;
    fn remove(&mut self, key: &str, recursive: bool) -> Result<(), Error>;

    fn save<T: serde::Serialize>(&mut self, key: &str, val: T) -> Result<(), Error> {
        let bs = serde_json::to_string(&val)?;
        self.set(key, &bs)
    }

    fn load<T: serde::de::DeserializeOwned>(&self, key: &str) -> Result<T, Error> {
        let data = self.get(key)?;
        let res: T = serde_json::from_str(&data)?;
        Ok(res)
    }
}

#[derive(Debug, Clone)]
pub struct GpgStorage {
    base_path: String,
    gpg_id: Option<String>,
}

impl Storage for GpgStorage {
    fn get(&self, key: &str) -> Result<String, Error> {
        log::debug!("Decrypting {}", key);
        let mut cmd = Command::new("gpg");
        cmd.arg("--decrypt");
        cmd.arg("--output");
        cmd.arg("-");
        cmd.arg("--default-recipient-self");
        if let Some(gpg_id) = &self.gpg_id {
            log::debug!("using gpg id: {}", gpg_id);
            cmd.arg("--recipient");
            cmd.arg(gpg_id);
        }
        cmd.arg(self.base_path.to_owned() + "/" + key + ".gpg");
        let output = cmd.output()?;
        if output.status.success() {
            Ok(String::from_utf8(output.stdout).unwrap())
        } else {
            Err(Error::InvalidKey(String::from_utf8(output.stderr).unwrap()))
        }
    }

    fn set(&mut self, key: &str, value: &str) -> Result<(), Error> {
        log::debug!("Encrypting {}", key);
        self.ensure_dir(key)?;

        let mut cmd = Command::new("gpg");
        cmd.arg("--encrypt");
        cmd.arg("--output");
        cmd.arg(self.base_path.to_owned() + "/" + key + ".gpg");
        cmd.arg("--default-recipient-self");
        if let Some(gpg_id) = &self.gpg_id {
            log::debug!("using gpg id: {}", gpg_id);
            cmd.arg("--recipient");
            cmd.arg(gpg_id);
        }
        cmd.arg("-");
        cmd.stdin(std::process::Stdio::piped());
        let mut child = cmd.spawn()?;
        child
            .stdin
            .as_mut()
            .ok_or_else(|| Error::IO("Failed to open child stdin".to_string()))?
            .write_all(value.as_bytes())?;

        let output = child.wait_with_output()?;

        if output.status.success() {
            Ok(())
        } else {
            Err(Error::InvalidKey(String::from_utf8(output.stderr).unwrap()))
        }
    }

    fn remove(&mut self, key: &str, recursive: bool) -> Result<(), Error> {
        log::debug!("Removing {}", key);
        let path = self.base_path.to_owned() + "/" + key;
        if std::path::Path::new(&path).is_dir() {
            log::debug!("{} is a directory", key);
            if recursive {
                log::debug!("Removing recursively");
                std::fs::remove_dir_all(path)?;
            } else {
                log::error!("{} is a directory, use -r to remove recursively", key);
                return Err(Error::InvalidKey("Key is a directory".to_string()));
            }
        } else {
            std::fs::remove_file(path + ".gpg")?;
        }
        Ok(())
    }

    fn list(&self, dir: &str) -> Result<Vec<String>, Error> {
        log::debug!("listing {}", dir);
        Ok(walkdir::WalkDir::new(self.base_path.to_owned())
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .map(|e| {
                e.path()
                    .to_str()
                    .unwrap()
                    .trim_start_matches(&self.base_path)
                    .trim_start_matches("/")
                    .to_string()
            })
            .filter(|p| !p.starts_with('.'))
            .map(|e| e.replace(".gpg", ""))
            .filter(|s| s.starts_with(dir))
            .collect())
    }
}

impl GpgStorage {
    pub fn new(base_path: &str, gpg_id: Option<String>) -> GpgStorage {
        GpgStorage {
            base_path: base_path.to_string(),
            gpg_id,
        }
    }

    fn ensure_dir(&mut self, path: &str) -> Result<(), Error> {
        let path = std::path::Path::new(path);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(self.base_path.to_owned() + "/" + parent.to_str().unwrap())?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum Error {
    InvalidKey(String),
    IO(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::InvalidKey(s) => write!(f, "Invalid key: {}", s),
            Error::IO(s) => write!(f, "IO error: {}", s),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IO(e.to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::IO(format!("failed to deserialize: {}", e))
    }
}

impl std::error::Error for Error {}

#[cfg(test)]
mod test {

    #[test]
    fn test_gpg_storage() {
        use super::{GpgStorage, Storage};

        let dir = tempfile::tempdir().unwrap();
        let mut storage = GpgStorage::new(dir.path().to_str().unwrap(), None);
        let key = "foo/bar/test_key";
        let value = "test_value";
        storage.set(key, value).unwrap();
        let result = storage.get(key).unwrap();
        assert_eq!(result, value);
        storage.remove(key, true).unwrap();
        let result = storage.get(key);
        assert!(result.is_err());
    }

    
}

pub struct GitStorage<S: Storage> {
    storage: S,
    base_path: String,
}

impl<S: Storage> GitStorage<S> {
    pub fn new(storage: S, base_path: &str) -> GitStorage<S> {
        // check if git is initialized
        let mut cmd = Command::new("git");
        cmd.current_dir(base_path);
        cmd.arg("status");
        let output = cmd.output().unwrap();
        if !output.status.success() {
            // git is not initialized, initialize it
            log::debug!("Initializing git repository");
            let mut cmd = Command::new("git");
            cmd.current_dir(base_path);
            cmd.arg("init");
            cmd.arg("-b");
            cmd.arg("main");
            cmd.output().unwrap();
        }
        GitStorage {
            storage,
            base_path: base_path.to_string(),
        }
    }

    pub fn pull(&self) -> Result<(), Error> {
        log::debug!("Pulling changes");
        let mut cmd = Command::new("git");
        cmd.current_dir(&self.base_path);
        cmd.arg("pull");
        cmd.stdout(Stdio::inherit());
        cmd.stderr(Stdio::inherit());
        let out = cmd.output()?;
        if !out.status.success() {
            return Err(Error::IO(format!(
                "git pull failed with status code {}",
                out.status
            )));
        }
        Ok(())
    }

    pub fn push(&self) -> Result<(), Error> {
        log::debug!("Pushing changes");
        let mut cmd = Command::new("git");
        cmd.current_dir(&self.base_path);
        cmd.arg("push");
        cmd.stdout(Stdio::inherit());
        cmd.stderr(Stdio::inherit());
        let out = cmd.output()?;
        if !out.status.success() {
            return Err(Error::IO(format!(
                "git push failed with status code {}",
                out.status
            )));
        }
        Ok(())
    }

    fn commit(&mut self, msg: &str) -> Result<(), Error> {
        log::debug!("Committing changes");
        let mut cmd = Command::new("git");
        cmd.current_dir(&self.base_path);
        cmd.arg("add");
        cmd.arg(".");
        cmd.output()?;

        let mut cmd = Command::new("git");
        cmd.current_dir(&self.base_path);
        cmd.arg("commit");
        cmd.arg("-m");
        cmd.arg(msg);
        cmd.output()?;
        Ok(())
    }
}

impl<S> Storage for GitStorage<S>
where
    S: Storage,
{
    fn get(&self, key: &str) -> Result<String, Error> {
        self.storage.get(key)
    }

    fn set(&mut self, key: &str, value: &str) -> Result<(), Error> {
        self.storage.set(key, value)?;
        self.commit(&("saved ".to_string() + key))
    }

    fn list(&self, dir: &str) -> Result<Vec<String>, Error> {
        self.storage.list(dir)
    }

    fn remove(&mut self, key: &str, recursive: bool) -> Result<(), Error> {
        self.storage.remove(key, recursive)?;
        self.commit(&("removed ".to_string() + key))
    }
}
