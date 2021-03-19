use backend_lib::*;
use actix_web::{self, http, test, dev, App, http::Method, web::ServiceConfig};
use actix_http;
use data::user::*;
use actix_session::{CookieSession};
use chrono::Local;

/// Creates a NewUser object by appending the formatted time to a given prefix
/// this ensures that no two test users will share a name
pub fn make_user(user_name_prefix: &str) -> NewUser {
    let user_name = format!("{}-{}", user_name_prefix, Local::now().format("%Y-%m-%d-%H-%M-%S"));
    println!("Using user {}", user_name);
    NewUser {
        uname: user_name,
        password: "pw1".to_string(),
    }
}

/// Makes a new app service with the specified routes
pub async fn make_service<F>(
    config: F,
    pool: &PgPool
) -> impl dev::Service<Request = actix_http::Request, Response = dev::ServiceResponse<dev::Body>, Error = actix_web::Error>
where
    F: FnOnce(&mut ServiceConfig)
{
    // Creates the service pool with both the sign in route to auth with
    // and the route passed as a parameter
    test::init_service(
        App::new()
            .data(pool.clone())
            .wrap(
                CookieSession::signed(&[0; 32]).secure(false)
            )
            .configure(config)
    ).await
}

/// Creates a new user with the given uname,
/// then logs that user in and returns the session cookie
pub async fn login(user: &NewUser, pool: &PgPool) -> Option<http::Cookie<'static>> {
    // Create a new user account by calling the query directly (not through http)
    let conn = pool.get().expect("Could not get connection from pool");
    let _ = query::user::save_new_user(&user, &conn);
    // Then sign in by calling the sign in route
    let mut app = make_service(|c| {c.service(route::user::sign_in);}, &pool).await;
    let req = test::TestRequest::with_header("content-type", "text/plain")
        .uri("/login")
        .method(Method::POST)
        .set_json(&user)
        .to_request();
    let resp = test::call_service(&mut app, req).await;
    // pull the cookies out of the response
    let response = resp.response();
    let cookies = response.cookies();
    // Find the cookie named "actix-session" and return it
    let session_cookie = cookies.into_iter().filter(|cookie| {
        cookie.name() == "actix-session"
    }).last()?;
    Some(session_cookie.into_owned())
}