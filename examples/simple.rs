extern crate iron;
extern crate http;
extern crate router;

// To build `make all examples`.
// To use, go to http://127.0.0.1:3000/test
// Anything after :3000/ will be written to the browser.

use std::io::net::ip::Ipv4Addr;
use iron::{Server, Iron, Alloy, Request, Response, Chain, Status, Unwind, FromFn};
use http::method::Get;
use http::status;
use router::{Router, Params};

fn main() {
    let mut server: Server = Iron::new();
    let mut router = Router::new();

    fn handler(req: &mut Request, res: &mut Response) -> Status {
        let query = req.alloy.find::<Params>().unwrap().get("query").unwrap();
        let _ = res.serve(status::Ok, query);
        Unwind
    }

    // Setup our route.
    router.route(
        Get,
        "/:query".to_string(),
        vec!["query".to_string()],
        FromFn::new(handler));

    server.chain.link(router);
    server.listen(Ipv4Addr(127, 0, 0, 1), 3000);
}

