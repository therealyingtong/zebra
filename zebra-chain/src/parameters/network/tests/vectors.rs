//! Fixed test vectors for the network consensus parameters.

use zcash_primitives::consensus::{self as zp_consensus, Parameters};

use crate::parameters::Network;

#[test]
fn check_parameters_impl() {
    let zp_network_upgrades = [
        zp_consensus::NetworkUpgrade::Overwinter,
        zp_consensus::NetworkUpgrade::Sapling,
        zp_consensus::NetworkUpgrade::Blossom,
        zp_consensus::NetworkUpgrade::Heartwood,
        zp_consensus::NetworkUpgrade::Canopy,
        zp_consensus::NetworkUpgrade::Nu5,
    ];

    for (network, zp_network) in [
        (Network::Mainnet, zp_consensus::Network::MainNetwork),
        (
            Network::new_default_testnet(),
            zp_consensus::Network::TestNetwork,
        ),
    ] {
        for nu in zp_network_upgrades {
            let activation_height = network
                .activation_height(nu)
                .expect("must have activation height for past network upgrades");

            assert_eq!(
                activation_height,
                zp_network
                    .activation_height(nu)
                    .expect("must have activation height for past network upgrades"),
                "Parameters::activation_heights() outputs must match"
            );

            let activation_height: u32 = activation_height.into();

            for height in (activation_height - 1)..=(activation_height + 1) {
                for nu in zp_network_upgrades {
                    let height = zp_consensus::BlockHeight::from_u32(height);
                    assert_eq!(
                        network.is_nu_active(nu, height),
                        zp_network.is_nu_active(nu, height),
                        "Parameters::is_nu_active() outputs must match",
                    );
                }
            }
        }

        assert_eq!(
            network.coin_type(),
            zp_network.coin_type(),
            "Parameters::coin_type() outputs must match"
        );
        assert_eq!(
            network.hrp_sapling_extended_spending_key(),
            zp_network.hrp_sapling_extended_spending_key(),
            "Parameters::hrp_sapling_extended_spending_key() outputs must match"
        );
        assert_eq!(
            network.hrp_sapling_extended_full_viewing_key(),
            zp_network.hrp_sapling_extended_full_viewing_key(),
            "Parameters::hrp_sapling_extended_full_viewing_key() outputs must match"
        );
        assert_eq!(
            network.hrp_sapling_payment_address(),
            zp_network.hrp_sapling_payment_address(),
            "Parameters::hrp_sapling_payment_address() outputs must match"
        );
        assert_eq!(
            network.b58_pubkey_address_prefix(),
            zp_network.b58_pubkey_address_prefix(),
            "Parameters::b58_pubkey_address_prefix() outputs must match"
        );
        assert_eq!(
            network.b58_script_address_prefix(),
            zp_network.b58_script_address_prefix(),
            "Parameters::b58_script_address_prefix() outputs must match"
        );
    }
}
