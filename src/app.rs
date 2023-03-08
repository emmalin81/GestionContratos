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
    let is_ready = use_state(|| false);
    let policies = use_state(Vec::new);

    {
        let is_ready = is_ready.clone();
        let policies = policies.clone();
        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    let secret_key = SecretKey::from_bech32(BECH32_SK).unwrap();
                    let keys = Keys::new(secret_key);
                    let client =
                        CoinstrClient::new(keys, vec![DEFAULT_RELAY.to_string()], Network::Testnet)
                            .await
                            .unwrap();
                    policies.set(client.get_policies(TIMEOUT).await.unwrap());
                    is_ready.set(true);
                });
                || ()
            },
            (),
        );
    }

    html! {
        <main>
            <h1>{ "Coinstr" }</h1>
            {
                if *is_ready {
                    policies.iter().map(|(policy_id, policy)| {
                        html! {
                            <div>
                                <p> { "Policy" } </p>
                                <p> { format!("ID: {policy_id}") } </p>
                                <p> { format!("Name: {}", policy.name.clone()) } </p>
                                <p> { format!("Description: {}", policy.description.clone()) } </p>
                                <p> { format!("Descriptor: {}", policy.descriptor) } </p>
                                <hr />
                            </div>
                        }
                    }).collect::<Html>()
                } else {
                    html! {
                        <p> { "Loading policies..." } </p>
                    }
                }
            }
        </main>
    }
}
