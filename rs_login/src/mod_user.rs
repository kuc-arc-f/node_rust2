use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs;
use std::io::{self, Write};
use std::env;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserReq {
    pub email: String,
    pub password: String,
}

pub fn login(body: &str) -> std::result::Result<i32, Box<dyn std::error::Error>> 
{
    let mut ret : i32 = -1;
    dotenv().ok();
    println!("body={}", &body);

    let user_mail = env::var("USER_MAIL")
        .expect("error, USER_MAIL none.");
    let user_pass = env::var("USER_PASS")
        .expect("error , USER_PASS none");

    println!("env.email: {}", user_mail);
    println!("env.user_pass: {}", user_pass);

    let user_data: UserReq = serde_json::from_str(body)?;
    println!("user_data: {:?}", user_data);
    println!("email: {}", user_data.email);
    println!("password: {}", user_data.password);
    if user_mail != user_data.email {
        println!("error, user_mail NG");
        return Ok(ret);
    }
    if user_pass != user_data.password {
        println!("error, user_pass NG");
        return Ok(ret);
    }    
    ret = 1;
    return Ok(ret);
}
