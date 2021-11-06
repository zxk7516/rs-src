use tonic::{
    transport::Server,
    Request,
    Response,
    Status,
};

use shortlink::short_link_server::{
    ShortLink,
    ShortLinkServer,
};
use shortlink::{
    ShortLinkRequest,
    ShortLinkResponse,
};

pub mod shortlink {
    tonic::include_proto!("shortlink"); // The string specified here must match the proto package name
}

#[derive(Debug, Default)]
pub struct MyShortLink {}

#[tonic::async_trait]
impl ShortLink for MyShortLink {
    async fn get_info (
        &self,
        request: Request<ShortLinkRequest>,
    ) -> Result<Response<ShortLinkResponse>, Status> {
        println!("Got a request: {:?}", request);
        let reply = shortlink::ShortLinkResponse {
            url: format!("Hello {}!", request.into_inner().id).into(),
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let short_linker = MyShortLink::default();

    Server::builder()
        .add_service(ShortLinkServer::new(short_linker))
        .serve(addr)
        .await?;

    Ok(())
}
