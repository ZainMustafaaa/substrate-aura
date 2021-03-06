
#![cfg_attr(not(feature = "std"), no_std)]
use frame_support::{debug ,decl_module, dispatch::{DispatchResult,Vec}, weights::{Weight},decl_storage, decl_event, decl_error, dispatch, 
traits::{Get, Currency, OriginTrait}};

use frame_system::{ensure_signed, ensure_root};
use super::Aura;
use sp_core::{
	crypto::{Public as _},
	H256,
	H512,
	sr25519::{Public, Signature},
};

type BalanceOf<T> = <<T as Trait>::Currency as Currency<<T as frame_system::Trait>::AccountId>>::Balance;// use super::{block_author::BlockAuthor, issuance::Issuance};

// #[cfg(test)]
// mod mock;

// #[cfg(test)]
// mod tests;

pub trait Trait: frame_system::Trait {
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
	type Currency: Currency<Self::AccountId>;
}



// pub type Value = u32;

decl_storage! {
	trait Store for Module<T: Trait> as TemplateModule {
		Something get(fn something) config(): BalanceOf<T>;

	}
}

decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Trait>::AccountId {
		SomethingStored( AccountId),
		// RewardsIssued(BalanceOf<T>),
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

		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn do_something(origin, something: BalanceOf<T>) -> dispatch::DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://substrate.dev/docs/en/knowledgebase/runtime/origin
			let who = ensure_root(origin)?;

			// Update storage.
			Something::<T>::put(something);

			// Emit an event.
            // Self::deposit_event(RawEvent::SomethingStored(who));
            
			// Return a successful DispatchResult
			Ok(())
		}

		fn on_finalize(block: T::BlockNumber) {
		    debug::info!("===================================================");
			debug::info!("Current Block From Reward Runtime :# {:?}",  block);
			debug::info!("===================================================");			
			debug::info!("Aura Authorities array :  {:?}", Aura::authorities().iter());
			debug::info!("===================================================");			
			// debug::info!("Account ID: {:?}", <pallet_treasury::Module<T>>::account_id());			

			


		let auth:Vec<_> = Aura::authorities().iter().map(|x| {
                let r: &Public = x.as_ref();
                r.0.into()
			}).collect();
			
			Self::disperse_reward(&auth);

		}
		// fn on_finalize() {
		// 	Self::disperse_reward()
		// 	// match T::BlockAuthor::block_author() {
		// 	// 	// Block author did not provide key to claim reward
		// 	// 	None => Self::deposit_event(Event::RewardsWasted),
		// 	// 	// Block author did provide key, so issue thir reward
		// 	// 	Some(author) => Self::disperse_reward(&author),
		// 	// }
		// }

		//  fn on_finalize() {
        //     let auth:Vec<_> = AuthorityId::authorities().iter().map(|x| {
        //         let r: &Public = x.as_ref();
        //         r.0.into()
        //     }).collect();
        //     Self::disperse_reward(&auth);
        // }
	}
}

impl<T: Trait> Module<T> {
	/// Redistribute combined reward value to block Author
	fn disperse_reward(to_reward: &[H256]) {
		debug::info!("My Authorities Inside the desperss method: {:?}", to_reward);


		// let mut reward: BalanceOf<T> = Something::<T>::get();
		let mut reward = BalanceOf::<T>::from(100);
		// reward += 1;
		Something::<T>::put(reward);
		
		debug::info!("My reward on finalize after upgrading the storage and increment: {:?}", reward);
		for who in to_reward { 
			Currency::deposit_into_existing(&who, reward).ok();
			// Self::deposit_event(RawEvent::RewardsIssued(reward));
		}
		
	}	
}


	// fn on_finalize() {
	// 	let auth:Vec<_> = Aura::authorities().iter().map(|x| {
    //             let r: &Public = x.as_ref();
    //             r.0.into()
	// 		}).collect();
			
	// 		Self::disperse_reward(&auth);
	// }


	// fn disperse_reward(to_reward: &[AccountId]) {	
	// 	let reward = BalanceOf::<T>::from(100);
	
	// 	for who in to_reward { 
	// 		Currency::deposit_into_existing(&who, reward).ok();
	// 		Self::deposit_event(RawEvent::RewardsIssued(reward));
	// 	}
	// }	


	/**
		HI , Shawn we have implemented the same code you have specifid in your answer.
		But still we are getting the infer type error.
		Can you guys give us any help to call `deposit_into_existing` method on_finalize to disperse
		balance to author node or any multisig wallet.
		Thanks waiting for your response.
	*/