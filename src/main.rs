use lambda_http::{
    handler,
    lambda_runtime::{self, Context, Error},
    Body, IntoResponse, Request,
};
use serde::Deserialize;
use serde_json::de::from_str;

#[derive(Deserialize)]
struct Person {
    name: String,
    age: i32,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_runtime::run(handler(hello)).await?;
    Ok(())
}

async fn hello(request: Request, _: Context) -> Result<impl IntoResponse, Error> {
    if let Body::Text(body) = request.body() {
        if body.is_empty() {
            Ok("Hello empty".to_string())
        } else {
            let person: Person = from_str(body)?;
            Ok(format!(
                "Hello {} who is {} years old",
                person.name, person.age
            ))
        }
    } else {
        Ok("Hello stranger".to_string())
    }
}
