use clap::Parser;
use jjj_core::{util::*, *};

///皎皎角签到工具
/// Usage: jjj-cli.exe --refresh-token <REFRESH_TOKEN>
///
/// Options:
///  -t, --refresh-token <REFRESH_TOKEN>  皎皎角的refresh token
fn main() {
    let cli = Cli::parse();

    let mut acc = Account::new(cli.refresh_token).exit_expect("启动失败");
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
        CallbakEvent::Log(msg) => {
            log(&msg);
        }
    });
}

#[derive(Parser, Debug)]
#[command(author="ZtionJam", version="0.1.0", about = "皎皎角签到工具", long_about = None)]
struct Cli {
    #[arg(
        short = 't',
        long = "refresh-token",
        required = true,
        help = "皎皎角的refresh token"
    )]
    refresh_token: String,
}
