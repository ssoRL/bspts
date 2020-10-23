use warp::Filter;

#[tokio::main]
async fn main() {

    let hi_route = warp::path!("hi").and(warp::get()).map(|| "Hi");
    let file_server = warp::fs::dir("../site");

    warp::serve(hi_route.or(file_server))
        .run(([127, 0, 0, 1], 3030))
        .await;
}