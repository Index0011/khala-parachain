use crate::*;
use frame_support::{
	traits::{Currency, Get, StorageVersion},
	weights::Weight,
};
use log;

type MiningBalanceOf<T> =
	<<T as mining::Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

type Versions = (
	StorageVersion,
	StorageVersion,
	StorageVersion,
	StorageVersion,
);

fn get_versions<T>() -> Versions
where
	T: mining::Config + mq::Config + registry::Config + stakepool::Config,
{
	(
		StorageVersion::get::<mining::Pallet<T>>(),
		StorageVersion::get::<mq::Pallet<T>>(),
		StorageVersion::get::<registry::Pallet<T>>(),
		StorageVersion::get::<stakepool::Pallet<T>>(),
	)
}

/// Migrations to v4
///
/// From v4, all the storage version will become 4 at once. Before v4, the latest storage versions
/// are:
/// - fat: 1
/// - mining: 3
/// - mq: 0
/// - ott: 1
/// - registry: 1
/// - stakepool: 1
pub mod v4 {
	use super::*;

	const EXPECTED_STORAGE_VERSION: Versions = (
		StorageVersion::new(3),
		StorageVersion::new(0),
		StorageVersion::new(1),
		StorageVersion::new(1),
	);

	#[cfg(feature = "try-runtime")]
	const FINAL_STORAGE_VERSION: Versions = (
		StorageVersion::new(4),
		StorageVersion::new(4),
		StorageVersion::new(4),
		StorageVersion::new(4),
	);

	#[cfg(feature = "try-runtime")]
	pub fn pre_migrate<T>() -> Result<(), &'static str>
	where
		T: mining::Config + mq::Config + registry::Config + stakepool::Config,
	{
		frame_support::ensure!(
			get_versions::<T>() == EXPECTED_STORAGE_VERSION,
			"incorrect pallet versions"
		);
		Ok(())
	}

	pub fn migrate<T>() -> Weight
	where
		T: mining::Config + mq::Config + registry::Config + stakepool::Config,
		MiningBalanceOf<T>: balance_convert::FixedPointConvert,
	{
		if get_versions::<T>() == EXPECTED_STORAGE_VERSION {
			let mut weight: Weight = 0;
			log::info!("Ᵽ migrating phala-pallets to v4");
			weight += mining::migrations::signal_phala_launch::<T>();
			weight += mining::migrations::enable_phala_tokenomic::<T>();
			log::info!("Ᵽ pallets migrated to v4");

			StorageVersion::new(4).put::<mining::Pallet<T>>();
			StorageVersion::new(4).put::<mq::Pallet<T>>();
			StorageVersion::new(4).put::<registry::Pallet<T>>();
			StorageVersion::new(4).put::<stakepool::Pallet<T>>();
			weight += T::DbWeight::get().writes(5);
			weight
		} else {
			T::DbWeight::get().reads(1)
		}
	}

	#[cfg(feature = "try-runtime")]
	pub fn post_migrate<T>() -> Result<(), &'static str>
	where
		T: mining::Config + mq::Config + registry::Config + stakepool::Config,
	{
		frame_support::ensure!(
			get_versions::<T>() == FINAL_STORAGE_VERSION,
			"incorrect pallet versions postmigrate"
		);
		log::info!(
			"Ᵽ phala mining start time is reset to {:?}",
			mining::MiningStartBlock::<T>::get()
		);
		log::info!("Ᵽ phala pallet migration passes POST migrate checks ✅",);
		Ok(())
	}
}

pub mod v5 {
	use super::*;

	const EXPECTED_STORAGE_VERSION: Versions = (
		StorageVersion::new(4),
		StorageVersion::new(4),
		StorageVersion::new(4),
		StorageVersion::new(4),
	);

	#[cfg(feature = "try-runtime")]
	const FINAL_STORAGE_VERSION: Versions = (
		StorageVersion::new(5),
		StorageVersion::new(5),
		StorageVersion::new(5),
		StorageVersion::new(5),
	);

	#[cfg(feature = "try-runtime")]
	pub fn pre_migrate<T>() -> Result<(), &'static str>
	where
		T: fat::Config + mining::Config + mq::Config + registry::Config + stakepool::Config,
	{
		frame_support::ensure!(
			get_versions::<T>() == EXPECTED_STORAGE_VERSION,
			"incorrect pallet versions"
		);
		Ok(())
	}

	pub fn migrate<T>() -> Weight
	where
		T: mining::Config + mq::Config + registry::Config + stakepool::Config,
		MiningBalanceOf<T>: balance_convert::FixedPointConvert + sp_std::fmt::Display,
		T: mining::pallet::Config<Currency = <T as stakepool::pallet::Config>::Currency>,
	{
		if get_versions::<T>() == EXPECTED_STORAGE_VERSION {
			let mut weight: Weight = 0;
			log::info!("Ᵽ migrating phala-pallets to v5");
			weight += mining::migrations::trigger_unresp_fix::<T>();
			weight += stakepool::Pallet::<T>::migration_remove_assignments();
			log::info!("Ᵽ pallets migrated to v5");

			StorageVersion::new(5).put::<mining::Pallet<T>>();
			StorageVersion::new(5).put::<mq::Pallet<T>>();
			StorageVersion::new(5).put::<registry::Pallet<T>>();
			StorageVersion::new(5).put::<stakepool::Pallet<T>>();
			weight += T::DbWeight::get().reads_writes(5, 5);
			weight
		} else {
			T::DbWeight::get().reads(5)
		}
	}

	#[cfg(feature = "try-runtime")]
	pub fn post_migrate<T>() -> Result<(), &'static str>
	where
		T: mining::Config + mq::Config + registry::Config + stakepool::Config,
	{
		frame_support::ensure!(
			get_versions::<T>() == FINAL_STORAGE_VERSION,
			"incorrect pallet versions postmigrate"
		);
		log::info!("Ᵽ phala pallet migration passes POST migrate checks ✅",);
		Ok(())
	}
}
