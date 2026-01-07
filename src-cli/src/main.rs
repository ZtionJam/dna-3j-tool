use jjj_core::{util::*, *};

fn main() {
    let mut acc = Account::new("your_refresh_token_here".to_string()).exit_expect("启动失败");
    log("程序启动成功");
    acc.run_loop(|e| match e {
        CallbakEvent::SigninOver => {
            log("今日已签到");
        }
        CallbakEvent::TokenRefreshFail(msg) => {
            log(&format!("刷新Token失败: {}", msg));
        }
        CallbakEvent::RoleSigninOk(day_award) => {
            log(&format!("角色签到成功: {:?}", day_award));
        }
        CallbakEvent::RoleSigninFail(msg) => {
            log(&format!("角色签到失败: {}", msg));
        }
        CallbakEvent::UserSigninOk(msg) => {
            log(&format!("社区签到成功: {}", msg));
        }
        CallbakEvent::UserSigninFail(msg) => {
            log(&format!("社区签到失败: {}", msg));
        }
    });
}
