use crate::manifest::yaml::Network;
use crate::types::code::Code;

fn network_provider_name(network: &Network) -> String {
    network_provider_name_from_name(&network.name)
}

fn network_provider_name_from_name(network_name: &str) -> String {
    format!(
        "{network_name}_PROVIDER",
        network_name = network_name.to_uppercase()
    )
}

pub fn network_provider_fn_name(network: &Network) -> String {
    format!(
        "get_{fn_name}",
        fn_name = network_provider_name(network).to_lowercase()
    )
}

fn generate_network_lazy_provider_code(network: &Network) -> Code {
    Code::new(format!(
        r#"
            static ref {network_name}: Arc<JsonRpcCachedProvider> = create_client(&public_read_env_value("{network_url}").unwrap_or("{network_url}".to_string()), {compute_units_per_second}).expect("Error creating provider");
        "#,
        network_name = network_provider_name(network),
        network_url = network.rpc,
        compute_units_per_second =
            if let Some(compute_units_per_second) = network.compute_units_per_second {
                format!("Some({})", compute_units_per_second)
            } else {
                "None".to_string()
            }
    ))
}

fn generate_network_provider_code(network: &Network) -> Code {
    Code::new(format!(
        r#"
            pub fn {fn_name}_cache() -> Arc<JsonRpcCachedProvider> {{
                {provider_lazy_name}.clone()
            }}
            
            pub fn {fn_name}() -> Arc<Provider<RetryClient<Http>>> {{
                {provider_lazy_name}.get_inner_provider()
            }}
        "#,
        fn_name = network_provider_fn_name(network),
        provider_lazy_name = network_provider_name(network)
    ))
}

fn generate_provider_cache_for_network_fn(networks: &[Network]) -> Code {
    let mut if_code = Code::blank();
    for network in networks {
        let network_if = format!(
            r#"
            if network == "{network_name}" {{
                return get_{network_name}_provider_cache();
            }}
        "#,
            network_name = network.name
        );

        if_code.push_str(&Code::new(network_if));
    }

    if_code.push_str(&Code::new(r#"panic!("Network not supported")"#.to_string()));

    let provider_cache_for_network_fn = format!(
        r#"
        pub fn get_provider_cache_for_network(network: &str) -> Arc<JsonRpcCachedProvider>  {{
            {if_code}
        }}
    "#
    );

    Code::new(provider_cache_for_network_fn)
}

pub fn generate_networks_code(networks: &[Network]) -> Code {
    let mut output = Code::new(r#"
            /// THIS IS A GENERATED FILE. DO NOT MODIFY MANUALLY.
            ///
            /// This file was auto generated by rindexer - https://github.com/joshstevens19/rindexer.
            /// Any manual changes to this file will be overwritten.
            
            use ethers::providers::{Provider, Http, RetryClient};
            use rindexer::{
                lazy_static,
                provider::{create_client, JsonRpcCachedProvider},
                public_read_env_value,
            };
            use std::sync::Arc;

            lazy_static! {
        "#
    .to_string());

    for network in networks {
        output.push_str(&generate_network_lazy_provider_code(network));
    }

    output.push_str(&Code::new("}".to_string()));

    for network in networks {
        output.push_str(&generate_network_provider_code(network));
    }

    output.push_str(&generate_provider_cache_for_network_fn(networks));

    output
}