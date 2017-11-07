extern crate iron;
extern crate serde;
extern crate serde_json;
extern crate postgres;
extern crate router;
#[macro_use]
extern crate serde_derive;


use router::Router;
use iron::prelude::*;
use iron::{status, headers};
use iron::method::Method;
use iron::modifiers::Header;

use std::io::Read;
use std::process::Command;

use postgres::{Connection, TlsMode};

#[derive(Debug, Deserialize, Serialize)]
struct Email {
	first_name: String,
	last_name: String,
	email: String
}

fn main() {

	


	let mut router = Router::new();

	router.post("/email", move |r: &mut Request| save_email(r), "save_email");


	fn save_email(req: &mut Request) -> IronResult<Response> {
		let conn = Connection::connect("postgres://postgres@localhost:5432", TlsMode::None).unwrap();

		let mut payload = String::new();

		req.body.read_to_string(&mut payload).expect("failed to parse string");


		match serde_json::from_str(&payload) {
			Ok(load) => {
				let parsed_user: Email = load;
				conn.execute("INSERT INTO emails (first_name, last_name, email) VALUES ($1, $2, $3)",
                 &[&parsed_user.first_name, &parsed_user.last_name, &parsed_user.email]).unwrap();
				println!("all good");
				let mut response = Response::with((status::Ok));
				response.set_mut(Header(headers::AccessControlAllowOrigin::Any));	
				response.set_mut(Header(headers::AccessControlAllowMethods(vec![Method::Post])));					
				Ok(response)
			},
			Err(e) => {
				println!("error");
				let mut response = Response::with((status::Ok, "error"));
				response.set_mut(Header(headers::AccessControlAllowOrigin::Any));	
				response.set_mut(Header(headers::AccessControlAllowMethods(vec![Method::Post])));					
				Ok(response)
			}
		}

		 

		 
		
	}

	Iron::new(router).http("localhost:3002").unwrap();
}
