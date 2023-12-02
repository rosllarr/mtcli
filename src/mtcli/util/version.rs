use std::fmt;

/// mtcli's version
pub struct VersionInfo {
    pub version: String,
}

impl fmt::Display for VersionInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.version)?;
        Ok(())
    }
}

/// Return information about mtcli's version.
pub fn version() -> VersionInfo {
    let minor = 0;
    let patch = 0;
    let version = format!("1.{}.{}", minor, patch);

    VersionInfo { version }
}
