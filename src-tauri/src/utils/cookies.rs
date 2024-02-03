use cookie::{Cookie, CookieJar};
use reqwest::Response;

pub fn get_response_cookie_by_name(response: &Response, cookie_name: &str) -> String {
    let cookie_str = response
        .headers()
        .get("set-cookie")
        .unwrap()
        .to_str()
        .unwrap();

    let mut jar = CookieJar::new();

    for cookie in cookie_str.split(';') {
        if let Ok(cookie) = Cookie::parse(cookie.trim()) {
            jar.add_original(cookie.into_owned());
        }
    }

    let auth_cookie = jar
        .get(cookie_name)
        .expect(format!("{} cookie not found in jar", cookie_name).as_str());

    auth_cookie.value().to_string()
}

// Helper function to convert cookies in CookieJar to a string
pub fn format_cookies(cookie_jar: &CookieJar) -> String {
    cookie_jar
        .iter()
        .map(|cookie| format!("{}={}", cookie.name(), cookie.value()))
        .collect::<Vec<_>>()
        .join("; ")
}
