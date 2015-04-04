extern crate iron;
extern crate router;

// To build, $ cargo test
// To use, go to http://127.0.0.1:3000/test

use iron::{Iron, Request, Response, IronResult};
use iron::status;
use router::{Router};

fn main() {
    let mut router = Router::new();
    // let's specify a handler for the landing page of our web page. We can respond to all GET requests this way. 
    // We specify a route as the first argument of router.get and we spefify "handler" as the function to be executed. 
    router.get("/", handler);
    // We can accept a variable after the root as an argument, in this case its called "query"
    // for example, "mywebserver.com/rust-iron", query would have the value "rust-iron"
    router.get("/:query", handler);
    
    //specify iron to run on localhost port 3000
    Iron::new(router).http("localhost:3000").unwrap();

    // this is our handler function to be called when someone send a GET to one of our routes.
    fn handler(req: &mut Request) -> IronResult<Response> {
        let ref query = req.extensions.get::<Router>()
            .unwrap().find("query").unwrap_or("/");
        Ok(Response::with((status::Ok, *query)))
    }
}
