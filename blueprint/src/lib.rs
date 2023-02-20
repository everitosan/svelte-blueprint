use std::path::PathBuf;

use std::ffi::{CStr};

extern crate lazy_static;
extern crate libc;

pub mod modules;
pub mod infra;

#[no_mangle]
pub extern fn blueprint(source_libc:  *const libc::c_char, destination_libc:  *const libc::c_char, template_libc:  *const libc::c_char) -> bool {  

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

  let params = modules::blueprint::DocumentParams {
    template: &template_path,
    destination: &destination_path
  };

  let bp = modules::blueprint::Blueprint::new(params);

  match bp.parse(&source_path) {
    Ok(_) => { return true },
    Err(_) => { return false }
  }
}