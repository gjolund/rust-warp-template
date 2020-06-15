use serde::{Deserialize, Serialize};
use warp::{filters::BoxedFilter, Filter, Reply};

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    message: String,
}

impl warp::Reply for Response {
    fn into_response(self) -> warp::reply::Response {
        warp::reply::json(&self).into_response()
    }
}

impl Response {
    fn new(user_agent: String) -> Response {
        Response {
            message: create_message(&user_agent),
        }
    }
}

fn create_message(user_agent: &str) -> String {
    format!("Hello world!!!! Your user agent is {}", user_agent)
}

pub fn main() -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(warp::header("user-agent"))
        .map(Response::new)
        .boxed()
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use std::str;
    use warp::http::StatusCode;
    use warp::test::request;

    use super::*;

    #[tokio::test]
    async fn test_main() {
        let fn_handler = main();
        let user_agent = "test";
        let expected_body = json!({ "message": create_message(user_agent) });

        let resp = request()
            .method("GET")
            .header("user-agent", user_agent)
            .reply(&fn_handler)
            .await;

        let body = str::from_utf8(resp.body()).unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(body, expected_body.to_string());
    }
}
