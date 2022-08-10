use cfg_if::cfg_if;
use worker::console_log;
use worker::Date;
use worker::Request;

pub const MINIMAL_HEADER: &str = r#"<link rel="preconnect" href="https://fonts.googleapis.com"><link rel="preconnect" href="https://fonts.gstatic.com" crossorigin><link href="https://fonts.googleapis.com/css2?family=Roboto:wght@400;700&family=Roboto+Condensed&display=swap" rel="stylesheet"><link rel="stylesheet" href="https://use.typekit.net/tzs5cir.css"><style>body { background-color: #f5f5f5; font-size: 100%; } h1 { color: #253447; font-family: 'Roboto', sans-serif; font-weight: 700; font-size: 4.25em; letter-spacing: 3px; text-align: center; } p { color: #253447; font-family: 'aileron', sans-serif; font-weight: 400; font-style: normal; font-size: 1.2em; text-align: center; } a { color: #253447; } .hover-link { text-decoration: none; } .hover-link:hover { text-decoration: underline; text-decoration-style: dotted; } .content { width: 60%; margin: auto; padding-top: 5%; margin-bottom: 5%; display: table; }</style>"#;

cfg_if! {
    // https://github.com/rustwasm/console_error_panic_hook#readme
    if #[cfg(feature = "console_error_panic_hook")] {
        extern crate console_error_panic_hook;
        pub use self::console_error_panic_hook::set_once as set_panic_hook;
    } else {
        #[inline]
        pub fn set_panic_hook() {}
    }
}

pub fn create_error_response(title: &str, error: &str, message: &str) -> String {
    format!(
        r#"<!DOCTYPE html><html lang="en"><head><title>{title}</title>{header}</head><body><div class="content"><h1>{error}</h1><p>{message}</p></div></body>"#,
        header = MINIMAL_HEADER
    )
}

pub fn log_request(req: &Request) {
    console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or("unknown region".into())
    );
}

pub fn log_not_present_error(kv: &str, key: &str) {
    console_log!(
        "{} - [{}], key \"{}\" not present in store",
        Date::now().to_string(),
        kv,
        key
    );
}

pub fn log_generic_error(key: &str, err: &str) {
    console_log!(
        "{} - [{}], received generic worker error: {}",
        Date::now().to_string(),
        key,
        err
    )
}
