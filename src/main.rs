// TODO: caching headers
// TODO: static file compression

use rouille::{
    Response,
    router,
};

fn main() {
    rouille::start_server("0.0.0.0:8080", |request| {
        router!(request,
            (GET) ["/"] =>
                Response::html(include_str!("../resources/form.html")),
            (GET) ["/search"] =>
                Response::html(include_str!("../resources/search.html")),
            (GET) ["/buglist.cgi"] =>
                Response::html(include_str!("../resources/search.html")),
            (GET) ["/static/mnemonic.js"] =>
                Response::from_data("application/javascript", include_str!("../resources/mnemonic.js")),
            (GET) ["/static/style.css"] =>
                Response::from_data("text/css", include_str!("../resources/style.css")),
            (GET) ["/{mnemonic}", mnemonic: String] => {
                let mut buf = [0; 4];
                if let Ok(n) = mnemonic::decode(mnemonic, &mut buf[..]) {
                    buf.rotate_right(4 - n);
                    let id = u32::from_be_bytes(buf);
                    let url = format!("https://bugzilla.mozilla.org/show_bug.cgi?id={}", id);
                    Response::redirect_303(url)
                } else {
                    Response::empty_404()
                }
            },
            _ => Response::empty_404()
        )
    });
}
