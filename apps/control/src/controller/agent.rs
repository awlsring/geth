use std::{str::FromStr, collections::HashMap};

use aws_smithy_client::{erase::{DynConnector, DynMiddleware}, SdkError};
use aws_smithy_http::operation::Request;
use geth_agent_client::{Builder, Client, Config, config::AuthApiKey, operation::{get_overview::{GetOverviewOutput, GetOverviewError}, stream_container_logs::{StreamContainerLogsOutput, StreamContainerLogsError}}};
use http::{uri::{Authority, Scheme}, Uri};

pub struct AgentController {
    pub agents: HashMap<String, Client<DynConnector, DynMiddleware<DynConnector>>>
}

impl AgentController {
    pub fn new() -> AgentController {
        let agents = HashMap::new();

        AgentController {
            agents
        }
    }

    pub async fn get_server_overview(&mut self, agent_id: &str) -> Result<GetOverviewOutput, SdkError<GetOverviewError>> {
        let c = self.get_agent_client(agent_id);
        c.get_overview().send().await
    }

    pub async fn get_container_logs(&mut self, agent_id: &str, container_id: &str) -> Result<StreamContainerLogsOutput, SdkError<StreamContainerLogsError>> {
        let c = self.get_agent_client(agent_id);
        c.stream_container_logs()
            .id(container_id)
            .follow(true)
            .send().await
    }

    fn cache_agent_client(&mut self, id: String, client: Client<DynConnector, DynMiddleware<DynConnector>>) {
        self.agents.insert(id, client);
    }

    fn get_agent_client(&mut self, agent_id: &str) -> &Client<DynConnector, DynMiddleware<DynConnector>> {
        let ip = "localhost".to_string(); //tmp, have look up
        let port = "7032".to_string(); //tmp have look up
        self.agents.entry(agent_id.to_string()).or_insert(make_agent_client(ip, port))
    }

}

fn make_agent_client(ip: String, port: String) -> Client<DynConnector, DynMiddleware<DynConnector>> {
    let base_url = format!("{}:{}", ip, port).to_owned();
    let authority = Authority::from_str(&base_url).expect("failed to parse authority");
    let raw_client = Builder::new()
        .rustls_connector(Default::default())
        .middleware_fn(rewrite_base_url(Scheme::HTTP, authority))
        .build_dyn();
    let config = Config::builder()
        .api_key(AuthApiKey::from("toes"))
        .build();
    Client::with_config(raw_client, config)
}

pub fn rewrite_base_url(
    scheme: Scheme,
    authority: Authority,
) -> impl Fn(Request) -> Request + Clone {
    move |mut req| {
        let http_req = req.http_mut();
        let mut uri_parts = http_req.uri().clone().into_parts();
        uri_parts.authority = Some(authority.clone());
        uri_parts.scheme = Some(scheme.clone());
        *http_req.uri_mut() = Uri::from_parts(uri_parts).expect("failed to create uri from parts");
        req
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn connectivity_test() {
        let mut controller = AgentController::new();
        let r = controller.get_server_overview("a").await;
        
        match r {
            Ok(s) => {
                println!("Success: {:?}", s);
            },
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
    }

    #[tokio::test]
    async fn follow_server_logs() {
        let mut controller = AgentController::new();
        let response = controller.get_container_logs("a", "da7f4533c5c38b95ed80c3923536b77667d17bb332a49d971424aa6888754986").await;

        match response {
            Ok(mut s) => {
                loop {
                    match s.logs.recv().await {
                        Ok(logs) => {
                            match logs {
                                Some(l) => {
                                    println!("Log: {:?}", l);
                                },
                                None => {
                                    println!("No logs");
                                    break;
                                }
                            }
                        },
                        Err(e) => {
                            println!("Error: {}", e);
                        }
                    }
                }
            },
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }

}