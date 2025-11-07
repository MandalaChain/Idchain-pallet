// KILT Blockchain – https://botlabs.org
// Copyright (C) 2019-2024 BOTLabs GmbH

// The KILT Blockchain is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The KILT Blockchain is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// If you feel like getting in touch with us, you can do so at info@botlabs.org

use ctype::mock::get_ctype_hash;
use frame_support::{assert_noop, assert_ok};
use kilt_support::mock::mock_origin::DoubleOrigin;
use sp_runtime::DispatchError;

use crate::{self as verification, mock::*, VerificationAccessControl, AttesterOf, Config};

#[test]
fn test_verify_without_authorization() {
	let attester: AttesterOf<Test> = sr25519_did_from_public_key(&ALICE_SEED);
	let claim_hash = claim_hash_from_seed(CLAIM_HASH_SEED_01);
	let ctype_hash = get_ctype_hash::<Test>(true);
	let authorization_info = None;

	ExtBuilder::default()
		.with_ctypes(vec![(ctype_hash, attester.clone())])
		.with_balances(vec![(ACCOUNT_00, <Test as Config>::Deposit::get() * 100)])
		.build_and_execute_with_sanity_tests(|| {
			assert_ok!(Verification::add(
				DoubleOrigin(ACCOUNT_00, attester.clone()).into(),
				claim_hash,
				ctype_hash,
				authorization_info.clone()
			));
			let stored_verification =
				Verification::verifications(claim_hash).expect("Verification should be present on chain.");

			assert_eq!(stored_verification.ctype_hash, ctype_hash);
			assert_eq!(stored_verification.attester, attester);
			assert_eq!(
				stored_verification.authorization_id,
				authorization_info.map(|ac| ac.authorization_id())
			);
			assert!(!stored_verification.revoked);
		});
}

#[test]
fn test_verify_authorized() {
	let attester: AttesterOf<Test> = sr25519_did_from_public_key(&ALICE_SEED);
	let claim_hash = claim_hash_from_seed(CLAIM_HASH_SEED_01);
	let ctype = get_ctype_hash::<Test>(true);
	let authorization_info = Some(MockAccessControl(attester.clone()));

	ExtBuilder::default()
		.with_ctypes(vec![(ctype, attester.clone())])
		.with_balances(vec![(ACCOUNT_00, <Test as Config>::Deposit::get() * 100)])
		.build_and_execute_with_sanity_tests(|| {
			assert_ok!(Verification::add(
				DoubleOrigin(ACCOUNT_00, attester.clone()).into(),
				claim_hash,
				ctype,
				authorization_info.clone()
			));
			let stored_verification =
				Verification::verifications(claim_hash).expect("Verification should be present on chain.");
			assert!(Verification::external_verifications(attester.clone(), claim_hash));

			assert_eq!(stored_verification.ctype_hash, ctype);
			assert_eq!(stored_verification.attester, attester);
			assert_eq!(
				stored_verification.authorization_id,
				authorization_info.map(|ac| ac.authorization_id())
			);
			assert!(!stored_verification.revoked);
		});
}

#[test]
fn test_verify_unauthorized() {
	let attester: AttesterOf<Test> = sr25519_did_from_public_key(&ALICE_SEED);
	let bob: AttesterOf<Test> = sr25519_did_from_public_key(&BOB_SEED);
	let claim_hash = claim_hash_from_seed(CLAIM_HASH_SEED_01);
	let ctype = get_ctype_hash::<Test>(true);
	let authorization_info = Some(MockAccessControl(bob));

	ExtBuilder::default()
		.with_ctypes(vec![(ctype, attester.clone())])
		.with_balances(vec![(ACCOUNT_00, <Test as Config>::Deposit::get() * 100)])
		.build_and_execute_with_sanity_tests(|| {
			assert_eq!(
				Verification::add(
					DoubleOrigin(ACCOUNT_00, attester.clone()).into(),
					claim_hash,
					ctype,
					authorization_info
				),
				Err(DispatchError::Other("Unauthorized"))
			);
		});
}

#[test]
fn test_verify_ctype_not_found() {
	let attester: AttesterOf<Test> = sr25519_did_from_public_key(&ALICE_SEED);
	let claim_hash = claim_hash_from_seed(CLAIM_HASH_SEED_01);
	let ctype_hash = get_ctype_hash::<Test>(true);

	ExtBuilder::default()
		.with_balances(vec![(ACCOUNT_00, <Test as Config>::Deposit::get() * 100)])
		.build_and_execute_with_sanity_tests(|| {
			assert_noop!(
				Verification::add(
					DoubleOrigin(ACCOUNT_00, attester.clone()).into(),
					claim_hash,
					ctype_hash,
					None
				),
				ctype::Error::<Test>::NotFound
			);
		});
}

#[test]
fn test_verify_already_exists() {
	let attester: AttesterOf<Test> = sr25519_did_from_public_key(&ALICE_SEED);
	let claim_hash = claim_hash_from_seed(CLAIM_HASH_SEED_01);
	let verification = generate_base_verification::<Test>(attester.clone(), ACCOUNT_00);

	ExtBuilder::default()
		.with_balances(vec![(ACCOUNT_00, <Test as Config>::Deposit::get() * 100)])
		.with_ctypes(vec![(verification.ctype_hash, attester.clone())])
		.with_verifications(vec![(claim_hash, verification.clone())])
		.build_and_execute_with_sanity_tests(|| {
			assert_noop!(
				Verification::add(
					DoubleOrigin(ACCOUNT_00, attester.clone()).into(),
					claim_hash,
					verification.ctype_hash,
					None
				),
				verification::Error::<Test>::AlreadyVerified
			);
		});
}
