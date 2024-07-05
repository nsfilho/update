use futures::stream::StreamExt;
use std::sync::Arc;

use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use tracing::info;
use uuid::Uuid;

use crate::{services::docker::types::ServiceResume, AppState};

#[derive(Debug, Deserialize)]
pub(crate) struct UpdateServiceRequest {
    image: String,
    tag: String,
    service: Option<String>,
}

#[derive(Debug, Serialize)]
pub(crate) struct UpdateServiceResponse {
    code: String,
    transaction: String,
    message: String,
    args: Vec<String>,
    data: Vec<ServiceResume>,
}

pub async fn update_service(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<UpdateServiceRequest>,
) -> Json<UpdateServiceResponse> {
    let transaction = Uuid::new_v4();

    let service_image_filter = format!("{}:", payload.image);
    let services = state.docker.services_list().await.unwrap();
    let mut services = services
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
        .map(|service| async {
            info!("Updating service: {:?}", service);
            service.update_image(&payload.image, &payload.tag).await;
            service.into()
            // ServiceResume::from(service)
        })
        .collect::<Vec<ServiceResume>>()
        .await;

    Json(UpdateServiceResponse {
        code: "200".to_string(),
        transaction: transaction.to_string(),
        message: "Service updated".to_string(),
        args: vec![],
        data: services
            .into_iter()
            .map(|service| service.into())
            .collect::<Vec<ServiceResume>>(),
    })
}
