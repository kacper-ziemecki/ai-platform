#[macro_use]
extern crate rocket;
use rocket::fs::NamedFile;
use rocket::http::{Cookie, CookieJar};
use rocket::response::Redirect;
use serde_json::Value;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, login, login_api])
}

#[get("/")]
async fn index(cookies: &CookieJar<'_>) -> Result<Option<NamedFile>, Redirect> {
    cookies.get("UUID").map(|c| println!("UUID: {}", c.value()));
    match cookies.get("UUID") {
        Some(coockie) => {
            if coockie.value() != "1234567890" {
                println!("test1");
                return Err(Redirect::to(uri!(login)));
            }
        }
        None => {
            println!("test2");
            return Err(Redirect::to(uri!(login)));
        }
    }
    println!("test3");
    return Ok(NamedFile::open("./website/index.html").await.ok());
}

#[get("/login")]
async fn login() -> Option<NamedFile> {
    NamedFile::open("./website/login.html").await.ok()
}

#[post("/login", data = "<raw_data>")]
fn login_api<'a>(raw_data: &'a str, cookies: &'a CookieJar<'a>) -> Result<Redirect, &'a str> {
    let data: Value = serde_json::from_str(&raw_data).unwrap();
    if data["login"].as_str().unwrap() == "admin"
        && data["password"].as_str().unwrap() == "password"
    {
        cookies.add(Cookie::build(("UUID", "1234567890")).path("/").secure(true));
        return Ok(Redirect::to(uri!(index)));
    }
    Err("nie zalogowano")
}
