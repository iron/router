extern crate iron;
extern crate router;

// To run, $ cargo run --example custom_404
// then go to http://localhost:3000 in a web browser

use iron::{Iron, Request, Response, IronResult, AfterMiddleware, Chain};
use iron::error::{IronError};
use iron::status;
use router::{Router, NoRoute};

struct Custom404;

impl AfterMiddleware for Custom404 {
    fn catch(&self, _: &mut Request, err: IronError) -> IronResult<Response> {
        println!("Hitting custom 404 middleware");

        if let Some(_) = err.error.downcast::<NoRoute>() {
            Ok(Response::with((status::NotFound, "Custom 404 response")))
        } else {
            Err(err)
        }
    }
}

fn main() {
    let mut router = Router::new();
    router.get("/", handler);

    let mut chain = Chain::new(router);
    chain.link_after(Custom404);

    Iron::new(chain).http("localhost:3000").unwrap();
}

fn handler(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Handling response")))
}
