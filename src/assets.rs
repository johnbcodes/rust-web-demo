use rocket::{
    fairing::AdHoc,
    http::{ContentType, Header},
    response::Responder,
};
use rust_embed::RustEmbed;
use std::borrow::Cow;
use std::ffi::OsStr;
use std::path::PathBuf;

#[derive(RustEmbed)]
#[folder = "$CARGO_MANIFEST_DIR/ui/target/public/"]
pub(crate) struct Asset;

#[derive(Responder)]
#[response(status = 200)]
struct AssetResponse {
    content: Cow<'static, [u8]>,
    content_type: ContentType,
    max_age: Header<'static>,
}

#[cfg(debug_assertions)]
const MAX_AGE: &str = "max-age=120";
#[cfg(not(debug_assertions))]
const MAX_AGE: &str = "max-age=31536000";

pub(crate) fn stage() -> AdHoc {
    AdHoc::on_ignite("Assets Stage", |rocket| async {
        rocket.mount("/dist", routes![asset_handler])
    })
}

#[get("/<file..>")]
fn asset_handler(file: PathBuf) -> Option<AssetResponse> {
    let filename = file.display().to_string();
    let asset = Asset::get(&filename)?;
    let content_type = file
        .extension()
        .and_then(OsStr::to_str)
        .and_then(ContentType::from_extension)
        .unwrap_or(ContentType::Bytes);
    Some(AssetResponse {
        content: asset.data,
        content_type,
        max_age: Header::new("Cache-Control", MAX_AGE),
    })
}
