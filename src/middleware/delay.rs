use actix_web::{dev::{Transform, ServiceRequest, Service, ServiceResponse}, body::{MessageBody, EitherBody}, http::Error};
use futures::Future;

use crate::errors::MyError;


// pub struct DevDelay;

// impl<S, B> Transform<S, ServiceRequest> for DevDelay
// where
//     S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
//     S::Future: 'static,
//     B: MessageBody + 'static,
// {
//     type Response = ServiceResponse<EitherBody<B>>;

//     type Error = MyError;

//     type Transform = ;  

//     type InitError = ();

//     type Future = Ready<Result<Self::Transform, Self::InitError>>;

//     fn new_transform(&self, service: S) -> Self::Future {
//         todo!()
//     }
// }