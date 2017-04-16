extern crate iron;
extern crate router;

// To run, $ cargo run --example crud
// To use: 
// GET     http://127.0.0.1:3000/todos
// POST    http://127.0.0.1:3000/todos
// GET     http://127.0.0.1:3000/todos/:id
// PATCH   http://127.0.0.1:3000/todos/:id
// DELETE  http://127.0.0.1:3000/todos/:id

use iron::Iron;
use router::Router;

mod controller {
    use iron::{Request, Response, IronResult};
    use iron::status;
    use router::Router;

    pub fn index(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "Showing all items on #index action.")))
    }

    pub fn create(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "New item created on #create action.")))
    }

    pub fn show(req: &mut Request) -> IronResult<Response> {
        let ref id = req.extensions.get::<Router>()
            .unwrap().find("id").unwrap();
        Ok(Response::with((status::Ok, format!("Showing item {}, on #show action.", id))))
    }

    pub fn update(req: &mut Request) -> IronResult<Response> {
        let ref id = req.extensions.get::<Router>()
            .unwrap().find("id").unwrap();
        Ok(Response::with((status::Ok, format!("Updating item {}, on #update action.", id))))
    }

    pub fn delete(req: &mut Request) -> IronResult<Response> {
        let ref id = req.extensions.get::<Router>()
            .unwrap().find("id").unwrap();
        Ok(Response::with((status::Ok, format!("Deleting item {}, on #delete action.", id))))
    }
}

fn main() {
    let mut router = Router::new();
    
    router.get("/todos", controller::index);
    router.post("/todos", controller::create);
    router.get("/todos/:id", controller::show);
    router.patch("/todos/:id", controller::update);
    router.delete("/todos/:id", controller::delete);

    Iron::new(router).listen("localhost:3000").unwrap();
}
