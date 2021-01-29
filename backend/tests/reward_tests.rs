mod setup;

use backend_lib::*;
use actix_web::{self, test, http::Method};
use data::{user::User, reward::*, icon::RewardIcon};
use setup::*;

#[actix_rt::test]
pub async fn get_rewards() {
    // create a new user and log in
    let user = make_user("get_rewards");
    let pool = get_connection_pool();
    let session_cookie = login(&user, &pool).await.expect("Failed to login");
    let mut app = make_service(|c| {c.service(route::reward::get_all);}, &pool).await;
    let req = test::TestRequest::with_header("content-type", "text/plain")
        .uri("/reward")
        .method(Method::GET)
        .cookie(session_cookie)
        .to_request();
    let resp = test::call_service(&mut app, req).await;
    println!("{:#?}", resp);
    assert!(resp.status().is_success());
}

#[actix_rt::test]
pub async fn add_reward() {
    let user = make_user("add_reward");
    let pool = get_connection_pool();
    let session_cookie = login(&user, &pool).await.expect("Failed to login");
    let mut app = make_service(|c| {c.service(route::reward::new);}, &pool).await;
    let reward_name = "RewardName".to_string();
    let new_reward = NewReward {
        name: reward_name.clone(),
        description: "".to_string(),
        bspts: 1,
        icon: RewardIcon::default(),
    };
    let req = test::TestRequest::with_header("content-type", "text/plain")
        .uri("/reward")
        .method(Method::POST)
        .cookie(session_cookie)
        .set_json(&new_reward)
        .to_request();
    let resp = test::call_service(&mut app, req).await;
    println!("{:#?}", resp);
    // First ensure that the call returned OK
    assert!(resp.status().is_success());
    // Then deserialize the body and check that the returned name is the same name as was added
    let body: Reward = test::read_body_json(resp).await;
    assert_eq!(body.name, reward_name);
}

#[actix_rt::test]
async fn update_reward() {
    let user = make_user("update_reward");
    let pool = get_connection_pool();
    let session_cookie = login(&user, &pool).await.expect("Failed to login");
    let mut app = make_service(
        |c| {
            c.service(route::reward::new);
            c.service(route::reward::update);
        },
        &pool
    ).await;
    let reward_name = "RewardName".to_string();
    let mut reward = NewReward {
        name: reward_name.clone(),
        description: "".to_string(),
        bspts: 1,
        icon: RewardIcon::default(),
    };
    let set_req = test::TestRequest::with_header("content-type", "text/plain")
        .uri("/reward")
        .method(Method::POST)
        .cookie(session_cookie.clone())
        .set_json(&reward)
        .to_request();
    let set_resp = test::call_service(&mut app, set_req).await;
    println!("{:#?}", set_resp);
    // First ensure that the call returned OK
    assert!(set_resp.status().is_success());
    let saved_reward: Reward = test::read_body_json(set_resp).await;
    // Now make a call to update the reward
    // First change the number of points
    let new_pts = 2;
    reward.bspts = new_pts;
    let put_req = test::TestRequest::with_header("content-type", "text/plain")
        .uri(format!("/reward/{}", saved_reward.id).as_str())
        .method(Method::PUT)
        .cookie(session_cookie)
        .set_json(&reward)
        .to_request();
    let put_resp = test::call_service(&mut app, put_req).await;
    println!("{:#?}", put_resp);
    // First ensure that the call returned OK
    assert!(put_resp.status().is_success());
    // Then deserialize the body and check that the
    // value of bspts changed
    let body: Reward = test::read_body_json(put_resp).await;
    assert_eq!(body.bspts, new_pts);
}

