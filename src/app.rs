use std::time::Duration;

use coinstr_core::bitcoin::Network;
use coinstr_core::nostr_sdk::prelude::*;
use coinstr_core::CoinstrClient;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

const BECH32_SK: &str = "nsec18lkp320pjm7n5eqhk3066uq9akermpff