extern crate iron;
extern crate logger;
extern crate env_logger;
extern crate router;
extern crate bodyparser;
extern crate persistent;
extern crate rustc_serialize;

// #[macro_use] extern crate serde_derive;

use iron::prelude::*;
use iron::{AfterMiddleware};
use iron::error::{IronError};
use iron::status;
// use logger::Logger;
use router::{Router, NoRoute};
use persistent::Read;
use rustc_serialize::json;

struct Custom404;

#[derive(RustcEncodable)]
struct Article {
    title: String
}

const MAX_BODY_LENGTH: usize = 1024 * 1024 * 10;

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

    // let (logger_before, _) = Logger::new(None);
    let mut router = Router::new();

    router.get("/", home_ctrl, "home");
    router.get("/posts/:id", show_ctrl, "show");

    let mut chain = Chain::new(router);
    chain.link_before(Read::<bodyparser::MaxBodyLength>::one(MAX_BODY_LENGTH));
    chain.link_after(Custom404);

    match Iron::new(chain).http("localhost:3000") {
        Result::Ok(listening) => println!("{:?}", listening),
        Result::Err(err) => panic!("{:?}", err),
    }
}

fn home_ctrl(_: &mut Request) -> IronResult<Response> {
    let article = Article { title: "Learning Rust".to_string() };
    let arrs: Vec<Article> = vec![article];
    let payload = json::encode(&arrs).unwrap();
    Ok(Response::with((status::Ok, payload)))
}

fn show_ctrl(req: &mut Request) -> IronResult<Response> {
    let ref id = req.extensions.get::<Router>().unwrap().find("id").unwrap();
    let mut content = String::from("Show");
    content.push_str(id);

    Ok(Response::with((status::Ok, content)))
}
