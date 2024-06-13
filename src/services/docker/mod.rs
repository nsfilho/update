use anyhow::{anyhow, Result};

pub mod types;

use types::Service;

pub struct Docker {
    http_url: String,
}

impl Docker {
    pub fn new(http_url: String) -> Self {
        Docker { http_url }
    }
    pub async fn services_list(&self) -> Result<Vec<Service>> {
        let url = format!("{}/services", self.http_url);
        reqwest::get(&url)
            .await?
            .json::<Vec<Service>>()
            .await
            .map_err(|e| anyhow!("Error: {}", e))
    }
    // pub async fn services_list(&self) -> Result<Vec<Service>> {
    //     let url = format!("{}/services", self.http_url);
    //     let response = reqwest::get(&url).await?.text().await?;
    //     let response = &mut serde_json::Deserializer::from_str(&response);
    //     serde_path_to_error::deserialize(response).map_err(|e| anyhow!("Error: {}", e))
    // }
    pub async fn services_update_image(
        &self,
        id: &str,
        version: u64,
        image: &str,
        tag: &str,
        service: &mut Service,
    ) -> Result<String> {
        let url = format!(
            "{}/services/{}/update?version={}",
            self.http_url, id, version
        );
        service.spec.task_template.container_spec.image = format!("{}:{}", image, tag);
        let client = reqwest::Client::new();
        let response = client
            .post(&url)
            .json(&service.spec)
            .send()
            .await
            .map_err(|e| anyhow!("Error: {}", e))?;
        let response = response.text().await?;
        Ok(response)
    }
}

pub struct DockerBuilder {
    http_url: String,
}

impl Default for DockerBuilder {
    fn default() -> Self {
        DockerBuilder {
            http_url: "http://localhost:8080".to_owned(),
        }
    }
}

impl DockerBuilder {
    pub fn builder() -> Self {
        DockerBuilder::default()
    }
    pub fn build(self) -> Docker {
        Docker::new(self.http_url)
    }
    pub fn with_http_url(mut self, http_url: &str) -> Self {
        self.http_url = http_url.to_owned();
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_service_list() {
        let docker = DockerBuilder::builder().build();
        let result = docker.services_list().await.unwrap();
        let content = serde_json::to_string(&result).unwrap();
        let mut f = tokio::fs::File::create("rust_service.json").await.unwrap();
        f.write_all(content.as_bytes()).await.unwrap();
    }
}
