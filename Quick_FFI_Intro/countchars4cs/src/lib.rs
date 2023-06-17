use std::os::raw::c_char;
use std::ffi::CStr;
use std::ffi::CString;

#[no_mangle]
pub extern "C" fn count_characters(ptr: *const c_char) -> u32 {

    // Dereference and wrap the incoming raw pointer.
    let c_string = unsafe {
        assert!(!ptr.is_null());

        CStr::from_ptr(ptr)
    };

    // Convert into a rust string.
    let rust_string = c_string.to_str().unwrap();

    // Return the number of characters.
    rust_string.chars().count() as u32
}

// https://notes.huy.rocks/en/string-ffi-rust.html
#[no_mangle]
pub extern fn string_from_rust() -> *const c_char {
    let s = CString::new("Hello World from Rust").unwrap();
    let p = s.as_ptr();
    std::mem::forget(s);
    p
}

// https://stackoverflow.com/questions/30154541/how-do-i-concatenate-strings
#[no_mangle]
pub extern fn echo_from_rust(ptr: *const c_char) -> *const c_char {
    // Dereference and wrap the incoming raw pointer.
    let c_string = unsafe {
        assert!(!ptr.is_null());

        CStr::from_ptr(ptr)
    };

    // Convert into a rust string and concatenate
    let rust_string1 = c_string.to_str().unwrap();
    let rust_string2: &str = " - echoed right back at you!";
    let rust_string12 = format!("{}{}", rust_string1, rust_string2);

    let s = CString::new(rust_string12).unwrap();
    let p = s.as_ptr();
    std::mem::forget(s);
    p
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn internal() {

	    // Simulate a call over the C interface by creating a CString.
	    let message = "Hello World!";
        let c_string = CString::new(message).expect("CString::new failed");

	    // Convert the c_string into a raw c_pointer. 
        let c_pointer = c_string.into_raw();

	    // Call into the library function to count the characters.
        let count = count_characters(c_pointer);
        println!("\nThe number of characters in {} = {}", message, count);
    }
}
