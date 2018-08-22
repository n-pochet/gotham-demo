extern crate gotham;
extern crate hyper;
extern crate mime;

use gotham::http::response::create_response;
use gotham::router::builder::*;
use gotham::router::Router;
use gotham::state::State;
use hyper::{Response, StatusCode};

fn index(state: State) -> (State, Response) {
    let res = create_response(
        &state,
        StatusCode::Ok,
        Some((String::from("Index").into_bytes(), mime::TEXT_PLAIN)),
    );
    (state, res)
}

fn get_appointments(state: State) -> (State, Response) {
    let res = create_response(
        &state,
        StatusCode::Ok,
        Some((
            String::from("No appointment").into_bytes(),
            mime::TEXT_PLAIN,
        )),
    );
    (state, res)
}

fn router() -> Router {
    build_simple_router(|route| {
        route.get("/").to(index);
        route.scope("/api", |route| {
            route.get("/appointments").to(get_appointments);
        })
    })
}

fn main() {
    let addr = "127.0.0.1:8080";
    println!("Listening for requests at http://{}", addr);
    gotham::start(addr, router());
}
