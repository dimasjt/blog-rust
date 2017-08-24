extern crate iron;
extern crate logger;
extern crate env_logger;
extern crate router;

use iron::prelude::*;
use iron::{AfterMiddleware};
use iron::error::{IronError};
use iron::status;

use logger::Logger;
use router::{Router, NoRoute};

struct Custom404;

impl AfterMiddleware for Custom404 {
    fn catch(&self, _: &mut Request, err: IronError) -> IronResult<Response> {
        if let Some(_) = err.error.downcast::<NoRoute>() {
            Ok(Response::with((status::NotFound, "Custom 404 response")))
        } else {
            Err(err)
        }
    }
}

fn main() {
    env_logger::init().unwrap();

    let (logger_before, _) = Logger::new(None);
    let mut router = Router::new();

    router.get("/", home_ctrl, "home");
    router.get("/posts/:id", show_ctrl, "show");

    let mut chain = Chain::new(router);
    chain.link_before(logger_before);
    chain.link_after(Custom404);

    match Iron::new(chain).http("localhost:3000") {
        Result::Ok(listening) => println!("{:?}", listening),
        Result::Err(err) => panic!("{:?}", err),
    }
}

fn home_ctrl(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Home Page")))
}

fn show_ctrl(req: &mut Request) -> IronResult<Response> {
    let ref id = req.extensions.get::<Router>().unwrap().find("id").unwrap();
    let mut content = String::from("Show");
    content.push_str(id);

    Ok(Response::with((status::Ok, content)))
}
