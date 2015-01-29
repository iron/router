extern crate iron;
extern crate router;

// To run, $ cargo run --example crud
// To use: 
// GET     http://127.0.0.1:3000/todos
// POST    http://127.0.0.1:3000/todos
// GET     http://127.0.0.1:3000/todos/:id
// PATCH   http://127.0.0.1:3000/todos/:id
// DELETE  http://127.0.0.1:3000/todos/:id

use iron::{Iron, Request, Response, IronResult};
use iron::status;
use router::{Router};

struct Controller;

impl Controller {
    fn index(req: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "Showing all items on #index action.")))
    }

    fn create(req: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "New item created on #create action.")))
    }

    fn show(req: &mut Request) -> IronResult<Response> {
        let ref id = req.extensions.get::<Router>()
            .unwrap().find("id").unwrap();
        Ok(Response::with((status::Ok, format!("Showing item {}, on #show action.", id))))
    }

    fn update(req: &mut Request) -> IronResult<Response> {
        let ref id = req.extensions.get::<Router>()
            .unwrap().find("id").unwrap();
        Ok(Response::with((status::Ok, format!("Updating item {}, on #update action.", id))))
    }

    fn delete(req: &mut Request) -> IronResult<Response> {
        let ref id = req.extensions.get::<Router>()
            .unwrap().find("id").unwrap();
        Ok(Response::with((status::Ok, format!("Deleting item {}, on #delete action.", id))))
    }
}

fn main() {
    let mut router = Router::new();
    
    router.get("/todos", Controller::index);
    router.post("/todos", Controller::create);
    router.get("/todos/:id", Controller::show);
    router.patch("/todos/:id", Controller::update);
    router.delete("/todos/:id", Controller::delete);

    Iron::new(router).listen("localhost:3000").unwrap();
}
