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

//! # Attestation Pallet
//!
//! Provides means of adding KILT attestations on chain and revoking them.
//!
//! - [`Config`]
//! - [`Call`]
//! - [`Pallet`]
//!
//! ### Terminology
//!
//! - **Claimer:**: A user which claims properties about themselves in the
//!   format of a CType. This could be a person which claims to have a valid
//!   driver's license.
//!
//! - **Attester:**: An entity which checks a user's claim and approves its
//!   validity. This could be a Citizens Registration Office which issues
//!   drivers licenses.
//!
//! - **Verifier:**: An entity which wants to check a user's claim by checking
//!   the provided verification.
//!
//! - **CType:**: CTypes are claim types. In everyday language, they are
//!   standardised structures for credentials. For example, a company may need a
//!   standard identification credential to identify workers that includes their
//!   full name, date of birth, access level and id number. Each of these are
//!   referred to as an attribute of a credential.
//!
//! - **Attestation:**: An approved or revoked user's claim in the format of a
//!   CType.
//!
//! - **Delegation:**: An verification which is not issued by the attester
//!   directly but via a (chain of) delegations which entitle the delegated
//!   attester. This could be an employe of a company which is authorized to
//!   sign documents for their superiors.
//!
//! ## Assumptions
//!
//! - The claim which shall be attested is based on a CType and signed by the
//!   claimer.
//! - The Verifier trusts the Attester. Otherwise, the verification is worthless
//!   for the Verifier

#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::unused_unit)]

pub mod verifications;
pub mod default_weights;
pub mod migrations;

#[cfg(any(feature = "mock", test))]
pub mod mock;

#[cfg(feature = "runtime-benchmarks")]
pub mod benchmarking;

#[cfg(any(feature = "try-runtime", test))]
mod try_state;

mod access_control;
#[cfg(test)]
mod tests;

