use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct GcdParameters {
    n: u64,
    m: u64,
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }

    n
}

#[post("/gcd")]
async fn post_gcd(form: web::Form<GcdParameters>) -> impl Responder {
    if form.n == 0 && form.m == 0 {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("Computing the GCD with Zero is boring.");
    };

    let response = format!(
        "The Gretest common divisor of the numbers {} and {} is <b>{}</b\n>",
        form.n,
        form.m,
        gcd(form.n, form.m)
    );

    HttpResponse::Ok().content_type("text/html").body(response)
}

#[get("/")]
async fn get_index() -> impl Responder {
    let content: &str = r#"
        <title>GCD Calculator</title>
        <form action="/gcd" method="post">
        <input type="text" name="n"/>
        <input type="text" name="m"/>
        <button type="submit">Calculate GCD</button>
        </form>
    "#;

    HttpResponse::Ok().content_type("text/html").body(content)
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let server =
        HttpServer::new(|| App::new().service(get_index).service(post_gcd)).bind(("0.0.0.0", 8080));
    match server {
        Ok(bind) => {
            if let Some(addr) = bind.addrs().iter().nth(0) {
                println!("Server Runing On Http://{:?}", addr);
            };

            bind.run().await
        }
        Err(e) => Err(e),
    }
}
