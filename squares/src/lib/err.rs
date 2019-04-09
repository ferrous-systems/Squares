#![feature(proc_macro_hygiene, decl_macro)]

//#[macro_use] extern crate error_chain;
//#[macro_use] extern crate rocket_contrib;
extern crate rocket;

use rocket_contrib::json::{Json, JsonValue};
use std::io::{Error, ErrorKind};


pub mod echain {
    use rocket::request::Request;
    use rocket::response::{Response, Responder};
    use std::io::Cursor;
    use rocket::http::{Status, ContentType};
    use serde_json::json;
    // This generates basic Error, Result, etc. types
    error_chain!{ }

    // Implement `Responder` for `error_chain`'s `Error` type
    // that we just generated
    impl<'r> Responder<'r> for Error {
        fn respond_to(self, _: &Request) -> ::std::result::Result<Response<'r>, Status> {
            // Render the whole error chain to a single string
            let mut rslt = String::new();
            rslt += &format!("Error: {}", self);
            self.iter().skip(1).map(|ce| rslt += &format!(", caused by: {}", ce)).collect::<Vec<_>>();

            // Create JSON response
            let resp = json!({
                "status": "failure",
                "message": rslt,
            }).to_string();

            // Respond. The `Ok` here is a bit of a misnomer. It means we
            // successfully created an error response
            Ok(Response::build()
                .status(Status::BadRequest)
                .header(ContentType::JSON)
                .sized_body(Cursor::new(resp))
                .finalize())
        }
    }

}


pub fn is_in_column_range(number: i32) -> std::io::Result<()> {
    if number < 0 || number > 14 {
        Err(Error::new(ErrorKind::Other, "Column value out of range"))
    } else {
        Ok(())
    }
}
