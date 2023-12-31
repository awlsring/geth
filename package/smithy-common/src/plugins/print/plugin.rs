use aws_smithy_http_server::{
    operation::{OperationShape},
    plugin::{Plugin, PluginPipeline, PluginStack},
    shape_id::ShapeId,
};
use log::info;
use tower::{Layer, Service};

use std::task::{Context, Poll};

/// A [`Service`] that prints a given string.
#[derive(Clone, Debug)]
pub struct PrintService<S> {
    inner: S,
    id: ShapeId,
}

impl<R, S> Service<R> for PrintService<S>
where
    S: Service<R>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: R) -> Self::Future {
        info!("Operation Invoked: {}", self.id.absolute());
        self.inner.call(req)
    }
}

/// A [`Layer`] which constructs the [`PrintService`].
#[derive(Debug)]
pub struct PrintLayer {
    id: ShapeId,
}
impl<S> Layer<S> for PrintLayer {
    type Service = PrintService<S>;

    fn layer(&self, service: S) -> Self::Service {
        PrintService {
            inner: service,
            id: self.id.clone(),
        }
    }
}

/// A [`Plugin`] for a service builder to add a [`PrintLayer`] over operations.
#[derive(Debug)]
pub struct PrintPlugin;

impl<P, Op, S> Plugin<P, Op, S> for PrintPlugin
where
    Op: OperationShape,
{
    type Service = PrintService<S>;

    fn apply(&self, inner: S) -> Self::Service {
        PrintService { inner, id: Op::ID }
    }
}

/// This provides a [`print`](PrintExt::print) method on [`PluginPipeline`].
pub trait PrintExt<ExistingPlugins> {
    /// Causes all operations to print the operation name when called.
    ///
    /// This works by applying the [`PrintPlugin`].
    fn print(self) -> PluginPipeline<PluginStack<PrintPlugin, ExistingPlugins>>;
}

impl<ExistingPlugins> PrintExt<ExistingPlugins> for PluginPipeline<ExistingPlugins> {
    fn print(self) -> PluginPipeline<PluginStack<PrintPlugin, ExistingPlugins>> {
        self.push(PrintPlugin)
    }
}
