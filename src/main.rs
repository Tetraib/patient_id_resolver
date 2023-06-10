use axum::{extract::{Path, State}, http::StatusCode, routing::post, Router};
use config::Config;
use reqwest;
use serde_json::json;
use std::net::SocketAddr;

enum RessourceType {
    Patient,
    Study,
    Series,
    Instance,
}

#[derive(Clone)]
struct AppState {
    orthanc_url: String,
}

async fn modify_patient_id(
    orthanc_url: &str,
    patient_id: &str,
    new_patient_id: &str,
) -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();
    let url = format!("{}/patients/{}/modify", orthanc_url, patient_id);

    let data = json!({
        "Replace": {
            "PatientID": new_patient_id
        },
        "Force": true,
        "KeepSource": false
    });

    client.post(&url).json(&data).send().await?;
    Ok(())
}

async fn set_metadata(
    orthanc_url: String,
    ressource_type: &str,
    orthanc_id: String,
    metadata_id: &str,
    metadata_content: &str,
) -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();
    let url = format!(
        "{}/{}/{}/metadata/{}",
        orthanc_url, ressource_type, orthanc_id, metadata_id
    );
    client
        .put(&url)
        .header("Content-Type", "application/json")
        .body(metadata_content.to_owned())
        .send()
        .await?;

    Ok(())
}

async fn get_metadata(
    ressource_type: &str,
    orthanc_id: String,
    orthanc_url: String,
    metadata_id: &str,
) -> String {
    let client = reqwest::Client::new();
    let url = format!(
        "{}/{}/{}/metadata/{}",
        orthanc_url, ressource_type, orthanc_id, metadata_id
    );
    client.get(&url).send().await.unwrap().text().await.unwrap()
}

async fn on_stable_study(
    State(state): State<AppState>, Path(study_id): Path<String>,
) -> StatusCode {
    let orthanc_url= state.orthanc_url;
    set_metadata(orthanc_url, "studies", study_id, "1025", "modified")
        .await
        .unwrap();
    StatusCode::OK
}

fn read_config(file:&str) ->Config{
     Config::builder()
        .add_source(config::File::with_name(file))
        .build()
        .unwrap()   
}

fn get_config_param(config:Config, param:&str) -> String{
    config.get::<String>(param).unwrap()
}

#[tokio::main]
async fn main() {
    let settings = read_config(".Config.toml");

    let state = AppState {
        orthanc_url: get_config_param(settings, "orthanc_url"),
    };

    let app = Router::new()
        .route("/studies/:studyId", post(on_stable_study))
        .with_state(state);
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

/*
stable study:
check patient metadata
call patient matcher
modify  async
check job
add metadata to new patient
todo:
    -add enum for ressource type
 */
