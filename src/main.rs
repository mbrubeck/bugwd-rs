// TODO: caching headers
// TODO: static file compression

use rouille::{
    Response,
    router,
};

fn get_server_port() -> u16 {
    std::env::var("PORT").ok().and_then(|p| p.parse().ok()).unwrap_or(8080)
}

macro_rules! static_file {
    ($mime: expr, $path: expr) => {
        Response::from_data($mime, include_str!(concat!("../resources/", $path)))
            .with_public_cache(600)
    }
}

fn main() {
    rouille::start_server(("0.0.0.0", get_server_port()), |request| {
        router!(request,
            (GET) ["/"] => static_file!("text/html", "form.html"),
            (GET) ["/search"] => static_file!("text/html", "search.html"),
            (GET) ["/buglist.cgi"] => static_file!("text/html", "search.html"),
            (GET) ["/static/mnemonic.js"] => static_file!("application/javascript", "mnemonic.js"),
            (GET) ["/static/style.css"] => static_file!("text/css", "style.css"),
            (GET) ["/{mnemonic}", mnemonic: String] => {
                let mut buf = [0; 4];
                if let Ok(n) = mnemonic::decode(mnemonic, &mut buf[..]) {
                    buf.rotate_right(4 - n);
                    let id = u32::from_be_bytes(buf);
                    let url = format!("https://bugzilla.mozilla.org/show_bug.cgi?id={}", id);
                    Response::redirect_301(url).with_public_cache(365 * 24 * 60 * 60)
                } else {
                    Response::empty_404()
                }
            },
            _ => Response::empty_404()
        )
    });
}
