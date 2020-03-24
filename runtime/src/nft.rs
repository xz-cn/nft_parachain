#![cfg_attr(not(feature = "std"), no_std)]

/// For more guidance on Substrate modules, see the example module
/// https://github.com/paritytech/substrate/blob/master/srml/example/src/lib.rs

use support::{decl_module, decl_storage, decl_event, StorageMap, StorageValue, ensure, dispatch::Result};
use system::ensure_signed;
use parity_codec::{Encode, Decode};
use super::{ Vec };

#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct CollectionType<AccountId> {
	pub owner: AccountId,
	pub next_item_id: u64,
	pub custom_data_size: u32,
}

#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct CollectionAdminsType<AccountId> {
	pub admin: AccountId,
	pub collection_id: u64,
}

#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct NftItemType<AccountId> {
	pub collection: u64,
	pub owner: AccountId,
	pub data: Vec<u8>,
}

// impl Clone for NftItemType<AccountId> {
//     fn clone(&self) -> NftItemType<AccountId> {
//         *self
// 	}
// }

/// The module's configuration trait.
pub trait Trait: system::Trait {
	// TODO: Add other types and constants required configure this module.

	// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

// This module's storage items.
decl_storage! {
	trait Store for Module<T: Trait> as nft {

		/// Next available collection ID
		pub NextCollectionID get(next_collection_id): u64;

		/// Collection map
		pub Collection get(collection): map u64 => CollectionType<T::AccountId>;

		/// Admins map (collection)
		pub AdminList get(collection_id): map u64 => Vec<T::AccountId>;

		/// Balance owner per collection map
		pub Balance get(owner): map (u64, T::AccountId) => u64;

		/// Item double map (collection)
		pub ItemList get(item_id): map (u64, u64) => NftItemType<T::AccountId>;
		pub ItemListIndex get(index): map u64 => u64;
	}
}

decl_module! {
	/// The NFT module declaration.
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Initializing events
		// this is needed only if you are using events in your module
		// fn deposit_event<T>() = default;

		// Create collection of NFT with given parameters
		//
		// @param customDataSz size of custom data in each collection item
		// returns collection ID

		// Create collection of NFT with given parameters
		//
		// @param customDataSz size of custom data in each collection item
		// returns collection ID
			pub fn create_collection(origin, custom_data_sz: u32) -> Result {
				// Anyone can create a collection
				let who = ensure_signed(origin)?;
	
				// Generate next collection ID
				let next_id = Self::next_collection_id();
				<NextCollectionID<T>>::put(next_id+1);
	
				// Create new collection
				let new_collection = CollectionType {
					owner: who,
					next_item_id: next_id,
					custom_data_size: custom_data_sz,
				};
				
				// Add new collection to map
				<Collection<T>>::insert(next_id, new_collection);
	
				Ok(())
			}

			pub fn destroy_collection(origin, collection_id: u64) -> Result {

				let sender = ensure_signed(origin)?;
				let owner = <Collection<T>>::get(collection_id).owner;

				ensure!(sender == owner, "You do not own this collection");
				<Collection<T>>::remove(collection_id);

				Ok(())
			}

			pub fn change_collection_owner(origin, collection_id: u64, new_owner: T::AccountId) -> Result {

				let sender = ensure_signed(origin)?;
				ensure!(<Collection<T>>::exists(collection_id), "This collection does not exist");

				let mut target_collection = <Collection<T>>::get(collection_id);
				ensure!(sender == target_collection.owner, "You do not own this collection");

				target_collection.owner = new_owner;
				<Collection<T>>::insert(collection_id, target_collection);

				Ok(())
			}

			pub fn add_collection_admin(origin, collection_id: u64, new_admin_id: T::AccountId) -> Result {

				runtime_io::print("Add admin called");
				let sender = ensure_signed(origin)?;
				ensure!(<Collection<T>>::exists(collection_id), "This collection does not exist");

				let target_collection = <Collection<T>>::get(collection_id);
				let is_owner = sender == target_collection.owner;

				let no_perm_mes = "You do not have permissions to modify this collection";
				let exists = <AdminList<T>>::exists(collection_id);

				if !is_owner 
				{
					 ensure!(exists, no_perm_mes);
					 ensure!(<AdminList<T>>::get(collection_id).contains(&sender), no_perm_mes);
				}
				
				let mut admin_arr: Vec<T::AccountId> = Vec::new();
				if exists
				{
					admin_arr = <AdminList<T>>::get(collection_id);
					ensure!(!admin_arr.contains(&new_admin_id), "Account already has admin role");
				}

				admin_arr.push(new_admin_id);
				<AdminList<T>>::insert(collection_id, admin_arr);

				Ok(())
			}

			pub fn remove_collection_admin(origin, collection_id: u64, account_id: T::AccountId) -> Result {

				let sender = ensure_signed(origin)?;
				ensure!(<Collection<T>>::exists(collection_id), "This collection does not exist");

				let target_collection = <Collection<T>>::get(collection_id);
				let is_owner = sender == target_collection.owner;

				let no_perm_mes = "You do not have permissions to modify this collection";
				let exists = <AdminList<T>>::exists(collection_id);

				if !is_owner 
				{
					ensure!(exists, no_perm_mes);
					ensure!(<AdminList<T>>::get(collection_id).contains(&sender), no_perm_mes);
				}

				if exists
				{
					let mut admin_arr = <AdminList<T>>::get(collection_id);
					admin_arr.retain(|i| *i != account_id);
					<AdminList<T>>::insert(collection_id, admin_arr);
				}

				Ok(())
			}

			pub fn create_item(origin, collection_id: u64, properties: Vec<u8>) -> Result {

				let sender = ensure_signed(origin)?;
				ensure!(<Collection<T>>::exists(collection_id), "This collection does not exist");

				let target_collection = <Collection<T>>::get(collection_id);
				ensure!(target_collection.custom_data_size >= properties.len() as u32, "Size of item is too large");
				let is_owner = sender == target_collection.owner;

				let no_perm_mes = "You do not have permissions to modify this collection";
				let exists = <AdminList<T>>::exists(collection_id);

				if !is_owner 
				{
					ensure!(exists, no_perm_mes);
					ensure!(<AdminList<T>>::get(collection_id).contains(&sender), no_perm_mes);
				}

				let new_balance = <Balance<T>>::get((collection_id, sender.clone())) + 1;
				<Balance<T>>::insert((collection_id, sender.clone()), new_balance);

				// Create new item
				let new_item = NftItemType {
					collection: collection_id,
					owner: sender,
					data: properties,
				};

				let current_index = <ItemListIndex<T>>::get(collection_id);
				<ItemListIndex<T>>::insert(collection_id, current_index+1);
				<ItemList<T>>::insert((collection_id, current_index), new_item);

				Ok(())
			}

			pub fn burn_item(origin, collection_id: u64, item_id: u64) -> Result {

				let sender = ensure_signed(origin)?;
				ensure!(<Collection<T>>::exists(collection_id), "This collection does not exist");

				let target_collection = <Collection<T>>::get(collection_id);
				let is_owner = sender == target_collection.owner;

				ensure!(<ItemList<T>>::exists((collection_id, item_id)), "Item does not exists");
				let item = <ItemList<T>>::get((collection_id, item_id));

				if !is_owner 
				{
					// check if item owner
					if item.owner != sender 
					{
						let no_perm_mes = "You do not have permissions to modify this collection";

						ensure!(<AdminList<T>>::exists(collection_id), no_perm_mes);
						ensure!(<AdminList<T>>::get(collection_id).contains(&sender), no_perm_mes);
					}
				}
				<ItemList<T>>::remove((collection_id, item_id));

				Ok(())
			}

			pub fn transfer(origin, collection_id: u64, item_id: u64, new_owner: T::AccountId) -> Result {

				let sender = ensure_signed(origin)?;
				ensure!(<Collection<T>>::exists(collection_id), "This collection does not exist");

				let target_collection = <Collection<T>>::get(collection_id);
				let is_owner = sender == target_collection.owner;

				ensure!(<ItemList<T>>::exists((collection_id, item_id)), "Item does not exists");
				let mut item = <ItemList<T>>::get((collection_id, item_id));

				if !is_owner 
				{
					// check if item owner
					if item.owner != sender 
					{
						let no_perm_mes = "You do not have permissions to modify this collection";

						ensure!(<AdminList<T>>::exists(collection_id), no_perm_mes);
						ensure!(<AdminList<T>>::get(collection_id).contains(&sender), no_perm_mes);
					}
				}
				<ItemList<T>>::remove((collection_id, item_id));

				// change owner
				item.owner = new_owner;
				<ItemList<T>>::insert((collection_id, item_id), item);


				Ok(())
			}
	}
}

