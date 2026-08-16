use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_longlong};

mod mod_todo;
mod mod_ssr;

#[no_mangle]
pub extern "C" fn get_htm_todo() -> *mut c_char {
    let resp = mod_ssr::get_htm_todo();

    CString::new(resp)
        .unwrap()
        .into_raw()
}

fn c_str_to_string(input: *const c_char) -> Option<String> {
    if input.is_null() {
        return None;
    }
    let cstr = unsafe { CStr::from_ptr(input) };
    cstr.to_str().ok().map(|s| s.to_owned())
}

fn to_json_ptr(result: std::result::Result<String, Box<dyn std::error::Error>>) -> *mut c_char {
    match result {
        Ok(s) => CString::new(s)
            .map(|c| c.into_raw())
            .unwrap_or(std::ptr::null_mut()),
        Err(e) => {
            eprintln!("Rustエラー: {}", e);
            let msg = format!(
                "{{\"ret\":\"NG\",\"message\":\"{}\"}}",
                e.to_string().replace('"', "\\\"")
            );
            CString::new(msg)
                .map(|c| c.into_raw())
                .unwrap_or(std::ptr::null_mut())
        }
    }
}

#[no_mangle]
pub extern "C" fn todo_add(input: *const c_char) -> *mut c_char {
    let input_str = match c_str_to_string(input) {
        Some(s) => s,
        None => return std::ptr::null_mut(),
    };
    to_json_ptr(mod_todo::add_handler(&input_str))
}

#[no_mangle]
pub extern "C" fn todo_list() -> *mut c_char {
    to_json_ptr(mod_todo::list_todo_json(true))
}

#[no_mangle]
pub extern "C" fn todo_delete(id: c_longlong) -> i32 {
    to_json_ptr(mod_todo::delete_handler(id));
    return 1;
}