#[actix_rt::test]
async fn delete_reward() {
    let user = make_user("delete_reward");
    let pool = get_connection_pool();
    let session_cookie = login(&user, &pool).await.expect("Failed to login");
    let mut app = make_service(
        |c| {
            c.service(route::reward::get_all);
            c.service(route::reward::new);
            c.service(route::reward::delete);
        },
        &pool
    ).await;
    
    println!("Create the new reward");
    let reward_name = "RewardName".to_string();
    let reward = NewReward {
        name: reward_name.clone(),
        description: "".to_string(),
        bspts: 1,
        icon: RewardIcon::default(),
    };
    let set_req = test::TestRequest::with_header("content-type", "text/plain")
        .uri("/reward")
        .method(Method::POST)
        .cookie(session_cookie.clone())
        .set_json(&reward)
        .to_request();
    let set_resp = test::call_service(&mut app, set_req).await;
    println!("{:#?}", set_resp);
    assert!(set_resp.status().is_success());
    let saved_reward: Reward = test::read_body_json(set_resp).await;

    println!("Now make a call to delete the reward");
    let delete_req = test::TestRequest::with_header("content-type", "text/plain")
        .uri(format!("/reward/{}", saved_reward.id).as_str())
        .method(Method::DELETE)
        .cookie(session_cookie.clone())
        .to_request();
    let delete_resp = test::call_service(&mut app, delete_req).await;
    println!("{:#?}", delete_resp);
    assert!(delete_resp.status().is_success());

    println!("Finally get the whole list of rewards and make sure the deleted one is not included");
    let get_req = test::TestRequest::with_header("content-type", "text/plain")
        .uri("/reward")
        .method(Method::GET)
        .cookie(session_cookie)
        .to_request();
    let get_resp = test::call_service(&mut app, get_req).await;
    println!("{:#?}", get_resp);
    assert!(get_resp.status().is_success());
    let rewards: Vec<Reward> = test::read_body_json(get_resp).await;
    for reward in rewards {
        if reward.id == saved_reward.id {
            panic!(format!(
                "Failed to delete reward with id {} for user {}",
                reward.id,
                user.uname,
            ))
        }
    }
}

#[actix_rt::test]
async fn do_reward() {
    println!("Setup complete test");
    let user = make_user("complete_reward");
    let pool = get_connection_pool();
    let session_cookie = login(&user, &pool).await.expect("Failed to login");
    let mut app = make_service(
        |c| {
            c.service(route::user::get_user);
            c.service(route::reward::get_all);
            c.service(route::reward::get_by_id);
            c.service(route::reward::new);
            c.service(route::reward::did_it);
        },
        &pool
    ).await;
    let reward_points = 1;

    println!("Create the reward");
    let reward = NewReward {
        name: "RewardName".to_string(),
        description: "".to_string(),
        bspts: reward_points,
        icon: RewardIcon::default(),
    };
    let set_req = test::TestRequest::with_header("content-type", "text/plain")
        .uri("/reward")
        .method(Method::POST)
        .cookie(session_cookie.clone())
        .set_json(&reward)
        .to_request();
    let set_resp = test::call_service(&mut app, set_req).await;
    println!("{:#?}", set_resp);
    assert!(set_resp.status().is_success());
    let saved_reward: Reward = test::read_body_json(set_resp).await;

    println!("Now make a call to do the reward");
    let complete_req = test::TestRequest::with_header("content-type", "text/plain")
        .uri(format!("/reward/do/{id}", id = saved_reward.id).as_str())
        .method(Method::POST)
        .cookie(session_cookie.clone())
        .to_request();
    let complete_resp = test::call_service(&mut app, complete_req).await;
    println!("{:#?}", complete_resp);
    assert!(complete_resp.status().is_success());

    println!("Get the user and ensure points were subtracted");
    let get_user_req = test::TestRequest::with_header("content-type", "text/plain")
        .uri("/user")
        .method(Method::GET)
        .cookie(session_cookie.clone())
        .to_request();
    let get_user_resp = test::call_service(&mut app, get_user_req).await;
    println!("{:#?}", get_user_resp);
    assert!(get_user_resp.status().is_success());
    let user: User = test::read_body_json(get_user_resp).await;
    assert_eq!(user.bspts, -reward_points, "BS Pts were not subtracted");
}