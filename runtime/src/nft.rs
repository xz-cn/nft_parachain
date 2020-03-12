/// For more guidance on Substrate modules, see the example module
/// https://github.com/paritytech/substrate/blob/master/srml/example/src/lib.rs

use support::{decl_module, decl_storage, decl_event, StorageMap, StorageValue, ensure, dispatch::Result};
use system::ensure_signed;
use parity_codec::{Encode, Decode};
use rstd::cmp;

#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct CollectionType<AccountId> {
	pub Owner: AccountId,
	pub NextItemId: u64,
	pub CustomDataSize: u32,
}

/// The module's configuration trait.
pub trait Trait: system::Trait {
	// TODO: Add other types and constants required configure this module.

	// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

// This module's storage items.
decl_storage! {
	trait Store for Module<T: Trait> as nft {

		//pub MyBool get(my_bool_getter): bool;

		/// Next available collection ID
		pub NextCollectionID get(next_collection_id): u64;

		/// Collection map
		pub Collection get(collection): map u64 => CollectionType<T::AccountId>;

		//pub some_address: T::AccountId; 


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
		//pub fn create_collection(origin, collection: Vec<u8>) -> Result {
			pub fn create_collection(origin, customDataSz: u32) -> Result {
				// Anyone can create a collection
				let who = ensure_signed(origin)?;
	
				//<some_address<T>>::put(who);
	
				// Generate next collection ID
				let nextId = Self::next_collection_id();
				//let nextId = 1;
				<NextCollectionID<T>>::put(nextId+1);
	
				// Extract collection parameters
				//let collectionName = String::from_utf8(collection).unwrap();
	
				// Create new collection
				let new_collection = CollectionType {
					Owner: who,
					NextItemId: 1,
					CustomDataSize: customDataSz,
				};
				
				// Add new collection to map
				<Collection<T>>::insert(nextId, new_collection);
	
				Ok(())
			}

			pub fn destroy_collection(origin, collection_id: u64) -> Result {

				let sender = ensure_signed(origin)?;
				let owner = <Collection<T>>::get(collection_id).Owner;

				ensure!(sender == owner, "You do not own this collection");
				<Collection<T>>::remove(collection_id);

				Ok(())
			}

			pub fn change_collection_owner(origin, collection_id: u64, new_owner: T::AccountId) -> Result {

				let sender = ensure_signed(origin)?;
				ensure!(<Collection<T>>::exists(collection_id), "This collection does not exist");

				let mut target_collection = <Collection<T>>::get(collection_id);
				ensure!(sender == target_collection.Owner, "You do not own this collection");

				target_collection.Owner = new_owner;
				<Collection<T>>::insert(collection_id, target_collection);

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

/// tests for this module
#[cfg(test)]
mod tests {
	use super::*;

	use runtime_io::with_externalities;
	use primitives::{H256, Blake2Hasher};
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

	#[test]
	fn it_works_for_default_value() {
		with_externalities(&mut new_test_ext(), || {
			// Just a dummy test for the dummy funtion `do_something`
			// calling the `do_something` function with a value 42
			assert_ok!(nft::do_something(Origin::signed(1), 42));
			// asserting that the stored value is equal to what we stored
			assert_eq!(nft::something(), Some(42));
		});
	}
}