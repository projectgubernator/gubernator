use gubernator::generated::cri;
use gubernator::generated::cri::runtime_service_server::{RuntimeService, RuntimeServiceServer};
use tonic::transport::Server;

#[derive(Debug, Default)]
pub struct CRIRuntimeService;

#[tonic::async_trait]
impl RuntimeService for CRIRuntimeService {
    async fn version(
        &self,
        _request: tonic::Request<cri::VersionRequest>,
    ) -> Result<tonic::Response<cri::VersionResponse>, tonic::Status> {
        todo!()
    }

    async fn run_pod_sandbox(
        &self,
        _request: tonic::Request<cri::RunPodSandboxRequest>,
    ) -> Result<tonic::Response<cri::RunPodSandboxResponse>, tonic::Status> {
        todo!()
    }

    async fn stop_pod_sandbox(
        &self,
        _request: tonic::Request<cri::StopPodSandboxRequest>,
    ) -> Result<tonic::Response<cri::StopPodSandboxResponse>, tonic::Status> {
        todo!()
    }

    async fn remove_pod_sandbox(
        &self,
        _request: tonic::Request<cri::RemovePodSandboxRequest>,
    ) -> Result<tonic::Response<cri::RemovePodSandboxResponse>, tonic::Status> {
        todo!()
    }

    async fn pod_sandbox_status(
        &self,
        _request: tonic::Request<cri::PodSandboxStatusRequest>,
    ) -> Result<tonic::Response<cri::PodSandboxStatusResponse>, tonic::Status> {
        todo!()
    }

    async fn list_pod_sandbox(
        &self,
        _request: tonic::Request<cri::ListPodSandboxRequest>,
    ) -> Result<tonic::Response<cri::ListPodSandboxResponse>, tonic::Status> {
        todo!()
    }

    async fn create_container(
        &self,
        _request: tonic::Request<cri::CreateContainerRequest>,
    ) -> Result<tonic::Response<cri::CreateContainerResponse>, tonic::Status> {
        todo!()
    }

    async fn start_container(
        &self,
        _request: tonic::Request<cri::StartContainerRequest>,
    ) -> Result<tonic::Response<cri::StartContainerResponse>, tonic::Status> {
        todo!()
    }

    async fn stop_container(
        &self,
        _request: tonic::Request<cri::StopContainerRequest>,
    ) -> Result<tonic::Response<cri::StopContainerResponse>, tonic::Status> {
        todo!()
    }

    async fn remove_container(
        &self,
        _request: tonic::Request<cri::RemoveContainerRequest>,
    ) -> Result<tonic::Response<cri::RemoveContainerResponse>, tonic::Status> {
        todo!()
    }

    async fn list_containers(
        &self,
        _request: tonic::Request<cri::ListContainersRequest>,
    ) -> Result<tonic::Response<cri::ListContainersResponse>, tonic::Status> {
        todo!()
    }

    async fn container_status(
        &self,
        _request: tonic::Request<cri::ContainerStatusRequest>,
    ) -> Result<tonic::Response<cri::ContainerStatusResponse>, tonic::Status> {
        todo!()
    }

    async fn update_container_resources(
        &self,
        _request: tonic::Request<cri::UpdateContainerResourcesRequest>,
    ) -> Result<tonic::Response<cri::UpdateContainerResourcesResponse>, tonic::Status> {
        todo!()
    }

    async fn reopen_container_log(
        &self,
        _request: tonic::Request<cri::ReopenContainerLogRequest>,
    ) -> Result<tonic::Response<cri::ReopenContainerLogResponse>, tonic::Status> {
        todo!()
    }

    async fn exec_sync(
        &self,
        _request: tonic::Request<cri::ExecSyncRequest>,
    ) -> Result<tonic::Response<cri::ExecSyncResponse>, tonic::Status> {
        todo!()
    }

    async fn exec(
        &self,
        _request: tonic::Request<cri::ExecRequest>,
    ) -> Result<tonic::Response<cri::ExecResponse>, tonic::Status> {
        todo!()
    }

    async fn attach(
        &self,
        _request: tonic::Request<cri::AttachRequest>,
    ) -> Result<tonic::Response<cri::AttachResponse>, tonic::Status> {
        todo!()
    }

    async fn port_forward(
        &self,
        _request: tonic::Request<cri::PortForwardRequest>,
    ) -> Result<tonic::Response<cri::PortForwardResponse>, tonic::Status> {
        todo!()
    }

    async fn container_stats(
        &self,
        _request: tonic::Request<cri::ContainerStatsRequest>,
    ) -> Result<tonic::Response<cri::ContainerStatsResponse>, tonic::Status> {
        todo!()
    }

    async fn list_container_stats(
        &self,
        _request: tonic::Request<cri::ListContainerStatsRequest>,
    ) -> Result<tonic::Response<cri::ListContainerStatsResponse>, tonic::Status> {
        todo!()
    }

    async fn update_runtime_config(
        &self,
        _request: tonic::Request<cri::UpdateRuntimeConfigRequest>,
    ) -> Result<tonic::Response<cri::UpdateRuntimeConfigResponse>, tonic::Status> {
        todo!()
    }

    async fn status(
        &self,
        _request: tonic::Request<cri::StatusRequest>,
    ) -> Result<tonic::Response<cri::StatusResponse>, tonic::Status> {
        todo!()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;

    Server::builder()
        .add_service(RuntimeServiceServer::new(CRIRuntimeService::default()))
        .serve(addr)
        .await?;

    Ok(())
}
