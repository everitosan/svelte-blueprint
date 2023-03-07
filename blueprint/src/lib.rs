use std::path::PathBuf;

use std::ffi::{CStr};
use std::ffi::CString;
use libc::c_char;

extern crate lazy_static;
extern crate libc;

pub mod modules;
pub mod infra;


#[no_mangle]
pub extern fn blueprint(source_libc:  *const libc::c_char, destination_libc:  *const libc::c_char, template_libc:  *const libc::c_char) -> *mut c_char  {  

  let source_str = unsafe { CStr::from_ptr(source_libc) };
  let source_str = source_str.to_str().unwrap();

  let destination_str = unsafe { CStr::from_ptr(destination_libc) };
  let destination_str = destination_str.to_str().unwrap();

  let template_str = unsafe { CStr::from_ptr(template_libc) };
  let template_str = template_str.to_str();

  let source_path = PathBuf::from(source_str);
  let destination_path = PathBuf::from(destination_str);
  let mut template_path = None;

  if let Ok(template_str) = template_str {
    template_path = Some(PathBuf::from(template_str));
  }

  let params = modules::blueprint::document::DocumentParams {
    template: &template_path,
    destination: &destination_path
  };

  let paths = modules::blueprint::process(&source_path, params);

  let result_string = paths.join("|");
  CString::new(result_string).unwrap().into_raw()

}