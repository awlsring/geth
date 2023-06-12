/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Provides an example [`Plugin`] implementation - [`PrintPlugin`].

use std::sync::Arc;

use aws_smithy_http_server::{
    operation::{Operation, OperationShape},
    plugin::{Plugin, PluginPipeline, PluginStack},
};
use tower::{layer::util::Stack};


use super::{layer::AuthLayer, controller::AuthController};

#[derive(Clone, Debug)]
pub struct AuthPlugin<Config> {
    controller: Arc<AuthController>,
    config: Config,
}

impl <Config> AuthPlugin<Config> {
    pub fn new(controller: Arc<AuthController>, config: Config) -> Self {
        AuthPlugin { controller, config }
    }
}

impl <Protocol, Op, Service, Layer, Config> Plugin<Protocol, Op, Service, Layer> for AuthPlugin<Config>
where
Op: OperationShape,
    Config: Clone,
{
    type Service = Service;
    type Layer = Stack<Layer, AuthLayer<Protocol, Op, Config>>;

    fn map(&self, input: Operation<Service, Layer>) -> Operation<Self::Service, Self::Layer> {
        let auth_layer = AuthLayer::new(Op::NAME.name(), self.controller.clone(), self.config.clone());
        input.layer(auth_layer)
    }
}

pub trait AuthExtension<ExistingPlugins> {
    fn auth<Config>(self, controller: Arc<AuthController>, config: Config) -> PluginPipeline<PluginStack<AuthPlugin<Config>, ExistingPlugins>>;
}

impl <ExistingPlugins> AuthExtension<ExistingPlugins> for PluginPipeline<ExistingPlugins> {
    fn auth<Config>(self, controller: Arc<AuthController>, config: Config) -> PluginPipeline<PluginStack<AuthPlugin<Config>, ExistingPlugins>> {
        self.push(AuthPlugin::new(controller, config))
    }
}