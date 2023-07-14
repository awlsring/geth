use std::sync::Arc;

use crate::{
    model::machine::Machine,
    persistence::{machine_repo::MachinePrismaRepository, repository::Repository},
    service::agent::AgentService,
};

pub struct AgentController {
    service: AgentService,
    repo: MachinePrismaRepository,
}

impl AgentController {
    pub fn new(service: AgentService, repo: MachinePrismaRepository) -> AgentController {
        AgentController { service, repo }
    }

    pub async fn register_machine(
        &mut self,
        endpoint: &str,
        group: &str,
    ) -> Result<Machine, String> {
        let overview = self.service.get_server_overview(endpoint).await;

        match overview {
            Ok(o) => match o.summary() {
                Some(s) => {
                    let machine = Machine::new_from_agent_overview(s, group);
                    let insert_result = self.repo.insert(machine.clone()).await;
                    match insert_result {
                        Ok(_) => Ok(machine),
                        Err(e) => Err(e),
                    }
                }
                None => Err("No summary found".to_string()),
            },
            Err(e) => Err(e.to_string()),
        }
    }

    pub async fn get_machine(&mut self, machine_id: &str) -> Result<Machine, String> {
        let machine = self.repo.find_by_id(machine_id.to_string()).await;
        match machine {
            Some(m) => Ok(m),
            None => Err("Machine not found".to_string()),
        }
    }

    pub async fn list_machines(&mut self) -> Arc<[Machine]> {
        self.repo.find_all().await
    }

    pub async fn remove_machine(&mut self, machine_id: &str) -> Result<(), String> {
        let delete_result = self.repo.delete(machine_id.to_string()).await;
        match delete_result {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}
