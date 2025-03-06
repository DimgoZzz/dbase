#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Ord)]
pub struct Version {
	major: u32,
	minor: u32,
	patch: u32,
}

impl Version {
	pub fn new(
		major: u32,
		minor: u32,
		patch: u32,
	) -> Version {
		Self {
			major,
			minor,
			patch,
		}
	}

	pub fn set_major(
		&mut self,
		new_major: u32,
	) {
		self.major = new_major;
	}

	pub fn set_minor(
		&mut self,
		new_minor: u32,
	) {
		self.minor = new_minor;
	}

	pub fn set_patch(
		&mut self,
		new_patch: u32,
	) {
		self.patch = new_patch;
	}

	pub fn get_major(&self) -> u32 {
		self.major
	}

	pub fn get_minor(&self) -> u32 {
		self.minor
	}

	pub fn get_patch(&self) -> u32 {
		self.patch
	}
}

impl std::fmt::Display for Version {
	fn fmt(
		&self,
		f: &mut std::fmt::Formatter<'_>,
	) -> std::fmt::Result {
		write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
	}
}

impl PartialOrd for Version {
	fn partial_cmp(
		&self,
		other: &Self,
	) -> Option<std::cmp::Ordering> {
		match self.major.partial_cmp(&other.major) {
			Some(core::cmp::Ordering::Equal) => {},
			ord => return ord,
		}
		match self.minor.partial_cmp(&other.minor) {
			Some(core::cmp::Ordering::Equal) => {},
			ord => return ord,
		}
		self.patch.partial_cmp(&other.patch)
	}
}
