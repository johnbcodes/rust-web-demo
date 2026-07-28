include!(concat!(env!("OUT_DIR"), "/vite_assets.rs"));

markup::define! {
    Layout<Head: markup::Render, Body: markup::Render>(head: Head, body: Body) {
        @markup::doctype()
        html[lang = "en"] {
            head {
                meta[charset = "utf-8"] {}
                @head
                @for stylesheet in STYLESHEETS {
                    link[rel = "stylesheet", href = {format!("/dist/{}", stylesheet)}] {}
                }
                link[rel = "icon", href = {format!("/dist/{}", FAVICON)}] {}
                @for preload in MODULE_PRELOADS {
                    link[rel = "modulepreload", href = {format!("/dist/{}", preload)}] {}
                }
                script["type" = "module", src = {format!("/dist/{}", SCRIPT)}] {}
            }
            body {
                @body
            }
        }
    }
}
