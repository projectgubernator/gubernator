#[macro_use]
extern crate log;

use gubernator_cri::runtime_service_server::{RuntimeService, RuntimeServiceServer};
use tonic::transport::Server;

#[derive(Debug, Default)]
pub struct MockRuntimeService;

#[tonic::async_trait]
impl RuntimeService for MockRuntimeService {
    async fn version(
        &self,
        _request: tonic::Request<gubernator_cri::VersionRequest>,
    ) -> Result<tonic::Response<gubernator_cri::VersionResponse>, tonic::Status> {
        info!("received VersionRequest");
        Ok(tonic::Response::new(gubernator_cri::VersionResponse {
            version: String::from("testing"),
            runtime_name: String::from("testing"),
            runtime_api_version: String::from("testing"),
            runtime_version: String::from("testing"),
        }))
    }

    async fn run_pod_sandbox(
        &self,
        _request: tonic::Request<gubernator_cri::RunPodSandboxRequest>,
    ) -> Result<tonic::Response<gubernator_cri::RunPodSandboxResponse>, tonic::Status> {
        todo!()
    }

    async fn stop_pod_sandbox(
        &self,
        _request: tonic::Request<gubernator_cri::StopPodSandboxRequest>,
    ) -> Result<tonic::Response<gubernator_cri::StopPodSandboxResponse>, tonic::Status> {
        todo!()
    }

    async fn remove_pod_sandbox(
        &self,
        _request: tonic::Request<gubernator_cri::RemovePodSandboxRequest>,
    ) -> Result<tonic::Response<gubernator_cri::RemovePodSandboxResponse>, tonic::Status> {
        todo!()
    }

    async fn pod_sandbox_status(
        &self,
        _request: tonic::Request<gubernator_cri::PodSandboxStatusRequest>,
    ) -> Result<tonic::Response<gubernator_cri::PodSandboxStatusResponse>, tonic::Status> {
        todo!()
    }

    async fn list_pod_sandbox(
        &self,
        _request: tonic::Request<gubernator_cri::ListPodSandboxRequest>,
    ) -> Result<tonic::Response<gubernator_cri::ListPodSandboxResponse>, tonic::Status> {
        todo!()
    }

    async fn create_container(
        &self,
        _request: tonic::Request<gubernator_cri::CreateContainerRequest>,
    ) -> Result<tonic::Response<gubernator_cri::CreateContainerResponse>, tonic::Status> {
        todo!()
    }

    async fn start_container(
        &self,
        _request: tonic::Request<gubernator_cri::StartContainerRequest>,
    ) -> Result<tonic::Response<gubernator_cri::StartContainerResponse>, tonic::Status> {
        todo!()
    }

    async fn stop_container(
        &self,
        _request: tonic::Request<gubernator_cri::StopContainerRequest>,
    ) -> Result<tonic::Response<gubernator_cri::StopContainerResponse>, tonic::Status> {
        todo!()
    }

    async fn remove_container(
        &self,
        _request: tonic::Request<gubernator_cri::RemoveContainerRequest>,
    ) -> Result<tonic::Response<gubernator_cri::RemoveContainerResponse>, tonic::Status> {
        todo!()
    }

    async fn list_containers(
        &self,
        _request: tonic::Request<gubernator_cri::ListContainersRequest>,
    ) -> Result<tonic::Response<gubernator_cri::ListContainersResponse>, tonic::Status> {
        todo!()
    }

    async fn container_status(
        &self,
        _request: tonic::Request<gubernator_cri::ContainerStatusRequest>,
    ) -> Result<tonic::Response<gubernator_cri::ContainerStatusResponse>, tonic::Status> {
        todo!()
    }

    async fn update_container_resources(
        &self,
        _request: tonic::Request<gubernator_cri::UpdateContainerResourcesRequest>,
    ) -> Result<tonic::Response<gubernator_cri::UpdateContainerResourcesResponse>, tonic::Status>
    {
        todo!()
    }

    async fn reopen_container_log(
        &self,
        _request: tonic::Request<gubernator_cri::ReopenContainerLogRequest>,
    ) -> Result<tonic::Response<gubernator_cri::ReopenContainerLogResponse>, tonic::Status> {
        todo!()
    }

    async fn exec_sync(
        &self,
        _request: tonic::Request<gubernator_cri::ExecSyncRequest>,
    ) -> Result<tonic::Response<gubernator_cri::ExecSyncResponse>, tonic::Status> {
        todo!()
    }

    async fn exec(
        &self,
        _request: tonic::Request<gubernator_cri::ExecRequest>,
    ) -> Result<tonic::Response<gubernator_cri::ExecResponse>, tonic::Status> {
        todo!()
    }

    async fn attach(
        &self,
        _request: tonic::Request<gubernator_cri::AttachRequest>,
    ) -> Result<tonic::Response<gubernator_cri::AttachResponse>, tonic::Status> {
        todo!()
    }

    async fn port_forward(
        &self,
        _request: tonic::Request<gubernator_cri::PortForwardRequest>,
    ) -> Result<tonic::Response<gubernator_cri::PortForwardResponse>, tonic::Status> {
        todo!()
    }

    async fn container_stats(
        &self,
        _request: tonic::Request<gubernator_cri::ContainerStatsRequest>,
    ) -> Result<tonic::Response<gubernator_cri::ContainerStatsResponse>, tonic::Status> {
        todo!()
    }

    async fn list_container_stats(
        &self,
        _request: tonic::Request<gubernator_cri::ListContainerStatsRequest>,
    ) -> Result<tonic::Response<gubernator_cri::ListContainerStatsResponse>, tonic::Status> {
        todo!()
    }

    async fn update_runtime_config(
        &self,
        _request: tonic::Request<gubernator_cri::UpdateRuntimeConfigRequest>,
    ) -> Result<tonic::Response<gubernator_cri::UpdateRuntimeConfigResponse>, tonic::Status> {
        todo!()
    }

    async fn status(
        &self,
        _request: tonic::Request<gubernator_cri::StatusRequest>,
    ) -> Result<tonic::Response<gubernator_cri::StatusResponse>, tonic::Status> {
        todo!()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    info!("starting MockRuntimeService on port 549");

    let address = "0.0.0.0:549".parse().expect("failed to parse address");
    Server::builder()
        .add_service(RuntimeServiceServer::new(MockRuntimeService::default()))
        .serve(address)
        .await?;

    Ok(())
}
