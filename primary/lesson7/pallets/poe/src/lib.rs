#![cfg_attr(not(feature = "std"), no_std)]

// 1. Imports
use frame_support::{decl_error, decl_event, decl_module, decl_storage, ensure, StorageMap};
use frame_system::ensure_signed;
use sp_std::vec::Vec;

// 2. Configuration
pub trait Trait: frame_system::Trait {
	/// Because this pallet emits events, it depends on the runtime's definition of an event.
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
}

// 3. Storage
decl_storage! {
	trait Store for Module<T: Trait> as TemplateModule {
			/// The storage item for our proofs.
			/// It maps a proof to the user who made the claim and when they made it.
			Proofs: map hasher(blake2_128_concat) Vec<u8> => (T::AccountId, T::BlockNumber);
	}
}

// 4. Events
decl_event! {
	pub enum Event<T> where AccountId = <T as frame_system::Trait>::AccountId {
			/// Event emitted when a proof has been claimed. [who, claim]
			ClaimCreated(AccountId, Vec<u8>),
			/// Event emitted when a claim is revoked by the owner. [who, claim]
			ClaimRevoked(AccountId, Vec<u8>),
			// transfer claim
			TransferClaim(Vec<u8>, AccountId),
	}
}

// 5. Errors
decl_error! {
	pub enum Error for Module<T: Trait> {
			/// The proof has already been claimed.
			ProofAlreadyClaimed,
			/// The proof does not exist, so it cannot be revoked.
			NoSuchProof,
			/// The proof is claimed by another account, so caller can't revoke it.
			NotProofOwner,
	}
}

// 6. Callable Functions
// Dispatchable functions allows users to interact with the pallet and invoke state changes.
// These functions materialize as "extrinsics", which are often compared to transactions.
// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
			// Errors must be initialized if they are used by the pallet.
			type Error = Error<T>;

			// Events must be initialized if they are used by the pallet.
			fn deposit_event() = default;

			/// Allow a user to claim ownership of an unclaimed proof.
			#[weight = 10_000]
			fn create_claim(origin, proof: Vec<u8>) {
					// Check that the extrinsic was signed and get the signer.
					// This function will return an error if the extrinsic is not signed.
					// https://substrate.dev/docs/en/knowledgebase/runtime/origin
					let sender = ensure_signed(origin)?;

					// Verify that the specified proof has not already been claimed.
					ensure!(!Proofs::<T>::contains_key(&proof), Error::<T>::ProofAlreadyClaimed);

					// Get the block number from the FRAME System module.
					let current_block = <frame_system::Module<T>>::block_number();

					// Store the proof with the sender and block number.
					Proofs::<T>::insert(&proof, (&sender, current_block));

					// Emit an event that the claim was created.
					Self::deposit_event(RawEvent::ClaimCreated(sender, proof));
			}

			/// Allow the owner to revoke their claim.
			#[weight = 10_000]
			fn revoke_claim(origin, proof: Vec<u8>) {
					// Check that the extrinsic was signed and get the signer.
					// This function will return an error if the extrinsic is not signed.
					// https://substrate.dev/docs/en/knowledgebase/runtime/origin
					let sender = ensure_signed(origin)?;

					// Verify that the specified proof has been claimed.
					ensure!(Proofs::<T>::contains_key(&proof), Error::<T>::NoSuchProof);

					// Get owner of the claim.
					let (owner, _) = Proofs::<T>::get(&proof);

					// Verify that sender of the current call is the claim owner.
					ensure!(sender == owner, Error::<T>::NotProofOwner);

					// Remove claim from storage.
					Proofs::<T>::remove(&proof);

					// Emit an event that the claim was erased.
					Self::deposit_event(RawEvent::ClaimRevoked(sender, proof));
			}

			/// Allow the owner to revoke their claim.
			#[weight = 10_000]
			fn transfer_claim(origin, proof: Vec<u8>, account_id: T::AccountId) {
					// Check that the extrinsic was signed and get the signer.
					// This function will return an error if the extrinsic is not signed.
					// https://substrate.dev/docs/en/knowledgebase/runtime/origin
					let sender = ensure_signed(origin)?;

					// Verify that the specified proof has been claimed.
					ensure!(Proofs::<T>::contains_key(&proof), Error::<T>::NoSuchProof);

					// Get owner of the claim.
					let (owner, _) = Proofs::<T>::get(&proof);

					// Verify that sender of the current call is the claim owner.
					ensure!(sender == owner, Error::<T>::NotProofOwner);

					// Get the block number from the FRAME System module.
					let current_block = <frame_system::Module<T>>::block_number();

					// remove
					Proofs::<T>::remove(&proof);
					// add new
					Proofs::<T>::insert(&proof, (&account_id, current_block));

					// update claim from storage.
					// Proofs::<T>::mutate(&proof, |a, b|(a=accountId, b = current_block));

					// Emit an event that the claim was erased.
					Self::deposit_event(RawEvent::TransferClaim(proof, account_id));
			}
	}
}
