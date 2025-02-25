#[cfg(client)]
mod freeze; // This has `FrozenApp` etc.
mod global_state;
mod rx_result;
mod rx_state;
mod state_generator_info;
mod state_store;
#[cfg(client)]
mod suspense;
mod template_state;
// #[cfg(feature = "rx-collections")]
pub mod rx_collections;

#[cfg(client)]
pub use freeze::{FrozenApp, PageThawPrefs, ThawPrefs};
#[cfg(client)]
pub(crate) use global_state::FrozenGlobalState;
pub use global_state::{GlobalState, GlobalStateCreator, GlobalStateType};
pub use rx_result::{RxResult, RxResultRx, SerdeInfallible};
pub use rx_state::{AnyFreeze, Freeze, MakeRx, MakeUnrx, UnreactiveState};
pub use state_generator_info::{BuildPaths, StateGeneratorInfo};
pub use state_store::{PageStateStore, PssContains, PssEntry, PssState};
pub use template_state::{TemplateState, TemplateStateWithType, UnknownStateType};

#[cfg(all(feature = "idb-freezing", client))]
mod freeze_idb;
#[cfg(all(feature = "idb-freezing", client))]
pub use freeze_idb::{IdbError, IdbFrozenStateStore};

// We'll allow live reloading (of which HSR is a subset) if it's feature-enabled
// and we're in development mode
#[cfg(all(feature = "live-reload", debug_assertions, client))]
mod live_reload;
#[cfg(all(feature = "live-reload", debug_assertions, client))]
pub(crate) use live_reload::connect_to_reload_server;
#[cfg(all(feature = "live-reload", debug_assertions, client))]
pub(crate) use live_reload::force_reload;
#[cfg(client)]
pub use suspense::{compute_nested_suspense, compute_suspense};
