use super::*;
use crate::{types::BtcPublicKey, Pallet as VaultRegistry};
use exchange_rate_oracle::Pallet as ExchangeRateOracle;
use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite};
use frame_support::traits::Currency;
use frame_system::RawOrigin;
use sp_std::prelude::*;

fn dummy_public_key() -> BtcPublicKey {
    BtcPublicKey([
        2, 205, 114, 218, 156, 16, 235, 172, 106, 37, 18, 153, 202, 140, 176, 91, 207, 51, 187, 55, 18, 45, 222, 180,
        119, 54, 243, 97, 173, 150, 161, 169, 230,
    ])
}

fn make_free_balance_be<T: currency::Config<currency::Collateral>>(account_id: &T::AccountId, amount: Collateral<T>) {
    // use crate::mock::sp_api_hidden_includes_construct_runtime::hidden_include::traits::Currency;
    <<T as currency::Config<currency::Collateral>>::Currency>::make_free_balance_be(account_id, amount);
}

benchmarks! {
    register_vault {
        let origin: T::AccountId = account("Origin", 0, 0);
        make_free_balance_be::<T>(&origin, (1u32 << 31).into());
        let amount: u32 = 100;
        let public_key = BtcPublicKey::default();
    }: _(RawOrigin::Signed(origin.clone()), amount.into(), public_key)
    verify {
        // assert_eq!(Vaults::<T>::get(origin).wallet.get_btc_address(), btc_address);
    }

    deposit_collateral {
        let origin: T::AccountId = account("Origin", 0, 0);
        let u in 0 .. 100;
        make_free_balance_be::<T>(&origin, (1u32 << 31).into());
        VaultRegistry::<T>::_register_vault(&origin, 1234u32.into(), dummy_public_key()).unwrap();
    }: _(RawOrigin::Signed(origin), u.into())
    verify {
    }

    withdraw_collateral {
        let origin: T::AccountId = account("Origin", 0, 0);
        let u in 0 .. 100;
        make_free_balance_be::<T>(&origin, (1u32 << 31).into());
        VaultRegistry::<T>::_register_vault(&origin, u.into(), dummy_public_key()).unwrap();
    }: _(RawOrigin::Signed(origin), u.into())
    verify {
    }

    update_public_key {
        let origin: T::AccountId = account("Origin", 0, 0);
        make_free_balance_be::<T>(&origin, (1u32 << 31).into());
        VaultRegistry::<T>::_register_vault(&origin, 1234u32.into(), dummy_public_key()).unwrap();
    }: _(RawOrigin::Signed(origin), BtcPublicKey::default())

    register_address {
        let origin: T::AccountId = account("Origin", 0, 0);
        make_free_balance_be::<T>(&origin, (1u32 << 31).into());
        VaultRegistry::<T>::_register_vault(&origin, 1234u32.into(), dummy_public_key()).unwrap();
    }: _(RawOrigin::Signed(origin), BtcAddress::default())

    accept_new_issues {
        let origin: T::AccountId = account("Origin", 0, 0);
        make_free_balance_be::<T>(&origin, (1u32 << 31).into());
        VaultRegistry::<T>::_register_vault(&origin, 1234u32.into(), dummy_public_key()).unwrap();
    }: _(RawOrigin::Signed(origin), true)

    report_undercollateralized_vault {
        let origin: T::AccountId = account("Origin", 0, 0);
        let vault_id: T::AccountId = account("Vault", 0, 0);
        make_free_balance_be::<T>(&vault_id, (1u32 << 31).into());

        VaultRegistry::<T>::_register_vault(&vault_id, 10_000u32.into(), dummy_public_key()).unwrap();
        ExchangeRateOracle::<T>::_set_exchange_rate(<T as exchange_rate_oracle::Config>::UnsignedFixedPoint::one()).unwrap();

        VaultRegistry::<T>::try_increase_to_be_issued_tokens(&vault_id, 5_000u32.into()).unwrap();
        VaultRegistry::<T>::issue_tokens(&vault_id, 5_000u32.into()).unwrap();

        ExchangeRateOracle::<T>::_set_exchange_rate(<T as exchange_rate_oracle::Config>::UnsignedFixedPoint::checked_from_rational(10, 1).unwrap()).unwrap();
    }: _(RawOrigin::Signed(origin), vault_id)
}

impl_benchmark_test_suite!(
    VaultRegistry,
    crate::mock::ExtBuilder::build_with(Default::default()),
    crate::mock::Test
);
