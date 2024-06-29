#[macro_use] extern crate rocket;

use rocket::serde::json::Json;

mod classifier;
use classifier::{classify_holders, Classification};

#[get("/classify")]
async fn classify() -> Result<Json<Classification>, Json<String>> {
    let api_url = "https://tn11api.kasplex.org/v1/krc20/token/NACHO?holder=true";
    match classify_holders(api_url).await {
        Ok(classification) => Ok(Json(classification)),
        Err(e) => Err(Json(format!("Error: {}", e))),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![classify])
}
