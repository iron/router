extern crate iron;
extern crate router;

// To run, $ cargo run --example custom_404
// To use, go to http://localhost:3000/foobar to see the custom 404
// Or, go to http://localhost:3000 for a standard 200 OK

use iron::{Iron, Request, Response, IronResult, AfterMiddleware, Chain};
use iron::error::{IronError};
use iron::status;
use router::{Router, RouterError};

struct Custom404;

impl AfterMiddleware for Custom404 {
    fn catch(&self, _: &mut Request, err: IronError) -> IronResult<Response> {
        println!("Hitting custom 404 middleware");

        if let Some(e) = err.error.downcast::<RouterError>() {
            if e == &RouterError::NotFound {
                return Ok(Response::with((status::NotFound, "Custom 404 response")))
            }
        }

        Err(err)
    }
}

fn main() {
    let mut router = Router::new();
    router.get("/", handler, "example");

    let mut chain = Chain::new(router);
    chain.link_after(Custom404);

    Iron::new(chain).http("localhost:3000").unwrap();
}

fn handler(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Handling response")))
}
