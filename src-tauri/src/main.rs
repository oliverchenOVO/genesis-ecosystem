#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use sim_app::{Action, App};
use tauri::Manager;

#[tauri::command]
async fn action(app: tauri::State<'_, App>, action: Action) -> Result<serde_json::Value, String> {
    let app = app.inner().clone();
    tauri::async_runtime::spawn_blocking(move || app.execute(action))
        .await
        .map_err(|e| e.to_string())?
}
fn main() {
    tracing_subscriber::fmt().with_env_filter("info").init();
    let result = tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .on_page_load(|window, payload| {
            if payload.event() == tauri::webview::PageLoadEvent::Finished {
                tracing::info!(url=%payload.url(), "webview_page_loaded");
                // Optional local distribution smoke evidence: real page load and worker response.
                if let Some(path) = std::env::var_os("GENESIS_SMOKE_REPORT") {
                    let snapshot = window.state::<App>().execute(Action::Snapshot);
                    let report = serde_json::json!({"url":payload.url().as_str(),"page_loaded":true,"snapshot":snapshot});
                    if let Err(error) = std::fs::write(path, report.to_string()) {
                        tracing::error!(%error,"smoke_report_failed");
                    }
                }
            }
        })
        .setup(|app| {
            let directory = app.path().app_data_dir()?.join("worlds");
            let worker = App::start(directory).map_err(std::io::Error::other)?;
            tracing::info!("application_startup");
            app.manage(worker);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![action])
        .run(tauri::generate_context!());
    if let Err(error) = result {
        tracing::error!(%error,"application_failed");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tauri::test::{get_ipc_response, mock_builder, mock_context, noop_assets, INVOKE_KEY};

    #[test]
    fn close_listener_can_finish_the_native_window_close() {
        // Tauri's onCloseRequested wrapper invokes destroy after our guard allows close.
        let capability: serde_json::Value =
            serde_json::from_str(include_str!("../capabilities/default.json")).unwrap();
        let permissions = capability["permissions"].as_array().unwrap();
        assert!(permissions.iter().any(|p| p == "core:window:allow-close"));
        assert!(permissions.iter().any(|p| p == "core:window:allow-destroy"));
        assert_eq!(capability["windows"], serde_json::json!(["main"]));
    }

    #[test]
    fn ipc_routes_real_worker_and_rejects_invalid_commands() {
        let directory = std::env::temp_dir().join(format!("genesis-ipc-{}", std::process::id()));
        let app = mock_builder()
            .manage(App::start(directory).unwrap())
            .invoke_handler(tauri::generate_handler![action])
            .build(mock_context(noop_assets()))
            .unwrap();
        let window = tauri::WebviewWindowBuilder::new(&app, "main", Default::default())
            .build()
            .unwrap();
        let invoke = |body: serde_json::Value| {
            get_ipc_response(
                &window,
                tauri::webview::InvokeRequest {
                    cmd: "action".into(),
                    callback: tauri::ipc::CallbackFn(0),
                    error: tauri::ipc::CallbackFn(1),
                    url: "http://tauri.localhost".parse().unwrap(),
                    body: tauri::ipc::InvokeBody::Json(body),
                    headers: Default::default(),
                    invoke_key: INVOKE_KEY.into(),
                },
            )
            .map(|body| body.deserialize::<serde_json::Value>().unwrap())
        };
        let snapshot = invoke(serde_json::json!({"action":{"op":"snapshot"}})).unwrap();
        assert_eq!(snapshot["seed"], "42");
        assert_eq!(snapshot["population"], 50);
        assert_eq!(snapshot["simulation_version"], sim_app::SIMULATION_VERSION);
        let changed = invoke(
            serde_json::json!({"action":{"op":"environment","temperature":1800,"regeneration":9}}),
        )
        .unwrap();
        assert_eq!(changed["temperature"], 1800);
        let invalid =
            invoke(serde_json::json!({"action":{"op":"control","running":true,"speed":3}}));
        assert!(invalid.is_err());
        let verified = invoke(serde_json::json!({"action":{"op":"replay"}})).unwrap();
        assert_eq!(verified["verified"], true);
    }
}
