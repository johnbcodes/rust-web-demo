use axum::{
    body::{boxed, Full},
    http::{header, StatusCode, Uri},
    response::{IntoResponse, Response},
};
use rust_embed::RustEmbed;
use tracing::info;

#[derive(RustEmbed)]
#[folder = "$CARGO_MANIFEST_DIR/ui/target/public/"]
pub(crate) struct Assets;

pub(crate) struct StaticFile<T>(pub(crate) T);

#[cfg(debug_assertions)]
const MAX_AGE: &str = "max-age=120";
#[cfg(not(debug_assertions))]
const MAX_AGE: &str = "max-age=31536000";

impl<T> IntoResponse for StaticFile<T>
where
    T: Into<String>,
{
    fn into_response(self) -> Response {
        let path = self.0.into();

        match Assets::get(path.as_str()) {
            Some(content) => {
                info!("Retrieving asset with path: {path}");
                let body = boxed(Full::from(content.data));
                let mime = mime_guess::from_path(path).first_or_octet_stream();
                Response::builder()
                    .header(header::CONTENT_TYPE, mime.as_ref())
                    .header(header::CACHE_CONTROL, MAX_AGE)
                    .body(body)
                    .unwrap()
            }
            None => Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(boxed(Full::from("Not Found")))
                .unwrap(),
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
