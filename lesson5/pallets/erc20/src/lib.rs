#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Codec, Decode, Encode};
use frame_support::{
	decl_error, decl_event, decl_module, decl_storage, dispatch::DispatchResult, ensure, Parameter,
};
use frame_system::{self as system, ensure_signed};
use sp_runtime::traits::{AtLeast32BitUnsigned, CheckedAdd, CheckedSub, Member};
use sp_std::prelude::*;

// the module trait
// contains type definitions
pub trait Trait: system::Trait {
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
	type TokenBalance: CheckedAdd
		+ CheckedSub
		+ Parameter
		+ Member
		+ Codec
		+ Default
		+ Copy
		+ AtLeast32BitUnsigned;
}

#[derive(Encode, Decode, Default, Clone, PartialEq, Debug)]
pub struct Erc20Token<U> {
	name: Vec<u8>,
	ticker: Vec<u8>,
	total_supply: U,
}

decl_storage! {
	trait Store for Module<T: Trait> as Erc20{
			// token id nonce for storing the next token id available for token initialization
			// inspired by the AssetId in the SRML assets module
			TokenId get(fn token_id): u32;
			Tokens get(fn token_details):  map hasher(blake2_128_concat) u32 => Erc20Token<T::TokenBalance>;
			Balanceof get(fn balance_of): map hasher(blake2_128_concat) (u32, T::AccountId) => T::TokenBalance;
			Allowance get(fn allowance): map hasher(blake2_128_concat) (u32, T::AccountId, T::AccountId) => T::TokenBalance;
	}
}

decl_module! {
		pub struct Module<T: Trait> for enum Call where origin: T::Origin {
				fn deposit_event() = default;

				#[weight=0]
				// initializes a new token
				// generates an integer token_id so that all tokens are unique
				// takes a name, ticker, total supply for the token
				// makes the initiating account the owner of the token
				// the balance of the owner is set to total supply
				fn init(origin, name: Vec<u8>, ticker: Vec<u8>, total_supply: T::TokenBalance) -> DispatchResult {
						let sender = ensure_signed(origin)?;
						ensure!(name.len() <= 64,"token name cannot exceed 64 bytes");
						ensure!(ticker.len() <= 32,"token ticker cannot exceed 32 bytes");

						let token_id = Self::token_id();
						let next_token_id = token_id.checked_add(1).ok_or("overflow in calculating next token id")?;

						let token = Erc20Token {
								name,
								ticker,
								total_supply,
						};
						<TokenId>::put(next_token_id);
						<Tokens<T>>::insert(token_id, token);
						<Balanceof<T>>::insert((token_id, sender),total_supply);

						Ok(())
				}

				// transfer tokens from one account to another
				#[weight=0]
				fn transfer(_origin, token_id: u32, to: T::AccountId, value: T::TokenBalance) -> DispatchResult {
						let sender = ensure_signed(_origin)?;
						Self::_transfer(token_id, sender,to,value)
				}

				#[weight=0]
				pub fn transfer_from(_origin, token_id: u32, from: T::AccountId, to: T::AccountId, value: T::TokenBalance) -> DispatchResult {
						ensure!(<Allowance<T>>::contains_key((token_id, from.clone(), to.clone())), "Allowance does not exist.");
						let allowance = Self::allowance((token_id, from.clone(), to.clone()));
						ensure!(allowance >= value, "Not enough allowance.");

						let updated_allowance = allowance.checked_sub(&value).ok_or(Error::<T>::Storageoverflow)?;
						<Allowance<T>>::insert((token_id, from.clone(), to.clone()), updated_allowance);

						Self::deposit_event(RawEvent::Approval(token_id, from.clone(), to.clone(),value));
						Self::_transfer(token_id, from,to,value)
				}

				// approve token transfer from one account to another
				#[weight=0]
				fn approve(_origin, token_id: u32, spender: T::AccountId,value: T::TokenBalance) -> DispatchResult{
						let sender = ensure_signed(_origin)?;
						ensure!(<Balanceof<T>>::contains_key((token_id, sender.clone())), "Account does not own this token");
						let allowance = Self::allowance((token_id, sender.clone(),spender.clone()));
						let updated_allowance= allowance.checked_add(&value).ok_or(Error::<T>::Storageoverflow)?;
						<Allowance<T>>::insert((token_id, sender.clone(),spender.clone()),updated_allowance);
						Self::deposit_event(RawEvent::Approval(token_id, sender.clone(),spender.clone(),value));
						Ok(())
				}
		}
}

decl_event!(
	pub enum Event<T>
	where
		AccountId = <T as system::Trait>::AccountId,
		Balance = <T as self::Trait>::TokenBalance,
	{
		Transfer(u32, AccountId, AccountId, Balance),
		Approval(u32, AccountId, AccountId, Balance),
	}
);
decl_error! {
		pub enum Error for Module<T: Trait> {
				Storageoverflow,
		}
}

impl<T: Trait> Module<T> {
	fn _transfer(
		token_id: u32,
		from: T::AccountId,
		to: T::AccountId,
		value: T::TokenBalance,
	) -> DispatchResult {
		let sender_balance = Self::balance_of((token_id, from.clone()));
		ensure!(sender_balance >= value, "Not enough balance.");

		let updated_from_balance = sender_balance
			.checked_sub(&value)
			.ok_or(Error::<T>::Storageoverflow)?;
		let receiver_balance = Self::balance_of((token_id, to.clone()));
		let updated_to_balance = receiver_balance
			.checked_sub(&value)
			.ok_or(Error::<T>::Storageoverflow)?;

		<Balanceof<T>>::insert((token_id, from.clone()), updated_from_balance);
		<Balanceof<T>>::insert((token_id, to.clone()), updated_to_balance);

		Self::deposit_event(RawEvent::Transfer(token_id, from, to, value));
		Ok(())
	}
}
