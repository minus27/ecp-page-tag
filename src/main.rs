//! Default Compute@Edge template program.

use fastly::http::{header, Method, StatusCode};
use fastly::{mime, Error, Request, Response};

use config::{Config, FileFormat};

use lol_html::html_content::ContentType;
use lol_html::{element, rewrite_str, RewriteStrSettings};

const BACKEND_NAME: &str = "info.demotool.site";

#[fastly::main]
fn main(mut req: Request) -> Result<Response, Error> {

    match req.get_method() {
        &Method::GET | &Method::HEAD => (),

        _ => {
            return Ok(Response::from_status(StatusCode::METHOD_NOT_ALLOWED)
                .with_header(header::ALLOW, "GET, HEAD")
                .with_body_str("This method is not allowed\n"))
        }
    };

    match req.get_path() {
        "/" => Ok(Response::from_status(StatusCode::OK)
            .with_content_type(mime::TEXT_HTML_UTF_8)
            .with_body("<iframe src='https://developer.fastly.com/compute-welcome' style='border:0; position: absolute; top: 0; left: 0; width: 100%; height: 100%'></iframe>\n")),

        "/invert_colors_script.js" => Ok(Response::from_status(StatusCode::OK)
            .with_content_type(mime::APPLICATION_JAVASCRIPT_UTF_8)
            .with_body("document.querySelector('body').setAttribute('style','color:white; background-color:black;')\n")),

        _ => {
            req.set_pass(true);
            let mut resp = req.send(BACKEND_NAME)?;

            let new_resp_body = modify_content(
                Some(resp).unwrap().into_body_str(),
                "<script src=\"invert_colors_script.js\" defer></script>",
            );

            Ok(Response::from_body(new_resp_body)
                .with_header("X-Toml-Version", &format!("{}", get_version()))
                .with_status(StatusCode::OK))
        }
    }

}

/// This function reads the fastly.toml file and gets the deployed version. This is only run at
/// compile time. Since we bump the version number after building (during the deploy) we return
/// the version incremented by one so the version returned will match the deployed version.
fn get_version() -> i32 {
    Config::new()
        .merge(config::File::from_str(
            include_str!("../fastly.toml"), // assumes the existence of fastly.toml
            FileFormat::Toml,
        ))
        .unwrap()
        .get_str("version")
        .unwrap()
        .parse::<i32>()
        .unwrap_or(0)
        + 1
}

// This function appends the script tag to the end of the head section.
fn modify_content(body_str: String, script_tag: &str) -> String {
    let html = rewrite_str(
        &body_str,
        RewriteStrSettings {
            element_content_handlers: vec![
                element!("head", |el| {
                    el.append(script_tag, ContentType::Html);
                    Ok(())
                }),
            ],
            ..RewriteStrSettings::default()
        },
    )
    .unwrap();
    html
}