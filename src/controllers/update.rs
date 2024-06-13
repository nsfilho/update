use axum::Json;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tracing::{debug, info};
use uuid::Uuid;

use crate::services::docker::{types::Service, DockerBuilder};

#[derive(Debug, Deserialize)]
pub(crate) struct UpdateServiceRequest {
    image: String,
    tag: String,
    service: Option<String>,
}

#[derive(Debug, Serialize)]
pub(crate) struct UpdateServiceResponseData {
    pub id: String,
    pub version: u64,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
    pub name: String,
    pub image: String,
    #[serde(rename = "fromTag")]
    pub from_tag: String,
}

#[derive(Debug, Serialize)]
pub(crate) struct UpdateServiceResponse {
    code: String,
    transaction: String,
    message: String,
    args: Vec<String>,
    data: Vec<UpdateServiceResponseData>,
}

pub async fn update_service(
    Json(payload): Json<UpdateServiceRequest>,
) -> Json<UpdateServiceResponse> {
    let transaction = Uuid::new_v4();

    let docker = DockerBuilder::builder().build();
    let service_image_filter = format!("{}:", payload.image);
    let services = docker.services_list().await.unwrap();
    let mut services: Vec<(UpdateServiceResponseData, Service)> = services
        .into_iter()
        .filter(|service| {
            if let Some(name) = &payload.service {
                return service.spec.name == *name;
            }
            service
                .spec
                .task_template
                .container_spec
                .image
                .starts_with(&service_image_filter)
        })
        .map(|service| {
            let mut parts = service.spec.task_template.container_spec.image.split(':');
            let image = parts.next().unwrap().to_string();
            let from_tag = parts.next().unwrap().split('@').next().unwrap().to_string();
            (
                UpdateServiceResponseData {
                    id: service.id.clone(),
                    version: service.version.index,
                    created_at: service.created_at,
                    updated_at: service.updated_at,
                    name: service.spec.name.clone(),
                    image,
                    from_tag,
                },
                service,
            )
        })
        .collect();

    for (response, service) in services.iter_mut() {
        info!("Updating service: {:?}", service);
        let answer = docker
            .services_update_image(
                response.id.as_str(),
                response.version,
                &payload.image,
                &payload.tag,
                service,
            )
            .await
            .unwrap();
        debug!("Response: {}", answer);
    }

    Json(UpdateServiceResponse {
        code: "200".to_string(),
        transaction: transaction.to_string(),
        message: "Service updated".to_string(),
        args: vec![],
        data: services
            .into_iter()
            .map(|(response, _)| response)
            .collect::<Vec<UpdateServiceResponseData>>(),
    })
}
