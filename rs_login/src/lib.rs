use std::ffi::{CStr, CString};
use std::os::raw::c_char;

mod mod_ssr;
mod mod_user;

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
pub extern "C" fn get_htm_login() -> *mut c_char {
    let resp = mod_ssr::get_htm_login();

    CString::new(resp)
        .unwrap()
        .into_raw()
}

#[no_mangle]
pub extern "C" fn user_login(input: *const c_char) -> i32 {
    let mut ret: i32 = -1;
    let in_text = unsafe {
        CStr::from_ptr(input)
    };
    let input_str = match in_text.to_str() {
        Ok(s) => s,
        Err(_) => return ret,
    };
    let result = mod_user::login(&input_str);
    let mut ret_num :i32 = 0;
    match result {
        Ok(value) => {
            println!("result: {}", value);
            ret_num = value;
        },
        Err(err) => println!("エラー: {}", err),
    }    
    println!("ret_num: {}", ret_num);
    ret = ret_num;
    return ret;
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
