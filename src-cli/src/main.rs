use jjj_core::{util::*, *};

fn main() {
    let mut acc = Account::new("your_refresh_token_here".to_string()).exit_expect("启动失败");
    log("程序启动成功");
    acc.run_loop(|e| match e {
        CallbakEvent::SigninOver => {
            log("今天已经签到过啦");
        }
        CallbakEvent::Fail(msg) => {
            log(&format!("出错啦: {}", msg));
        }
        CallbakEvent::RoleSigninOk(day_award) => {
            log(&format!(
                "角色签到成功: 获得{}x{}",
                day_award.award_name, day_award.award_num
            ));
        }
        CallbakEvent::RoleSigninFail(msg) => {
            log(&format!("角色签到失败: {}", msg));
        }
        CallbakEvent::UserSigninOk(user_sign) => {
            log(&format!("社区签到成功: {:?}", user_sign));
        }
        CallbakEvent::UserSigninFail(msg) => {
            log(&format!("社区签到失败: {}", msg));
        }
    });
}
