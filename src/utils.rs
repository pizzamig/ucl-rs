use libc::c_char;

use std::ffi::{
    CString,
    CStr
};

pub fn to_c_str<T : Into<Vec<u8>>>( string: T) -> CString
{
	CString::new(string).unwrap_or(CString::new("").unwrap())
}

pub fn to_str(cstring: *const c_char) -> Option<String> {
    if cstring.is_null() { return None }
    Some(unsafe { CStr::from_ptr(cstring).to_str() }.unwrap().to_string())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_to_c_str() {
		let test_str = "LOL";
		let rc = to_c_str( "LOL" );

		assert_eq!( rc.into_string(), Ok(test_str.to_string()));

		let rc2 = to_c_str( "LOL" );
		let back = to_str( rc2.as_ptr());
		assert_eq!( back, Some(test_str.to_string()));
	}

	#[test]
	fn test_to_c_str_2() {
		let rc = to_c_str( "LOL2".to_string() );

		assert_eq!( rc.into_string(), Ok("LOL2".to_string()));

		let rc2 = to_c_str( "LOL2".to_string() );
		let back = to_str( rc2.as_ptr());
		assert_eq!( back, Some("LOL2".to_string()));
	}
}
