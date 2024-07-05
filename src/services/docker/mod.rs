pub mod error;
pub mod types;

use error::DockerError;
use types::Service;

/// Docker Builder
///
/// # Example
///
/// ```rust
/// let docker: Docker = DockerBuilder::builder()
///     .with_http_url("http://localhost:8080")
///     .build();
/// ```
///
pub struct DockerBuilder {
    http_url: String,
}

impl Default for DockerBuilder {
    fn default() -> Self {
        DockerBuilder::builder()
    }
}

impl DockerBuilder {
    pub fn builder() -> Self {
        DockerBuilder {
            http_url: "http://localhost:8080".to_owned(),
        }
    }
    pub fn with_http_url(mut self, http_url: &str) -> Self {
        self.http_url = http_url.to_owned();
        self
    }
    pub fn build(self) -> Docker {
        Docker::new(&self.http_url)
    }
}

/// Docker API
///
/// Implement the basic services to interact with the Docker API
/// - services_list: List all services
///
pub struct Docker {
    http_url: String,
}

impl Docker {
    /// Create a new Docker instance
    ///
    /// # Arguments
    /// * `http_url` - The URL of the Docker API
    ///
    /// # Example
    ///
    /// ```rust
    /// let docker = Docker::new("http://localhost:8080".to_owned());
    /// ```
    ///
    pub fn new(http_url: &str) -> Self {
        Docker {
            http_url: http_url.to_owned(),
        }
    }
    /// List all services
    ///
    /// # Example
    ///
    /// ```rust
    /// let docker = Docker::new("http://localhost:8080".to_owned());
    /// let services = docker.services_list().await.unwrap();
    /// for service in services {
    ///    println!("{:?}", service);
    /// }
    /// ```
    pub async fn services_list(&self) -> Result<Vec<Service>, DockerError> {
        let url = format!("{}/services", self.http_url);
        let services = reqwest::get(&url)
            .await?
            .json::<Vec<Service>>()
            .await?
            .into_iter()
            .map(|service| service.with_service_http(&self.http_url))
            .collect();
        Ok(services)
    }
    // pub async fn services_list(&self) -> Result<Vec<Service>> {
    //     let url = format!("{}/services", self.http_url);
    //     let response = reqwest::get(&url).await?.text().await?;
    //     let response = &mut serde_json::Deserializer::from_str(&response);
    //     serde_path_to_error::deserialize(response).map_err(|e| anyhow!("Error: {}", e))
    // }
}

impl Service {
    pub async fn update_image(&mut self, image: &str, tag: &str) -> Result<String, DockerError> {
        let url = format!(
            "{}/update?version={}",
            self.service_http_url, self.version.index
        );
        self.spec.task_template.container_spec.image = format!("{}:{}", image, tag);
        if let Some(labels) = &mut self.spec.labels {
            labels.insert(
                "com.docker.stack.image".into(),
                self.spec.task_template.container_spec.image.clone(),
            );
        }
        let client = reqwest::Client::new();
        let response = client.post(&url).json(&self.spec).send().await?;
        if response.status().is_success() {
            Ok("Service updated".to_owned())
        } else {
            Err(DockerError::ServiceUpdateError(response.text().await?))
        }
    }
    pub fn with_service_http(mut self, http_url: &str) -> Self {
        self.service_http_url = format!("{}/services/{}", http_url, self.id);
        self
    }
}

impl From<types::Service> for types::ServiceResume {
    fn from(value: types::Service) -> Self {
        let mut parts = value.spec.task_template.container_spec.image.split(':');
        let image = parts.next().unwrap().to_string();
        let tag = parts.next().unwrap().split('@').next().unwrap().to_string();
        Self {
            id: value.id.clone(),
            version: value.version.index,
            created_at: value.created_at,
            updated_at: value.updated_at,
            name: value.spec.name.clone(),
            image,
            tag,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_service_list() {
        let docker = DockerBuilder::builder().build();
        let services = docker.services_list().await.unwrap();
        assert!(services.len() > 0);
    }
}
