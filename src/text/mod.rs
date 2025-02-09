pub use ansiterm;

// CString
pub use std::ffi::{CStr,
                   CString};

// CString extension
pub trait DfwCString
{
	fn new_filled_with(fill_character : u8,
	                   len : usize)
	                   -> CString
	{
		// allocate buffer of correct size
		let mut buffer : Vec<u8> = Vec::with_capacity(len + 1);
		// fill it with len spaces
		buffer.extend([fill_character].iter().cycle().take(len));
		// convert buffer to CString
		unsafe { CString::from_vec_unchecked(buffer) }
	}
}
impl DfwCString for CString {}
