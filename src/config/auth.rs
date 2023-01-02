use actix_web::dev::ServiceRequest;
use actix_web_httpauth::extractors::{bearer::BearerAuth};
use actix_web::Error;


pub async fn bearer_token_validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {

    // eprintln!("{:?}", credentials);
    //
    // if credentials == "test" {
    //     Ok(req)
    // }
    // else {
    //     Ok(req)
    // }
    eprintln!("{:?}", credentials);
    Ok(req)
}
