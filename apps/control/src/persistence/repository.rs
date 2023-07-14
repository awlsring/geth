use std::sync::Arc;
use async_trait::async_trait;

#[async_trait]
pub trait Repository<Type, Id> {
    async fn find_by_id(&self, id: Id) -> Option<Type>;
    async fn find_all(&self) -> Arc<[Type]>;
    async fn modify(&mut self, item: Type) -> Result<(), String>;
    async fn insert(&mut self, item: Type) -> Result<(), String>;
    async fn delete(&mut self, id: Id) -> Result<(), String>;
}