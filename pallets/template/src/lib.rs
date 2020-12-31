
#![cfg_attr(not(feature = "std"), no_std)]
use frame_support::{debug ,decl_module, dispatch::{DispatchResult,Vec}, weights::{Weight},decl_storage, decl_event, decl_error, dispatch, 
traits::{Get, Currency, OriginTrait}};

use frame_system::{ensure_signed};
// use super::Aura;
use sp_core::{
	crypto::{Public as _},
	H256,
	H512,
	sr25519::{Public, Signature},
};
// #[cfg(test)]
// mod mock;

// #[cfg(test)]
// mod tests;
// pub type BalanceOf<T> = <<T as frame_system::Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
// pub type BalanceOf<T> = <<T as Trait>::Currency as Currency<<T as frame_system::Trait>::AccountId>>::Balance;

pub trait Trait: frame_system::Trait {
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
}
// pub type Value = u128;

decl_storage! {
	trait Store for Module<T: Trait> as TemplateModule {
		Something get(fn something): u32;

	}
}

decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Trait>::AccountId {
		SomethingStored(u32, AccountId),
		// RewardsIssued(u32),
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
		type Error = Error<T>;

		// Events must be initialized if they are used by the pallet.
		fn deposit_event() = default;

		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn do_something(origin, something: u32) -> dispatch::DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://substrate.dev/docs/en/knowledgebase/runtime/origin
			let who = ensure_signed(origin)?;

			// Update storage.
			Something::put(something);

			// Emit an event.
			Self::deposit_event(RawEvent::SomethingStored(something, who));
			// Return a successful DispatchResult
			Ok(())
		}

		// fn getOrigin(origin) -> dispatch::DispatchResult {
		// 	let who = ensure_signed(origin)?;
			
		// 	// Origin Get,
		// 	// Last balance of that T.

		// 	//T::Currency::deposit_creating(&additional_destination, additional_reward);
		// }

		
//--------------

		// fn on_finalize(block: T::BlockNumber) {
		// 	// let origin: T::Origin;

			// let author = frame_system::Module::<T>::digest()
			// 	.logs
			// 	.iter()
			// 	.filter_map(|s| s.as_pre_runtime())
			// 	.filter_map(|(id, mut data)| if id == AURA_ENGINE_ID {
			// 	T::AccountId::decode(&mut data).ok()
			// 	} else {
			// 		None
			// 	})
			// 	.next();


		// 	// let who = ensure_root(origin);


		//     debug::info!("===================================================");
		// 	debug::info!("Module REWARD on_finalize Called for Block :# {:?}",  block);
		// 	// debug::info!("=================================================== : {:?}", who);
			
		// 	// let auth:Vec<_> = Aura::authorities().iter().map(|x| {
        //     //     let r: &Public = x.as_ref();
        //     //     r.0.into()
		// 	// }).collect();
			
		// 	Self::disperse_reward();

		// }
		
//---------------------


		//  fn on_finalize() {
        //     let auth:Vec<_> = AuthorityId::authorities().iter().map(|x| {
        //         let r: &Public = x.as_ref();
        //         r.0.into()
        //     }).collect();
        //     Self::disperse_reward(&auth);
        // }
	}
}

// impl<T: Trait> Module<T> {
// 	/// Redistribute combined reward value to block Author
// 	fn disperse_reward() {
// 		let mut reward: u32 = <Something>::get();
// 		reward += 1;
// 		Something::put(reward);
// 		debug::info!("My reward on finalize: {:?}", reward);
// 		Self::deposit_event(RawEvent::RewardsIssued(reward));
// 	}	
// }