use std::{marker::PhantomData, sync::Arc};

use tower::Layer;

use super::{service::AuthService, controller::AuthController};

pub struct AuthLayer<Protocol, Operation, Config> {
    controller: Arc<AuthController>,
    operation: &'static str,
    config: Config,
    _protocol: PhantomData<Protocol>,
    _operation: PhantomData<Operation>,
}

impl<Protocol, Operation, Config> AuthLayer<Protocol, Operation, Config> {
    pub fn new(operation: &'static str, auth: Arc<AuthController>, config: Config) -> AuthLayer<Protocol, Operation, Config> {
        AuthLayer {
            controller: auth,
            operation,
            config,
            _protocol: PhantomData,
            _operation: PhantomData,
        }
    }
}

impl <Service, Protocol, Operation, Config> Layer<Service> for AuthLayer<Protocol, Operation, Config>
where
    Config: Clone,
{
    type Service = AuthService<Protocol, Operation, Service, Config>;

    fn layer(&self, service: Service) -> Self::Service {
        AuthService::new(
            service,
            self.operation,
            self.controller.clone(),
            self.config.clone(),
        )
    }
}