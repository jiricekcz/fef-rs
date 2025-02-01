//! Module for runtime handling of version information.

/// Description of a version of the FEF specification.
///
/// Holds information about the major, minor, and micro version of the FEF specification.
///
/// # Examples
///
/// Getting a version string:
/// ```rust
/// let version = fef::common::version::SpecVersion::new(1, 2, 3);
/// assert_eq!(format!("{}", version), "v1.2.3");
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct SpecVersion {
    major: u32,
    minor: u32,
    micro: u32,
}

impl SpecVersion {
    /// Creates a new `SpecVersion` with the given major, minor, and micro version numbers.
    ///
    /// # Examples
    /// ```rust
    /// # use fef::common::version::SpecVersion;
    /// let version = SpecVersion::new(1, 2, 3);
    /// assert_eq!(version.major(), 1);
    /// assert_eq!(version.minor(), 2);
    /// assert_eq!(version.micro(), 3);
    /// ```
    pub const fn new(major: u32, minor: u32, micro: u32) -> SpecVersion {
        SpecVersion {
            major: major,
            minor: minor,
            micro: micro,
        }
    }

    /// Returns the major version number.
    ///
    /// # Examples
    /// ```rust
    /// # use fef::common::version::SpecVersion;
    /// let version = SpecVersion::new(1, 2, 3);
    /// assert_eq!(version.major(), 1);
    /// ```
    pub const fn major(&self) -> u32 {
        self.major
    }

    /// Returns the minor version number.
    ///
    /// # Examples
    /// ```rust
    /// # use fef::common::version::SpecVersion;
    /// let version = SpecVersion::new(1, 2, 3);
    /// assert_eq!(version.minor(), 2);
    /// ```
    pub const fn minor(&self) -> u32 {
        self.minor
    }

    /// Returns the micro version number.
    ///
    /// # Examples
    /// ```rust
    /// # use fef::common::version::SpecVersion;
    /// let version = SpecVersion::new(1, 2, 3);
    /// assert_eq!(version.micro(), 3);
    /// ```
    pub const fn micro(&self) -> u32 {
        self.micro
    }
}

/// Formats the version as a string when displayed in the format "v{MAJOR}.{MINOR}.{MICRO}".
///
/// # Examples
/// ```rust
/// # use fef::common::version::SpecVersion;
/// let version = SpecVersion::new(1, 2, 3);
/// assert_eq!(format!("{}", version), "v1.2.3");
/// ```
impl std::fmt::Display for SpecVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "v{}.{}.{}", self.major, self.minor, self.micro)
    }
}

/// Formats the version as a string when displayed in the format "SpecVersion({MAJOR}.{MINOR}.{MICRO})".
///
/// # Examples
/// ```rust
/// # use fef::common::version::SpecVersion;
/// let version = SpecVersion::new(1, 2, 3);
/// assert_eq!(format!("{:?}", version), "SpecVersion(1.2.3)");
/// ```
impl std::fmt::Debug for SpecVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "SpecVersion({}.{}.{})",
            self.major, self.minor, self.micro
        )
    }
}

/// Implements the default value for `SpecVersion` as `SpecVersion(0.0.0)`.
///
/// # Examples
/// ```rust
/// # use fef::common::version::SpecVersion;
/// let version = SpecVersion::default();
/// assert_eq!(version, SpecVersion::new(0, 0, 0));
/// ```
impl Default for SpecVersion {
    fn default() -> Self {
        SpecVersion::new(0, 0, 0)
    }
}

/// Versions are ordered by their major, minor, and micro version numbers lexicographically.
///
/// # Examples
/// ```rust
/// # use fef::common::version::SpecVersion;
/// let version1 = SpecVersion::new(1, 2, 3);
/// let version2 = SpecVersion::new(1, 2, 4);
/// let version3 = SpecVersion::new(1, 3, 0);
/// let version4 = SpecVersion::new(2, 0, 0);
///
/// assert!(version1 < version2);
/// assert!(version2 < version3);
/// assert!(version3 < version4);
/// ```
impl Ord for SpecVersion {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.major != other.major {
            self.major.cmp(&other.major)
        } else if self.minor != other.minor {
            self.minor.cmp(&other.minor)
        } else {
            self.micro.cmp(&other.micro)
        }
    }
}

impl PartialOrd for SpecVersion {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
