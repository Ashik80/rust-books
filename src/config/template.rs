use askama::Template;
use axum::{response::{IntoResponse, Html}, http::StatusCode};

pub struct HtmlResponse<T>(pub T);

impl<T> IntoResponse for HtmlResponse<T> where T: Template {
    fn into_response(self) -> axum::response::Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Error: {}", err)).into_response()
        }
    }
}
