// tinyhttpd - An example of crate tiny_http
//  derived from https://github.com/tiny-http/tiny-http/blob/0.8.1/examples/serve-root.rs

use ascii::AsciiString;
use std::fs;
use std::path::{Path, PathBuf};
use std::env;
use log::info;
use simplelog::{ConfigBuilder, SimpleLogger, LevelFilter};
use time::macros::format_description;

fn get_content_type(path: &Path) -> &'static str {
    let extension = match path.extension() {
        None => return "text/plain",
        Some(e) => e,
    };

    match extension.to_str().unwrap() {
        "gif" => "image/gif",
        "jpg" => "image/jpeg",
        "jpeg" => "image/jpeg",
        "png" => "image/png",
        "woff" => "font/woff",
        "woff2" => "font/woff2",
        "pdf" => "application/pdf",
        "js" => "application/javascript; charset=utf8",
        "ddeb" => "application/vnd.debian.binary-package",
        "deb" => "application/vnd.debian.binary-package",
        "htm" => "text/html; charset=utf8",
        "html" => "text/html; charset=utf8",
        "css" => "text/css; charset=utf8",
        "txt" => "text/plain; charset=utf8",
        _ => "text/plain; charset=utf8",
    }
}

#[cfg(feature = "canon_path")]
fn getpath(url: &String, cwd: &str) -> Option<PathBuf> {
    let str = String::from(url);
    let str = str.strip_prefix("/")?;
    let path = Path::new(str);
    let cpath = fs::canonicalize(path).ok()?;
    let rpath = cpath.strip_prefix(cwd.to_string()).ok()?;
    Some(rpath.to_owned())
}

#[cfg(not(feature = "canon_path"))]
fn getpath(url: &String, _cwd: &str) -> Option<PathBuf> {
    let str = String::from(url);
    let str = str.strip_prefix("/")?;
    Some(Path::new(str).to_owned())
}

fn main() {
    let config = ConfigBuilder::new()
        .set_time_offset_to_local().unwrap()
        .set_time_format_custom(format_description!(
            "[month repr:short] [day padding:space] [hour]:[minute]:[second]"))
        .build();
    SimpleLogger::init(LevelFilter::Info, config).unwrap();

    let curdir = env::current_dir().unwrap();
    let curdir = curdir.to_str().unwrap();
    info!("The current directory is {}", curdir);

    let server = tiny_http::Server::http("0.0.0.0:8000").unwrap();
    let port = server.server_addr().to_ip().unwrap().port();
    info!("Now listening on port {}", port);

    loop {
        let rq = match server.recv() {
            Ok(rq) => rq,
            Err(_) => break,
        };

        info!("{:?}", rq);

        let url = rq.url().to_string();
        let path = if let Some(path) = getpath(&url, curdir) { path } else {
            let rep = tiny_http::Response::new_empty(tiny_http::StatusCode(404));
            let _ = rq.respond(rep);
            continue;
        };
        let file = fs::File::open(&path);

        if file.is_ok() {
            let response = tiny_http::Response::from_file(file.unwrap());

            let response = response.with_header(tiny_http::Header {
                field: "Content-Type".parse().unwrap(),
                value: AsciiString::from_ascii(get_content_type(&path)).unwrap(),
            });

            let _ = rq.respond(response);
        } else {
            let rep = tiny_http::Response::new_empty(tiny_http::StatusCode(404));
            let _ = rq.respond(rep);
        }
    }
}
