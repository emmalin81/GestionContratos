use std::time::Duration;

use coinstr_core::bitcoin::Network;
use coinstr_core::nostr_sdk::prelude::*;
use coinstr_core::CoinstrClient;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

const BECH32_SK: &str = "nsec18lkp320pjm7n5eqhk3066uq9akermpffedqa3trn3n7a054h2ems37v3ar";
const DEFAULT_RELAY: &str = "wss://relay.rip";
const TIMEOUT: Option<Duration> = Some(Duration::from_secs(300));

#[function_component(App)]
pub fn app() -> Html {
    let is_ready = use_state(|| fa