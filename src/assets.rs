use axum::{
    body::{boxed, Full},
    http::{header, StatusCode, Uri},
    response::{IntoResponse, Response},
};
use mime_guess;
use rust_embed::RustEmbed;
use tracing::info;

#[derive(RustEmbed)]
#[folder = "public/"]
pub(crate) struct Asset;
pub(crate) struct StaticFile<T>(pub(crate) T);

impl<T> IntoResponse for StaticFile<T>
    where
        T: Into<String>,
{
    fn into_response(self) -> Response {
        let path = self.0.into();
        match Asset::get(path.as_str()) {
            Some(content) => {
                info!("Retrieving asset with path: {path}");
                let body = boxed(Full::from(content.data));
                let mime = mime_guess::from_path(path).first_or_octet_stream();
                Response::builder().header(header::CONTENT_TYPE, mime.as_ref()).body(body).unwrap()
            }
            None => {
                Response::builder().status(StatusCode::NOT_FOUND).body(boxed(Full::from("404"))).unwrap()
            },
        }
    }
}

pub(crate) async fn asset_handler(uri: Uri) -> impl IntoResponse {
    let mut path = uri.path().trim_start_matches('/').to_string();
    if path.starts_with("dist/") {
        path = path.replace("dist/", "");
    }
    StaticFile(path)
}