const STYLESHEET: &str = env!("STYLESHEET");
const FAVICON: &str = env!("FAVICON");
const SCRIPT: &str = env!("SCRIPT");

markup::define! {
    Layout<Head: markup::Render, Body: markup::Render>(head: Head, body: Body) {
        @markup::doctype()
        html[lang = "en"] {
            head {
                meta[charset = "utf-8"] {}
                @head
                link[rel = "stylesheet", href = {format!("/dist/{}", STYLESHEET)}] {}
                link[rel = "icon", href = {format!("/dist/{}", FAVICON)}] {}
                script["type" = "module", src = {format!("/dist/{}", SCRIPT)}] {}
            }
            body {
                @body
            }
        }
    }
}
