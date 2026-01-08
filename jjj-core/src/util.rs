use std::{fmt::Display, process};

use chrono::Local;

use crate::api::DayAward;
use crate::api::UserSign;
pub fn log(msg: &str) {
    println!("[{}] -> {}", Local::now().format("%Y-%m-%d %H:%M:%S"), msg);
}

#[derive(Debug)]
pub enum CallbakEvent {
    /// 签到失败，已经签到
    SigninOver,
    /// 角色签到成功
    RoleSigninOk(DayAward),
    /// 角色签到失败
    RoleSigninFail(String),
    /// 社区签到成功
    UserSigninOk(UserSign),
    /// 社区签到失败
    UserSigninFail(String),
    /// 刷新Token失败
    Fail(String),
    //输出下日志
    Log(String),
}
pub trait ExitExpect<T, E: Display> {
    fn exit_expect(self, msg: &str) -> T;
}

impl<T, E: Display> ExitExpect<T, E> for Result<T, E> {
    fn exit_expect(self, msg: &str) -> T {
        match self {
            Ok(val) => val,
            Err(e) => {
                eprintln!("{}: {}", msg, e);
                process::exit(1);
            }
        }
    }
}
