use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

///
/// 从刷新token获取token
///
pub fn auth(refresh_token: String) -> Result<String, String> {
    let refresh: RefreshToken = post(
        refresh_token.clone(),
        request::REFRESH_TOKEN,
        &[("refreshToken".to_string(), refresh_token.clone())],
    )?;
    Ok(refresh.token)
    // Err("刷新token失败".to_string())
}

///
/// 福利签到详情
///
pub fn get_encourage_signin_show(token: String) -> Result<Encourage, String> {
    post(token, request::ENCOURAGE_SIGNIN_SHOW_URL, &[])
}

///
/// 社区签到详情
///
pub fn get_user_have_signin_new(token: String) -> Result<UserHaveSignin, String> {
    post(token, request::USER_HAVE_SIGN_IN_NEW, &[])
    // Ok(UserHaveSignin {
    //     have_sign_in: false,
    //     gain_list: vec![GainItem {
    //         gain_value: 10,
    //         gain_typ: 1,
    //     }],
    //     total_sign_in_day: 1,
    //     have_role_sign_in: false,
    // })
}

///
/// 角色签到
///
pub fn role_signin(token: String) -> Result<DayAward, String> {
    let encourage = get_encourage_signin_show(token.clone())?;
    let day_award = encourage
        .day_award
        .get(encourage.signin_time as usize)
        .ok_or("未找到奖励")?;

    let role_sign: RoleSignin = post(
        token,
        request::ENCOURAGE_SIGNIN,
        &[
            ("dayAwardId".to_string(), day_award.id.to_string()),
            ("signinType".to_string(), "1".to_string()),
            ("periodId".to_string(), encourage.period.id.to_string()),
        ],
    )?;
    if role_sign.send_day_award {
        return Ok(day_award.clone());
    }
    Err("签到失败，未发放奖励".to_string())
    // Ok(DayAward {
    //     id: 123,
    //     award_name: "牙膏".to_string(),
    //     award_num: 3,
    //     icon_url: "https://herobox-img.yingxiong.com/config/signin/1761104660591167739.png".to_string(),
    // })
}

///
/// 社区签到
///
pub fn user_signin(token: String) -> Result<UserSign, String> {
    post(token, request::USER_SIGNIN, &[])
    // Ok(UserSign {
    //     has_new_product: false,
    //     gain_vo_list: vec![
    //         GainItem {
    //             gain_value: 10,
    //             gain_typ: 1,
    //         },
    //         GainItem {
    //             gain_value: 10,
    //             gain_typ: 2,
    //         },
    //     ],
    //     has_new_draw: false,
    //     continuity_sign_in_day: 4,
    //     total_sign_in_day: 5,
    // })
}

mod request {
    use reqwest::header::HeaderValue;

    pub const ENCOURAGE_SIGNIN_SHOW_URL: &str =
        "https://dnabbs-api.yingxiong.com/encourage/signin/show";

    pub const USER_HAVE_SIGN_IN_NEW: &str = "https://dnabbs-api.yingxiong.com/user/haveSignInNew";
    pub const USER_SIGNIN: &str = "https://dnabbs-api.yingxiong.com/user/signIn";
    pub const ENCOURAGE_SIGNIN: &str = "https://dnabbs-api.yingxiong.com/encourage/signin/signin";
    pub const REFRESH_TOKEN: &str = "https://dnabbs-api.yingxiong.com/user/refreshToken";

    pub fn get_header(token: String) -> reqwest::header::HeaderMap {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "Content-Type",
            HeaderValue::from_static("application/x-www-form-urlencoded"),
        );
        headers.insert("source", HeaderValue::from_static("android"));
        headers.insert("token", HeaderValue::from_str(&token).unwrap());
        headers
    }
}
///
/// Post请求
///
pub fn post<T: for<'de> Deserialize<'de>>(
    token: String,
    url: &str,
    param: &[(String, String)],
) -> Result<T, String> {
    let mut request = Client::new().post(url);
    if !param.is_empty() {
        request = request.form(param);
    }

    let res = request
        .headers(request::get_header(token))
        .send()
        .and_then(|r| r.json::<Value>())
        .map_err(|e| e.to_string())?;
    res.get("code")
        .and_then(|c| c.as_i64())
        .filter(|&code| code == 200)
        .ok_or_else(|| format!("请求失败{}", res))?;

    Ok(
        serde_json::from_value(res.get("data").ok_or(format!("data不存在{}", res))?.clone())
            .map_err(|e| format!("格式化失败: {}", e))?,
    )
}


#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RefreshToken {
    pub token: String
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoleSignin {
    pub signin_time_now: String,
    pub send_day_award: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserSign {
    pub has_new_product: bool,
    pub gain_vo_list: Vec<GainItem>,
    pub has_new_draw: bool,
    pub continuity_sign_in_day: u8,
    pub total_sign_in_day: u8,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserHaveSignin {
    pub have_sign_in: bool,
    pub gain_list: Vec<GainItem>,
    pub total_sign_in_day: u64,
    pub have_role_sign_in: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GainItem {
    pub gain_value: u64,
    pub gain_typ: u8,
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Encourage {
    pub today_signin: bool,
    pub signin_time: u32,
    pub period: Period,
    pub role_info: GameRoleInfo,
    pub day_award: Vec<DayAward>,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DayAward {
    pub id: u64,
    pub award_name: String,
    pub award_num: u32,
    pub icon_url: String,
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Period {
    pub start_date: u64,
    pub end_date: u64,
    pub name: String,
    pub id: u64,
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GameRoleInfo {
    pub role_id: String,
    pub role_name: String,
    pub head_url: String,
    pub level: u32,
}
