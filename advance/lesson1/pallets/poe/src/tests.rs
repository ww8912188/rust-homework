use super::*;
use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

const USER: u64 = 0;
#[test]
fn it_works_for_create_poe() {
	new_test_ext().execute_with(|| {
		// create claim should work smoothly
		let claim = vec![0, 1];
		assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));
		assert_eq!(
			Proofs::<Test>::get(&claim),
			(1, frame_system::Module::<Test>::block_number())
		);
	});
}

#[test]
fn claim_exist_test() {
	new_test_ext().execute_with(|| {
		// could not create an exist claim
		let claim = vec![0, 1];
		let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
		assert_noop!(
			PoeModule::create_claim(Origin::signed(2), claim.clone()),
			Error::<Test>::ProofAlreadyClaimed
		);
	});
}

#[test]
fn it_works_for_revoke_poe() {
	new_test_ext().execute_with(|| {
		// revoke claim by owner should works smoothly
		let claim = vec![0, 1];
		let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
		assert_ok!(PoeModule::revoke_claim(Origin::signed(1), claim.clone()));
	});
}

#[test]
fn revoke_nonexist_poe() {
	new_test_ext().execute_with(|| {
		// try to revoke un-exist claim
		let claim = vec![0, 1];
		assert_noop!(
			PoeModule::revoke_claim(Origin::signed(1), claim.clone()),
			Error::<Test>::NoSuchProof
		);
	});
}

#[test]
fn auth_revoke_poe() {
	new_test_ext().execute_with(|| {
		// other user could not revoke claim
		let claim = vec![0, 1];
		let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
		assert_noop!(
			PoeModule::revoke_claim(Origin::signed(2), claim.clone()),
			Error::<Test>::NotProofOwner
		);
	});
}

#[test]
fn transfer_poe_works() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1];
		let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
		let _ = PoeModule::transfer_claim(Origin::signed(1), claim.clone(), USER);
		// make sure user 1 is not the claim owner anymore
		assert_noop!(
			PoeModule::revoke_claim(Origin::signed(1), claim.clone()),
			Error::<Test>::NotProofOwner
		);
	});
}

#[test]
fn long_poe_should_not_work() {
	new_test_ext().execute_with(|| {
		let mut vec = vec![];
		let count: u64 = 65;
		let mut n = 0;
		while n < count {
			vec.push(1);
			n += 1;
		}
		// make sure it will raise claim too long error if claim length is over than 64
		assert_noop!(
			PoeModule::create_claim(Origin::signed(1), vec),
			Error::<Test>::ClaimTooLong
		);
	});
}
