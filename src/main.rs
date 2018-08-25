extern crate gotham;
extern crate hyper;
extern crate mime;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use gotham::handler::IntoResponse;
use gotham::http::response::create_response;
use gotham::router::builder::*;
use gotham::router::Router;
use gotham::state::State;
use hyper::{Response, StatusCode};

#[derive(Serialize)]
struct Appointment {
    date: String,
    patient: String,
    doctor: String,
}

impl IntoResponse for Appointment {
    fn into_response(self, state: &State) -> Response {
        create_response(
            state,
            StatusCode::Ok,
            Some((
                serde_json::to_string(&self)
                    .expect("serialized appointments")
                    .into_bytes(),
                mime::APPLICATION_JSON,
            )),
        )
    }
}

fn index(state: State) -> (State, Response) {
    let res = create_response(
        &state,
        StatusCode::Ok,
        Some((String::from("Index").into_bytes(), mime::TEXT_PLAIN)),
    );
    (state, res)
}

fn get_appointment(state: State) -> (State, Appointment) {
    let appointment = Appointment {
        date: "now".to_string(),
        patient: "Jon".to_string(),
        doctor: "Mac".to_string(),
    };
    (state, appointment)
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
            route.get("/appointments/1").to(get_appointment);
        })
    })
}

fn main() {
    let addr = "127.0.0.1:8080";
    println!("Listening for requests at http://{}", addr);
    gotham::start(addr, router());
}
