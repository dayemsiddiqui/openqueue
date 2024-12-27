use askama::Template;
use askama_axum::IntoResponse;

// Control Panel Template
#[derive(Template)]
#[template(path = "index.html")]
pub struct ControlPanelTemplate {
    title: String,
    content: String,
}   

pub async fn control_panel() -> impl IntoResponse {
    ControlPanelTemplate {
        title: "OpenQueue Control Panel".to_string(),
        content: "Basic Control Panel".to_string(),
    }
}