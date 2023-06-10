use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use reqwest::{Client};
use serde::Deserialize;
use chrono::{DateTime, Utc, Duration};

#[derive(Deserialize)]
struct StudyMetadata {
    modified: Option<DateTime<Utc>>,
    // other metadata fields...
}

async fn get_study_metadata(study_id: &str) -> Result<StudyMetadata, reqwest::Error> {
    let client = Client::new();
    let url = format!("https://demo.orthanc-server.com/studies/{}", study_id);
    let resp = client.get(&url).send().await?;
    let metadata: StudyMetadata = resp.json().await?;
    Ok(metadata)
}

async fn process_stable_study(path: web::Path<String>) -> impl Responder {
    let study_id = path.into_inner();
    let metadata = get_study_metadata(&study_id).await.unwrap();

    let now = Utc::now();
    let one_hour_ago = now - Duration::hours(1);

    if let Some(modified) = metadata.modified {
        if modified > one_hour_ago {
            println!("Skipping study {} because it was recently modified", study_id);
            return HttpResponse::Ok().body("Study recently modified, skipping processing");
        }
    }

    // TODO: Implement actual processing logic here
    println!("Received stable study with ID: {}", study_id);

    HttpResponse::Ok().body("Processing complete")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/study/{id}", web::post().to(process_stable_study))
    })
    .bind("127.0.0.1:3030")?
    .run()
    .await
}