decl_event!(
    pub enum Event<T>
    where
        <T as system::Trait>::AccountId
    {
        Created(u32, AccountId),
    }
);


// impl<T: Trait> Module<T> {
//     fn is_owner_or_admin(sender: T::AccountId, collection_id: u64, adm_lst: Vec<T::AccountId>) -> Result {

// 		Ok(())
// 	}
// }

/// tests for this module
#[cfg(test)]
mod tests {
	use super::*;

	use primitives::{H256, Blake2Hasher};
	use runtime_io::{with_externalities, TestExternalities};
	use support::{impl_outer_origin, assert_ok};
	use runtime_primitives::{
		BuildStorage,
		traits::{BlakeTwo256, IdentityLookup},
		testing::{Digest, DigestItem, Header}
	};

	impl_outer_origin! {
		pub enum Origin for Test {}
	}

	// For testing the module, we construct most of a mock runtime. This means
	// first constructing a configuration type (`Test`) which `impl`s each of the
	// configuration traits of modules we want to use.
	#[derive(Clone, Eq, PartialEq)]
	pub struct Test;
	impl system::Trait for Test {
		type Origin = Origin;
		type Index = u64;
		type BlockNumber = u64;
		type Hash = H256;
		type Hashing = BlakeTwo256;
		type Digest = Digest;
		type AccountId = u64;
		type Lookup = IdentityLookup<Self::AccountId>;
		type Header = Header;
		type Event = ();
		type Log = DigestItem;
	}
	impl Trait for Test {
		type Event = ();
	}
	type nft = Module<Test>;

	// This function basically just builds a genesis storage key/value store according to
	// our desired mockup.
	fn new_test_ext() -> runtime_io::TestExternalities<Blake2Hasher> {
		system::GenesisConfig::<Test>::default().build_storage().unwrap().0.into()
	}

    fn build_ext() -> TestExternalities<Blake2Hasher> {
        let mut t = system::GenesisConfig::<Test>::default().build_storage().unwrap().0;
        t.into()
    }

	#[test]
	fn create_collection_test() {
		with_externalities(&mut new_test_ext(), || {
			let id = 1;
			// Just a dummy test for the dummy funtion `do_something`
			// calling the `do_something` function with a value 42
			assert_ok!(nft::create_collection(Origin::signed(1), id));
		});
	}
}