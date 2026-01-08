use chrono::Local;
use chrono::NaiveTime;
use chrono::TimeZone;

use crate::api::*;
use crate::util::*;
pub mod api;
pub mod util;

pub struct Account {
    token: String,
    refresh_token: String,
}

impl Account {
    pub fn new(refresh_token: String) -> Result<Account, String> {
        let token = api::auth(refresh_token.clone())?;
        Ok(Account {
            token,
            refresh_token,
        })
    }

    pub fn get_game_role_info(&self) -> Result<api::GameRoleInfo, String> {
        Ok(get_encourage_signin_show(self.token.clone())?.role_info)
    }
    pub fn auth_and_signin(&mut self, callbak: &fn(CallbakEvent)) {
        match api::auth(self.refresh_token.clone()) {
            Ok(new_token) => {
                self.token = new_token;
                match get_user_have_signin_new(self.token.clone()) {
                    Ok(info) => {
                        if info.have_sign_in && info.have_role_sign_in {
                            callbak(CallbakEvent::SigninOver);
                        }
                        if !info.have_role_sign_in {
                            match api::role_signin(self.token.clone()) {
                                Ok(da) => {
                                    callbak(CallbakEvent::RoleSigninOk(da));
                                }
                                Err(e) => {
                                    callbak(CallbakEvent::RoleSigninFail(e));
                                }
                            }
                        } else {
                            callbak(CallbakEvent::RoleSigninFail("角色今日已签到".to_string()));
                        }
                        if !info.have_sign_in {
                            match api::user_signin(self.token.clone()) {
                                Ok(m) => {
                                    callbak(CallbakEvent::UserSigninOk(m));
                                }
                                Err(e) => {
                                    callbak(CallbakEvent::UserSigninFail(e));
                                }
                            }
                        } else {
                            callbak(CallbakEvent::UserSigninFail("社区今日已签到".to_string()));
                        }
                    }
                    Err(e) => {
                        callbak(CallbakEvent::Fail(format!("查询签到失败: {}", e)));
                    }
                }
            }
            Err(e) => {
                callbak(CallbakEvent::Fail(format!("刷新Token失败: {}", e)));
            }
        }
    }

    pub fn run_loop(&mut self, callbak: fn(CallbakEvent)) {
        self.auth_and_signin(&callbak);
        loop {
            let now = Local::now();
            let tomorrow_date = now.date_naive() + chrono::Duration::days(1);
            let exec_time = NaiveTime::from_hms_opt(2, 15, 0).expect("无效的时分秒");
            let tomorrow = Local
                .from_local_datetime(&tomorrow_date.and_time(exec_time))
                .unwrap();

            let duration = if tomorrow > now {
                tomorrow - now
            } else {
                tomorrow + chrono::Duration::days(1) - now
            };
            let sleep_duration = std::time::Duration::from_secs(duration.num_seconds() as u64);
            callbak(CallbakEvent::Log(format!(
                "下次签到时间[{}]，还有[{}]秒",
                tomorrow,
                sleep_duration.as_secs()
            )));
            std::thread::sleep(sleep_duration);
            self.auth_and_signin(&callbak);
        }
    }
}
