use std::ffi::{CStr, CString};
use std::os::raw::c_char;

//mod mod_todo;
mod mod_chat;
mod mod_ssr;

#[no_mangle]
pub extern "C" fn ssr_htm_top() -> *mut c_char {
    let resp = mod_ssr::get_htm_top();

    CString::new(resp)
        .unwrap()
        .into_raw()
}
#[no_mangle]
pub extern "C" fn get_htm_about() -> *mut c_char {
    let resp = mod_ssr::get_htm_about();

    CString::new(resp)
        .unwrap()
        .into_raw()
}

#[no_mangle]
pub extern "C" fn get_htm_chat() -> *mut c_char {
    let resp = mod_ssr::get_htm_chat();

    CString::new(resp)
        .unwrap()
        .into_raw()
}

#[no_mangle]
pub extern "C" fn chat_send(input: *const c_char) -> *mut c_char {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let in_text = unsafe {
        CStr::from_ptr(input)
    };
    let input_str = match in_text.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };
    let in_str : String = input_str.to_string();
    let mut out_str = "".to_string();
    runtime.block_on(async {
        let resp = mod_chat::chat_post(input_str.to_string()).await;
        out_str = resp.clone();  
    });    
    CString::new(out_str)
        .unwrap()
        .into_raw()    
}

#[no_mangle]
pub extern "C" fn hello(name: *const c_char) -> *mut c_char {
    let input = unsafe {
        CStr::from_ptr(name)
    };

    let input = input.to_string_lossy();

    let result = format!("Hello, {}!", input);

    CString::new(result)
        .unwrap()
        .into_raw()
}

#[no_mangle]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    a + b
}
