#[macro_use]
extern crate log;

use gubernator_cri::image_service_server::{ImageService, ImageServiceServer};
use tonic::transport::Server;

// use crate::generated::image_service_server::{ImageService, ImageServiceServer};
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct MockImageService;

#[tonic::async_trait]
impl ImageService for MockImageService {
    async fn list_images(
        &self,
        _request: tonic::Request<gubernator_cri::ListImagesRequest>,
    ) -> Result<tonic::Response<gubernator_cri::ListImagesResponse>, tonic::Status> {
        info!("received ListImagesRequest");
        Ok(tonic::Response::new(gubernator_cri::ListImagesResponse {
            images: Vec::new(),
        }))
    }

    async fn image_status(
        &self,
        _request: tonic::Request<gubernator_cri::ImageStatusRequest>,
    ) -> Result<tonic::Response<gubernator_cri::ImageStatusResponse>, tonic::Status> {
        info!("received ImageStatusRequest");
        Ok(tonic::Response::new(gubernator_cri::ImageStatusResponse {
            image: None,
            info: HashMap::new(),
        }))
    }

    async fn pull_image(
        &self,
        _request: tonic::Request<gubernator_cri::PullImageRequest>,
    ) -> Result<tonic::Response<gubernator_cri::PullImageResponse>, tonic::Status> {
        todo!()
    }

    async fn remove_image(
        &self,
        _request: tonic::Request<gubernator_cri::RemoveImageRequest>,
    ) -> Result<tonic::Response<gubernator_cri::RemoveImageResponse>, tonic::Status> {
        todo!()
    }

    async fn image_fs_info(
        &self,
        _request: tonic::Request<gubernator_cri::ImageFsInfoRequest>,
    ) -> Result<tonic::Response<gubernator_cri::ImageFsInfoResponse>, tonic::Status> {
        todo!()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    info!("starting MockImageService on port 548");

    let address = "0.0.0.0:548".parse().expect("failed to parse address");
    Server::builder()
        .add_service(ImageServiceServer::new(MockImageService::default()))
        .serve(address)
        .await?;

    Ok(())
}
