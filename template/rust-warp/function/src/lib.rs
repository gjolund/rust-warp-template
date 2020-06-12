use warp::{Filter, filters::BoxedFilter, Reply};

pub fn main() -> BoxedFilter<(impl Reply,)> {
  // GET / => 200 OK with body "Hello Warp World!"
  warp::get()
    .map(|| String::from("Hello Warp World!"))
    .boxed()
}