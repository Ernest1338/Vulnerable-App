use crate::{AppState, error::AppError};
use axum::{
    extract::{Query, State},
    response::{Html, IntoResponse, Redirect},
};
use minijinja::context;
use std::{collections::HashMap, sync::Arc};

pub async fn index(State(state): State<Arc<AppState>>) -> Result<impl IntoResponse, AppError> {
    let index_template = state.templates.get_template("index.html")?; // .context("Failed to load index.html from disk")?; // .context is optional!
    let html = index_template.render(context!())?;
    Ok(Html(html))
}

pub async fn open_redirect(
    State(state): State<Arc<AppState>>,
    params: Query<HashMap<String, String>>,
) -> Result<impl IntoResponse, AppError> {
    if let Some(url) = &params.get("url") {
        return Ok(Redirect::to(url).into_response());
    }

    let tmpl = state.templates.get_template("open-redirect.html")?;
    let html = tmpl.render(context!())?;
    Ok(Html(html).into_response())
}
