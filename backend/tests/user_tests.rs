mod setup;

use backend_lib::*;
use actix_web::{self, test, http::Method};
use actix_session::{CookieSession};
use setup::*;

#[actix_rt::test]
async fn sign_up_user() {
    let pool = get_connection_pool();
    let user = make_user("sign_up_user");
    println!("u: {:#?}", user);
    let mut app = make_service(|c| {c.service(route::user::sign_up);}, &pool).await;
    let req = test::TestRequest::with_header("content-type", "text/plain")
        .uri("/user")
        .method(Method::POST)
        .set_json(&user)
        .to_request();
    let resp = test::call_service(&mut app, req).await;
    println!("{:#?}", resp);
    assert!(resp.status().is_success());
}

// TODO: test login fails for uname and pw