extern crate iron;

use iron::prelude::*;

fn main() {
    let mut chain = Chain::new(hello_world);

    Iron::new(chain).http("localhost:3000").unwrap();
}

fn hello_world(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((iron::status::Ok, "Hello")))
}
