use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde_json::json;
use serde::{Serialize, Deserialize};

const API_PREFIX: &'static str = "/rust-api";

#[derive(Serialize, Deserialize)]
struct ChartData {
    name: String,
    chart_type: String,
    chart_data: Vec<i32>,
}

#[get("/chart-data")]
async fn get_chart_data() -> impl Responder {
    let data = ChartData {
        name: String::from("test-chart"),
        chart_type: String::from("line"),
        chart_data: vec![1, 2, 3, 4, 5],
    };

    println!("{:?}", json!(data));
    HttpResponse::Ok().json(json!(data))
}

#[post("/chart-data")]
async fn post_chart_data(data: web::Json<ChartData>) -> impl Responder {
    println!("{:?}", json!(data));

    HttpResponse::Ok().json(json!({
        "status": "ok"
    }))
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::scope(API_PREFIX)
                .service(get_chart_data)
                .service(post_chart_data)
        )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await?;

    Ok(())
}
