use askama::Template;
use axum::{extract::{State, Path}, response::{IntoResponse, Redirect}, Form, Router, routing::get};
use crate::{config::template::HtmlResponse, ApiState};
use super::{models::{NewContact, Contact}, contact_repository::ContactRepository};

pub fn get_routes() -> Router<ApiState> {
    Router::new()
        .route("/", get(index_get).post(index_post))
        .route("/:id/view", get(get_contact))
        .route("/:id/update", get(update_get).post(update_put))
        .route("/:id/delete", get(delete_contact))
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    contacts: Vec<Contact>,
}

#[derive(Template)]
#[template(path = "contact.html")]
struct ContactTemplate {
    contact: Contact,
}

#[derive(Template)]
#[template(path = "update-contact.html")]
struct UpdateContactTemplate {
    contact: Contact,
}

#[derive(Template)]
#[template(path = "error.html")]
struct ErrorTemplate {
    message: String
}

async fn index_get(State(app_state): State<ApiState>) -> impl IntoResponse {
    match ContactRepository::new(&app_state.pool).get_all().await {
        Ok(contacts) => HtmlResponse(IndexTemplate { contacts }).into_response(),
        Err(err) => HtmlResponse(ErrorTemplate { message: err.to_string() }).into_response()
    }
}

async fn index_post(State(app_state): State<ApiState>, Form(data): Form<NewContact>) -> impl IntoResponse {
    match ContactRepository::new(&app_state.pool).create(data).await {
        Ok(_) => Redirect::to("/contacts").into_response(),
        Err(err) => HtmlResponse(ErrorTemplate { message: err.to_string() }).into_response()
    }
}

async fn get_contact(State(app_state): State<ApiState>, Path(id): Path<i32>) -> impl IntoResponse {
    match ContactRepository::new(&app_state.pool).get_by_id(id).await {
        Ok(contact) => HtmlResponse(ContactTemplate { contact }).into_response(),
        Err(err) => HtmlResponse(ErrorTemplate { message: err.to_string() }).into_response()
    }
}

async fn update_get(State(app_state): State<ApiState>, Path(id): Path<i32>) -> impl IntoResponse {
    match ContactRepository::new(&app_state.pool).get_by_id(id).await {
        Ok(contact) => HtmlResponse(UpdateContactTemplate { contact }).into_response(),
        Err(err) => HtmlResponse(ErrorTemplate { message: err.to_string() }).into_response()
    }
}

async fn update_put(State(app_state): State<ApiState>, Path(id): Path<i32>, Form(data): Form<NewContact>) -> impl IntoResponse {
    match ContactRepository::new(&app_state.pool).update(id, data).await {
        Ok(_) => Redirect::to(format!("/contacts/{}/view", id).as_str()).into_response(),
        Err(err) => HtmlResponse(ErrorTemplate { message: err.to_string() }).into_response()
    }
}

async fn delete_contact(State(app_state): State<ApiState>, Path(id): Path<i32>) -> impl IntoResponse {
    match ContactRepository::new(&app_state.pool).delete(id).await {
        Ok(_) => Redirect::to("/contacts").into_response(),
        Err(err) => HtmlResponse(ErrorTemplate { message: err.to_string() }).into_response()
    }
}
