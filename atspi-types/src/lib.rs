pub mod events;
pub use events::Event;
pub mod errors;
pub mod interfaces;
pub mod traits;
pub use interfaces::InterfaceSet;
pub mod state;
pub use errors::AtspiError as Error;
pub use state::StateSet;
pub mod accessible;
pub use accessible::Accessible;
pub mod cache;
pub use cache::CacheItem;
pub mod role;
pub use role::Role;

pub type ObjectPair = (String, zvariant::OwnedObjectPath);
