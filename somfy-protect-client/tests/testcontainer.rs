use std::{
    env,
    fmt::Display,
    time::{Duration, Instant},
};

use docker_api::{
    models::ImageBuildChunk,
    opts::{ContainerCreateOpts, ContainerStopOpts, ImageBuildOpts, LogsOpts},
    Container, Docker,
};
use futures_util::StreamExt;

pub struct SomfyMockContainer {
    container: HighLevelContainer,
}

impl SomfyMockContainer {
    pub async fn new() -> Self {
        let cwd = env::var("CARGO_MANIFEST_DIR").unwrap();
        let docker = HighLevelDockerClient::default();
        docker
            .build(
                &ImageBuildOpts::builder(format!("{cwd}/../mock"))
                    .tag("somfy-protect-api-mock")
                    .build(),
            )
            .await
            .unwrap();
        let container = docker
            .create(
                &ContainerCreateOpts::builder()
                    .image("somfy-protect-api-mock")
                    .publish_all_ports()
                    .build(),
            )
            .await
            .unwrap()
            .ready_on_log("Somfy Protect API mock listening on port 3000");
        Self { container }
    }

    pub async fn start(self) -> Self {
        self.container.start().await.unwrap();
        self
    }

    pub async fn stop(&self) -> Result<(), docker_api::Error> {
        self.container
            .inner_container
            .stop(&ContainerStopOpts::builder().build())
            .await
    }

    pub async fn get_server_port(&self) -> u16 {
        self.container.get_mapped_port("3000/tcp").await.unwrap()
    }
}

struct HighLevelDockerClient {
    docker: Docker,
}

impl Default for HighLevelDockerClient {
    fn default() -> Self {
        Self {
            docker: Docker::unix("/var/run/docker.sock"),
        }
    }
}

impl HighLevelDockerClient {
    async fn build(&self, opts: &ImageBuildOpts) -> Result<(), docker_api::Error> {
        format!("{:?}", opts);
        let images = self.docker.images();
        let mut stream = images.build(opts);
        while let Some(build_chunk) = stream.next().await {
            match build_chunk {
                Ok(build_chunk) => println!("{}", Loggable::from(build_chunk)),
                Err(error) => return Err(error),
            }
        }
        Ok(())
    }

    async fn create(
        &self,
        opts: &ContainerCreateOpts,
    ) -> Result<HighLevelContainer, docker_api::Error> {
        Ok(HighLevelContainer::new(
            self.docker.containers().create(&opts).await?,
        ))
    }
}

struct Loggable {
    message: String,
}

impl From<ImageBuildChunk> for Loggable {
    fn from(chunk: ImageBuildChunk) -> Self {
        let message = match chunk {
            ImageBuildChunk::Update { stream } => format!("Update: {stream}"),
            ImageBuildChunk::Error {
                error,
                error_detail,
            } => format!("Error: {error}: {}", error_detail.message),
            ImageBuildChunk::Digest { aux } => format!("Digest: {}", aux.id),
            ImageBuildChunk::PullStatus {
                status,
                id,
                progress,
                progress_detail: _,
            } => {
                let id = id.unwrap_or("".to_string());
                let progress = progress.unwrap_or("".to_string());
                format!("Pull: {status} {id} {progress}")
            }
        };
        Loggable { message }
    }
}

impl Display for Loggable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = self.message.trim();
        write!(f, "🐋 {message}")
    }
}

struct HighLevelContainer {
    inner_container: Container,
    ready_log: Option<String>,
    ready_timeout: Duration,
}

impl HighLevelContainer {
    fn new(container: Container) -> Self {
        HighLevelContainer {
            inner_container: container,
            ready_log: None,
            ready_timeout: Duration::from_secs(30),
        }
    }

    fn ready_on_log(mut self, log: &str) -> Self {
        self.ready_log = Some(log.into());
        self
    }

    async fn logs(&self, opts: &LogsOpts) -> Result<String, docker_api::Error> {
        let logs = self
            .inner_container
            .logs(&opts)
            .map(|chunk| match chunk {
                Ok(chunk) => chunk.to_vec(),
                Err(e) => {
                    eprintln!("Error: {e}");
                    vec![]
                }
            })
            .collect::<Vec<_>>()
            .await
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();
        Ok(String::from_utf8_lossy(&logs).to_string())
    }

    async fn start(&self) -> Result<(), docker_api::Error> {
        self.inner_container.start().await?;
        if let Some(ready_log) = self.ready_log.clone() {
            let timeout_instant = Instant::now() + self.ready_timeout;
            let stdout = LogsOpts::builder().stdout(true).all().build();
            loop {
                let logs = self.logs(&stdout).await?;
                if logs.contains(ready_log.as_str()) {
                    break;
                }
                if timeout_instant < Instant::now() {
                    return Err(docker_api::Error::StringError(
                        "Container takes too much time to be ready".to_string(),
                    ));
                } else {
                    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                }
            }
        }
        let name = self
            .inner_container
            .inspect()
            .await
            .unwrap()
            .name
            .unwrap_or(self.inner_container.id().to_string());
        println!("🐋 Container {name} is ready");
        Ok(())
    }

    async fn get_mapped_port(&self, port_spec: &str) -> Option<u16> {
        let published_ports = self
            .inner_container
            .inspect()
            .await
            .ok()?
            .network_settings?
            .ports?;
        let port_bindings = published_ports[port_spec].clone()?;
        let port_binding = port_bindings
            .iter()
            .find(|p| p.host_ip == Some("0.0.0.0".to_string()))?
            .clone();
        let port = port_binding.host_port.unwrap();
        port.parse::<u16>().ok()
    }
}
