mod generated;

pub use generated::*;
pub use generated::{
    image_service_client::ImageServiceClient, runtime_service_client::RuntimeServiceClient,
};

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn image_service_client() {
        let mut client = crate::ImageServiceClient::connect("http://[::1]:50048")
            .await
            .expect("failed to connect to mock image service");

        let response = client
            .list_images(crate::ListImagesRequest { filter: None })
            .await
            .expect("ListImagesRequest failed");

        println!("{:?}", response);
    }

    #[tokio::test]
    async fn runtime_service_client() {
        let mut client = crate::RuntimeServiceClient::connect("http://[::1]:50049")
            .await
            .expect("failed to connect to mock runtime service");

        let response = client
            .version(crate::VersionRequest {
                version: String::from("test"),
            })
            .await
            .expect("ListImagesRequest failed");

        println!("{:?}", response);
    }
}
