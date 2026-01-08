use jjj_core::{util::*, *};
use std::sync::{Arc, Mutex};
use tauri::AppHandle;
use tauri::Emitter;
// 全局 AppHandle 存储，用于在回调函数中发送事件
static APP_HANDLE: Mutex<Option<Arc<AppHandle>>> = Mutex::new(None);

// 回调函数，将 CallbakEvent 转换为日志并发送到前端
fn handle_callback_event(event: CallbakEvent) {
    let log_msg = match event {
        CallbakEvent::SigninOver => "今天已经签到过啦".to_string(),
        CallbakEvent::Fail(msg) => format!("出错啦: {}", msg),
        CallbakEvent::RoleSigninOk(day_award) => {
            format!(
                "角色签到成功: 获得{}x{}",
                day_award.award_name, day_award.award_num
            )
        }
        CallbakEvent::RoleSigninFail(msg) => format!("角色签到失败: {}", msg),
        CallbakEvent::UserSigninOk(user_sign) => {
            format!("社区签到成功: {:?}", user_sign)
        }
        CallbakEvent::UserSigninFail(msg) => format!("社区签到失败: {}", msg),
        CallbakEvent::Log(msg) => msg,
    };

    // 从全局存储中获取 AppHandle 并发送事件
    if let Ok(handle_guard) = APP_HANDLE.lock() {
        if let Some(app) = handle_guard.as_ref() {
            let _ = app.emit("log", log_msg);
        }
    }
}


#[tauri::command]
fn start_signin(app: AppHandle, token: String) -> Result<(), String> {
    // 验证 token 不为空
    if token.is_empty() {
        return Err("Token 不能为空".to_string());
    }

    // 将 AppHandle 存储到全局变量中
    {
        let mut handle_guard = APP_HANDLE.lock().unwrap();
        *handle_guard = Some(Arc::new(app.clone()));
    }

    // 在单独线程中运行签到循环
    std::thread::spawn(move || {
        // 创建账户
        let mut acc = match Account::new(token) {
            Ok(acc) => acc,
            Err(e) => {
                let _ = app.emit("log", format!("启动失败: {}", e));
                return;
            }
        };

        // 发送启动成功日志
        let _ = app.emit("log", "程序启动成功");

        // 运行阻塞循环
        acc.run_loop(handle_callback_event);
    });

    Ok(())
}

#[tauri::command]
fn exit_app(app: AppHandle) {
    app.exit(0);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![start_signin, exit_app])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