pub use crate::{
	access_control::VerificationAccessControl, verifications::VerificationDetails, default_weights::WeightInfo, pallet::*,
};

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	use frame_support::{
		dispatch::{DispatchResult, DispatchResultWithPostInfo},
		pallet_prelude::*,
		traits::{
			fungible::{Inspect, MutateHold},
			Get, StorageVersion,
		},
	};
	use frame_system::pallet_prelude::*;

	use uid_credential::CtypeHashOf;
	use kilt_support::{
		traits::{BalanceMigrationManager, CallSources, StorageDepositCollector},
		Deposit,
	};

	/// The current storage version.
	const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

	/// Type of a claim hash.
	pub type ClaimHashOf<T> = <T as frame_system::Config>::Hash;

	/// Type of an attester identifier.
	pub type AttesterOf<T> = <T as Config>::AttesterId;

	/// Authorization id type
	pub(crate) type AuthorizationIdOf<T> = <T as Config>::AuthorizationId;

	pub(crate) type AccountIdOf<T> = <T as frame_system::Config>::AccountId;

	pub(crate) type BalanceOf<T> = <<T as Config>::Currency as Inspect<AccountIdOf<T>>>::Balance;

	pub(crate) type CurrencyOf<T> = <T as Config>::Currency;

	pub(crate) type HoldReasonOf<T> = <T as Config>::RuntimeHoldReason;

	pub(crate) type BalanceMigrationManagerOf<T> = <T as Config>::BalanceMigrationManager;

	pub type VerificationDetailsOf<T> =
		VerificationDetails<CtypeHashOf<T>, AttesterOf<T>, AuthorizationIdOf<T>, AccountIdOf<T>, BalanceOf<T>>;

	#[pallet::composite_enum]
	pub enum HoldReason {
		Deposit,
	}

	#[pallet::config]
	pub trait Config: frame_system::Config + uid_credential::Config {
		type EnsureOrigin: EnsureOrigin<
			<Self as frame_system::Config>::RuntimeOrigin,
			Success = <Self as Config>::OriginSuccess,
		>;
		type OriginSuccess: CallSources<AccountIdOf<Self>, AttesterOf<Self>>;
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type WeightInfo: WeightInfo;

		type RuntimeHoldReason: From<HoldReason>;

		/// The currency that is used to hold funds for each verification.
		type Currency: MutateHold<AccountIdOf<Self>, Reason = HoldReasonOf<Self>>;

		/// The deposit that is required for storing an verification.
		#[pallet::constant]
		// TODO: remove deposit
		type Deposit: Get<BalanceOf<Self>>;

		/// The maximum number of delegated verification credentials which can be made by
		/// the same delegation.
		#[pallet::constant]
		type MaxDelegatedVerifications: Get<u32>;

		type AttesterId: Parameter + MaxEncodedLen;

		type AuthorizationId: Parameter + MaxEncodedLen;

		type AccessControl: Parameter
			+ VerificationAccessControl<Self::AttesterId, Self::AuthorizationId, CtypeHashOf<Self>, ClaimHashOf<Self>>;

		/// Migration manager to handle new created entries
		type BalanceMigrationManager: BalanceMigrationManager<AccountIdOf<Self>, BalanceOf<Self>>;
	}

	#[pallet::pallet]
	#[pallet::storage_version(STORAGE_VERSION)]
	pub struct Pallet<T>(_);

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		#[cfg(feature = "try-runtime")]
		fn try_state(_n: BlockNumberFor<T>) -> Result<(), sp_runtime::TryRuntimeError> {
			crate::try_state::do_try_state::<T>()
		}
	}

	/// Verifications stored on chain.
	///
	/// It maps from a claim hash to the full verification credential.
	#[pallet::storage]
	#[pallet::getter(fn verifications)]
	pub type Verifications<T> = StorageMap<_, Blake2_128Concat, ClaimHashOf<T>, VerificationDetailsOf<T>>;

	/// Delegated verifications stored on chain.
	///
	/// It maps from a delegation ID to a vector of claim hashes.
	#[pallet::storage]
	#[pallet::getter(fn external_verifications)]
	pub type ExternalVerifications<T> =
		StorageDoubleMap<_, Twox64Concat, AuthorizationIdOf<T>, Blake2_128Concat, ClaimHashOf<T>, bool, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A new verification credential has been created.
		/// \[attester ID, claim hash, CType hash, (optional) delegation ID\]
		VerificationCreated(
			AttesterOf<T>,
			ClaimHashOf<T>,
			CtypeHashOf<T>,
			Option<AuthorizationIdOf<T>>,
		),
		/// A verification credential has been revoked.
		/// \[account id, claim hash\]
		VerificationRevoked(AttesterOf<T>, ClaimHashOf<T>),
		/// A verification credential has been removed.
		/// \[account id, claim hash\]
		VerificationRemoved(AttesterOf<T>, ClaimHashOf<T>),
		/// The deposit owner reclaimed a deposit by removing a verification credential.
		/// \[account id, claim hash\]
		DepositReclaimed(AccountIdOf<T>, ClaimHashOf<T>),
	}

	#[pallet::error]
	pub enum Error<T> {
		/// There is already a verification credential with the same claim hash stored on
		/// chain.
		AlreadyVerified,
		/// The verification credential has already been revoked.
		AlreadyRevoked,
		/// No verification credential on chain matching the claim hash.
		NotFound,
		/// The verification credential CType does not match the CType specified in the
		/// delegation hierarchy root.
		CTypeMismatch,
		/// The call origin is not authorized to change the verification credential.
		NotAuthorized,
		/// The maximum number of delegated verification credentials has already been
		/// reached for the corresponding delegation id such that another one
		/// cannot be added.
		MaxDelegatedVerificationsExceeded,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Create a new verification credential.
		///
		/// The attester can optionally provide a reference to an existing
		/// delegation that will be saved along with the verification credential itself in
		/// the form of an attested delegation.
		///
		/// The referenced CType hash must already be present on chain.
		///
		/// If an optional delegation id is provided, the dispatch origin must
		/// be the owner of the delegation. Otherwise, it could be any
		/// `DelegationEntityId`.
		///
		/// Emits `VerificationCreated`.
		///
		/// # <weight>
		/// Weight: O(1)
		/// - Reads: [Origin Account], Ctype, Verifications
		/// - Reads if delegation id is provided: Delegations, Roots,
		///   DelegatedVerifications
		/// - Writes: Verifications, (DelegatedVerifications)
		/// # </weight>
		#[pallet::call_index(0)]
		#[pallet::weight(
			<T as pallet::Config>::WeightInfo::add()
			.saturating_add(authorization.as_ref().map(|ac| ac.can_attest_weight()).unwrap_or(Weight::zero()))
		)]
		pub fn add(
			origin: OriginFor<T>,
			claim_hash: ClaimHashOf<T>,
			ctype_hash: CtypeHashOf<T>,
			authorization: Option<T::AccessControl>,
		) -> DispatchResult {
			let source = <T as Config>::EnsureOrigin::ensure_origin(origin)?;
			let payer = source.sender();
			let who = source.subject();
			let deposit_amount = <T as Config>::Deposit::get();

			ensure!(
				uid_credential::Ctypes::<T>::contains_key(ctype_hash),
				uid_credential::Error::<T>::NotFound
			);
			ensure!(
				!Verifications::<T>::contains_key(claim_hash),
				Error::<T>::AlreadyVerified
			);

			// Check for validity of the delegation node if specified.
			authorization
				.as_ref()
				.map(|ac| ac.can_attest(&who, &ctype_hash, &claim_hash))
				.transpose()?;
			let authorization_id = authorization.as_ref().map(|ac| ac.authorization_id());

			let deposit = VerificationStorageDepositCollector::<T>::create_deposit(payer, deposit_amount)?;
			<T as Config>::BalanceMigrationManager::exclude_key_from_migration(&Verifications::<T>::hashed_key_for(
				claim_hash,
			));

			log::debug!("insert Verification");

			Verifications::<T>::insert(
				claim_hash,
				VerificationDetails {
					ctype_hash,
					attester: who.clone(),
					authorization_id: authorization_id.clone(),
					revoked: false,
					deposit,
				},
			);
			if let Some(authorization_id) = &authorization_id {
				ExternalVerifications::<T>::insert(authorization_id, claim_hash, true);
			}

			Self::deposit_event(Event::VerificationCreated(who, claim_hash, ctype_hash, authorization_id));

			Ok(())
		}

		/// Revoke an existing verification credential.
		///
		/// The revoker must be either the creator of the verification credential being
		/// revoked or an entity that in the delegation tree is an ancestor of
		/// the attester, i.e., it was either the delegator of the attester or
		/// an ancestor thereof.
		///
		/// Emits `VerificationRevoked`.
		///
		/// # <weight>
		/// Weight: O(P) where P is the number of steps required to verify that
		/// the dispatch Origin controls the delegation entitled to revoke the
		/// verification. It is bounded by `max_parent_checks`.
		/// - Reads: [Origin Account], Verifications, delegation::Roots
		/// - Reads per delegation step P: delegation::Delegations
		/// - Writes: Verifications, DelegatedVerifications
		/// # </weight>
		#[pallet::call_index(1)]
		#[pallet::weight(
			<T as pallet::Config>::WeightInfo::revoke()
			.saturating_add(authorization.as_ref().map(|ac| ac.can_revoke_weight()).unwrap_or(Weight::zero()))
		)]
		pub fn revoke(
			origin: OriginFor<T>,
			claim_hash: ClaimHashOf<T>,
			authorization: Option<T::AccessControl>,
		) -> DispatchResultWithPostInfo {
			let source = <T as Config>::EnsureOrigin::ensure_origin(origin)?;
			let who = source.subject();

			let verification = Verifications::<T>::get(claim_hash).ok_or(Error::<T>::NotFound)?;

			ensure!(!verification.revoked, Error::<T>::AlreadyRevoked);

			if verification.attester != who {
				let verification_auth_id = verification.authorization_id.as_ref().ok_or(Error::<T>::NotAuthorized)?;
				authorization.ok_or(Error::<T>::NotAuthorized)?.can_revoke(
					&who,
					&verification.ctype_hash,
					&claim_hash,
					verification_auth_id,
				)?;
			}

			log::debug!("revoking Verification");
			Verifications::<T>::insert(
				claim_hash,
				VerificationDetails {
					revoked: true,
					..verification
				},
			);

			Self::deposit_event(Event::VerificationRevoked(who, claim_hash));

			Ok(Some(<T as pallet::Config>::WeightInfo::revoke()).into())
		}

		/// Remove an verification credential.
		///
		/// The origin must be either the creator of the verification credential or an
		/// entity which is an ancestor of the attester in the delegation tree,
		/// i.e., it was either the delegator of the attester or an ancestor
		/// thereof.
		///
		/// Emits `VerificationRemoved`.
		///
		/// # <weight>
		/// Weight: O(P) where P is the number of steps required to verify that
		/// the dispatch Origin controls the delegation entitled to revoke the
		/// verification credential. It is bounded by `max_parent_checks`.
		/// - Reads: [Origin Account], Verifications, delegation::Roots
		/// - Reads per delegation step P: delegation::Delegations
		/// - Writes: Verifications, DelegatedVerifications
		/// # </weight>
		#[pallet::call_index(2)]
		#[pallet::weight(
			<T as pallet::Config>::WeightInfo::remove()
			.saturating_add(authorization.as_ref().map(|ac| ac.can_remove_weight()).unwrap_or(Weight::zero()))
		)]
		pub fn remove(
			origin: OriginFor<T>,
			claim_hash: ClaimHashOf<T>,
			authorization: Option<T::AccessControl>,
		) -> DispatchResultWithPostInfo {
			let source = <T as Config>::EnsureOrigin::ensure_origin(origin)?;
			let who = source.subject();

			let verification = Verifications::<T>::get(claim_hash).ok_or(Error::<T>::NotFound)?;

			if verification.attester != who {
				let verification_auth_id = verification.authorization_id.as_ref().ok_or(Error::<T>::NotAuthorized)?;
				authorization.ok_or(Error::<T>::NotAuthorized)?.can_remove(
					&who,
					&verification.ctype_hash,
					&claim_hash,
					verification_auth_id,
				)?;
			}

			log::debug!("removing Verification");

			Self::remove_verification(verification, claim_hash)?;
			Self::deposit_event(Event::VerificationRemoved(who, claim_hash));

			Ok(Some(<T as pallet::Config>::WeightInfo::remove()).into())
		}

		/// Reclaim a storage deposit by removing an verification credential
		///
		/// Emits `DepositReclaimed`.
		///
		/// # <weight>
		/// Weight: O(1)
		/// - Reads: [Origin Account], Verifications, DelegatedVerifications
		/// - Writes: Verifications, DelegatedVerifications
		/// # </weight>
		#[pallet::call_index(3)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::reclaim_deposit())]
		pub fn reclaim_deposit(origin: OriginFor<T>, claim_hash: ClaimHashOf<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let verification = Verifications::<T>::get(claim_hash).ok_or(Error::<T>::NotFound)?;

			ensure!(verification.deposit.owner == who, Error::<T>::NotAuthorized);

			log::debug!("removing Verification");

			Self::remove_verification(verification, claim_hash)?;
			Self::deposit_event(Event::DepositReclaimed(who, claim_hash));

			Ok(())
		}

		/// Changes the deposit owner.
		///
		/// The balance that is reserved by the current deposit owner will be
		/// freed and balance of the new deposit owner will get reserved.
		///
		/// The subject of the call must be the attester who issues the
		/// verification credential. The sender of the call will be the new deposit owner.
		#[pallet::call_index(4)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::change_deposit_owner())]
		pub fn change_deposit_owner(origin: OriginFor<T>, claim_hash: ClaimHashOf<T>) -> DispatchResult {
			let source = <T as Config>::EnsureOrigin::ensure_origin(origin)?;
			let subject = source.subject();
			let sender = source.sender();

			let verification = Verifications::<T>::get(claim_hash).ok_or(Error::<T>::NotFound)?;
			ensure!(verification.attester == subject, Error::<T>::NotAuthorized);

			VerificationStorageDepositCollector::<T>::change_deposit_owner::<BalanceMigrationManagerOf<T>>(
				&claim_hash,
				sender,
			)?;

			Ok(())
		}

		/// Updates the deposit amount to the current deposit rate.
		///
		/// The sender must be the deposit owner.
		#[pallet::call_index(5)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::update_deposit())]
		pub fn update_deposit(origin: OriginFor<T>, claim_hash: ClaimHashOf<T>) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let verification = Verifications::<T>::get(claim_hash).ok_or(Error::<T>::NotFound)?;
			ensure!(verification.deposit.owner == sender, Error::<T>::NotAuthorized);

			VerificationStorageDepositCollector::<T>::update_deposit::<BalanceMigrationManagerOf<T>>(&claim_hash)?;

			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		fn remove_verification(verification: VerificationDetailsOf<T>, claim_hash: ClaimHashOf<T>) -> DispatchResult {
			let is_key_migrated =
				<T as Config>::BalanceMigrationManager::is_key_migrated(&Verifications::<T>::hashed_key_for(claim_hash));
			if is_key_migrated {
				VerificationStorageDepositCollector::<T>::free_deposit(verification.deposit)?;
			} else {
				<T as Config>::BalanceMigrationManager::release_reserved_deposit(
					&verification.deposit.owner,
					&verification.deposit.amount,
				)
			}

			Verifications::<T>::remove(claim_hash);
			if let Some(authorization_id) = &verification.authorization_id {
				ExternalVerifications::<T>::remove(authorization_id, claim_hash);
			}
			Ok(())
		}
	}

	pub(crate) struct VerificationStorageDepositCollector<T: Config>(PhantomData<T>);
	impl<T: Config> StorageDepositCollector<AccountIdOf<T>, ClaimHashOf<T>, T::RuntimeHoldReason>
		for VerificationStorageDepositCollector<T>
	{
		type Currency = <T as Config>::Currency;
		type Reason = HoldReason;

		fn reason() -> Self::Reason {
			HoldReason::Deposit
		}

		fn get_hashed_key(key: &ClaimHashOf<T>) -> Result<sp_std::vec::Vec<u8>, DispatchError> {
			Ok(Verifications::<T>::hashed_key_for(key))
		}

		fn deposit(
			key: &ClaimHashOf<T>,
		) -> Result<Deposit<AccountIdOf<T>, <Self::Currency as Inspect<AccountIdOf<T>>>::Balance>, DispatchError> {
			let verification = Verifications::<T>::get(key).ok_or(Error::<T>::NotFound)?;
			Ok(verification.deposit)
		}

		fn deposit_amount(_key: &ClaimHashOf<T>) -> <Self::Currency as Inspect<AccountIdOf<T>>>::Balance {
			T::Deposit::get()
		}

		fn store_deposit(
			key: &ClaimHashOf<T>,
			deposit: Deposit<AccountIdOf<T>, <Self::Currency as Inspect<AccountIdOf<T>>>::Balance>,
		) -> Result<(), DispatchError> {
			let verification = Verifications::<T>::get(key).ok_or(Error::<T>::NotFound)?;
			Verifications::<T>::insert(key, VerificationDetails { deposit, ..verification });

			Ok(())
		}
	}
}
