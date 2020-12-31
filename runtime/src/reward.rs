
#![cfg_attr(not(feature = "std"), no_std)]
use frame_support::{debug ,decl_module, dispatch::{DispatchResult,Vec}, weights::{Weight},decl_storage, decl_event, decl_error, dispatch,
traits::{Get, Currency, OriginTrait}};

use frame_system::{ensure_signed, ensure_root};
use super::Aura;
use sp_core::{
	crypto::{Public as _},
	H256,
	sr25519::{Public, Signature},
};
use codec::{Encode, Decode};

type BalanceOf<T> = <<T as Trait>::Currency as Currency<<T as frame_system::Trait>::AccountId>>::Balance;// use super::{block_author::BlockAuthor, issuance::Issuance};

// #[cfg(test)]
// mod mock;

// #[cfg(test)]
// mod tests;

pub trait Trait: frame_system::Trait {
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
	type Currency: Currency<Self::AccountId>;
	type RewardAmount: Get<BalanceOf<Self>>;
}

decl_storage! {
	trait Store for Module<T: Trait> as TemplateModule {
		Something get(fn something) config(): BalanceOf<T>;

	}
}

decl_event!(
	pub enum Event<T>
	where
		AccountId = <T as frame_system::Trait>::AccountId,
		Balance = BalanceOf<T>,
	{
		SomethingStored( AccountId),
		RewardsIssued(AccountId, Balance),
	}
);

decl_error! {
	pub enum Error for Module<T: Trait> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}
}
decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Errors must be initialized if they are used by the pallet.
		// type Error = Error<T>;

		// Events must be initialized if they are used by the pallet.
		fn deposit_event() = default;

		fn on_finalize(block: T::BlockNumber) {
		    debug::info!("===================================================");
			debug::info!("Current Block From Reward Runtime :# {:?}",  block);
			debug::info!("===================================================");
			debug::info!("Aura Authorities array :  {:?}", Aura::authorities().iter());
			debug::info!("===================================================");

			let auth:Vec<H256> = Aura::authorities().iter().map(|x| {
                let r: &Public = x.as_ref();
                r.0.into()
			}).collect();

			Self::disperse_reward(&auth);

		}

	}
}

impl<T: Trait> Module<T> {
	/// Redistribute combined reward value to block Author
	fn disperse_reward(to_reward: &[H256]) {
		debug::info!("My Authorities Inside the desperss method: {:?}", to_reward);

		let reward = T::RewardAmount::get();

		debug::info!("My reward on finalize after upgrading the storage and increment: {:?}", reward);
		for who in to_reward {
			// Assume each H256 correctly encodes an AccountId
			let bytes = who.encode();
			// Force the H256 to the AccountId type, if that decoding fails, use default AccountId instead.
			let account = T::AccountId::decode(&mut &bytes[..]).unwrap_or_default();
			// Ignore any errors from this call, not much we can do about that.
			match T::Currency::deposit_into_existing(&account, reward) {
				Ok(_) => Self::deposit_event(RawEvent::RewardsIssued(account, reward)),
				_ => {}
			};
		}

	}
}
