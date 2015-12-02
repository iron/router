extern crate iron;
extern crate router;

// To build, $ cargo test
// To use, go to http://127.0.0.1:3000/test

use iron::{Iron, Request, Response, IronResult, AfterMiddleware, Chain};
use iron::error::{IronError};
use iron::status;
use router::{Router};

struct Custom404;

impl AfterMiddleware for Custom404 {

    fn catch(&self, _: &mut Request, err: IronError) -> IronResult<Response> {
        println!("Hitting custom 404 middleware");

        match err.response.status {
            Some(status::NotFound) => Ok(Response::with((status::NotFound, "Custom 404 response"))),
            _ => Err(err)
        }
    }
}

fn main() {
    let mut router = Router::new();
    router.get("/", handler);

    let mut chain = Chain::new(router);
    chain.link_after(Custom404);

    fn handler(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "Handling response")))
    }

    Iron::new(chain).http("localhost:3000").unwrap();
}
