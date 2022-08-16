use std::{future::{Ready, ready}, time::Duration};

use actix_web::{dev::{Transform, ServiceRequest, Service, ServiceResponse, forward_ready}, body::MessageBody, Error};
use futures::future::LocalBoxFuture;

pub struct DevDelay(Duration);

impl DevDelay {
    pub fn new(duration: Duration) -> Self{
        Self(duration)
    }
}

impl<S, B> Transform<S, ServiceRequest> for DevDelay
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<B>;

    type Error = Error;

    type Transform = DelayMiddleware<S>;  

    type InitError = ();

    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(DelayMiddleware{ service, duration: self.0 }))
    }
}

pub struct DelayMiddleware<S> {
    service: S,
    duration: Duration,
}

impl<S, B> Service<ServiceRequest> for DelayMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static
{
    type Response = ServiceResponse<B>;

    type Error = Error;

    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let fut = self.service.call(req);
        let duration = self.duration.clone();
        Box::pin(async move {
            let res = fut.await?;
            actix::clock::sleep(duration).await;
            Ok(res)
        })
    }
}
