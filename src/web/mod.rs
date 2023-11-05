pub mod api;
mod error;
pub mod middleware;
pub mod routes_login;

pub use self::error::{Error, Result};
use crate::crypt::token::Token;
use tower_cookies::{Cookie, Cookies};

// endregion: --- Modules

pub const AUTH_TOKEN: &str = "auth-token";

fn set_token_cookie(cookies: &Cookies, token: Token) {
    let mut cookie = Cookie::new(AUTH_TOKEN, token.to_string());
    cookie.set_http_only(true);
    cookie.set_path("/");

    cookies.add(cookie);
}

fn remove_token_cookie(cookies: &Cookies) {
    let mut cookie = Cookie::named(AUTH_TOKEN);
    cookie.set_path("/");

    cookies.remove(cookie);
}