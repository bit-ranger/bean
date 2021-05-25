pub use std::sync::Arc;

pub trait HasComponent<C> {

    fn put(&mut self, name: &str, component: Arc<C>) -> &mut Self;

    fn get(&self, name: &str) -> Option<Arc<C>>;

    fn get_all(&self) -> Vec<(&str, Arc<C>)>;
}