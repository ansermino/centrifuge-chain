use frame_support::{decl_event, decl_module, decl_storage, dispatch::DispatchResult, dispatch::DispatchError, ensure};
use frame_system::{self as system, ensure_signed, ensure_root};
use sp_std::vec::Vec;
// TODO: Replace Vec<u8> with U256 where possible
use sp_core::U256;


pub trait Trait: system::Trait {
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
}

decl_event!(
    pub enum Event<T> where <T as frame_system::Trait>::Hash {
        AssetTransfer(Vec<u8>, Vec<u8>, Vec<u8>),
        UselessEvent(Hash),
    }
);

decl_storage!(
    trait Store for Module<T: Trait> as Bridge {
        EmitterAddress get(emitter_address): Vec<u8>;
        Chains get(fn chains): map Vec<u8> => Option<Counter>;
    }
);

decl_module!(
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        // Default method for emitting events
        fn deposit_event() = default;

        // Implicitly checks origin as root
        pub fn set_address(origin, addr: Vec<u8>) -> DispatchResult {
            // TODO: Limit access
            ensure_signed(origin)?;
            <EmitterAddress>::put(addr);
            Ok(())
        }

        pub fn whitelist_chain(origin, id: Vec<u8>) -> DispatchResult {
            // TODO: Limit access
            ensure_signed(origin)?;
            <Chains>::insert(&id, true);
            Ok(())
        }

        // TODO: Add metadata
        pub fn transfer_asset(origin, dest_id: Vec<u8>, to: Vec<u8>, token_id: Vec<u8>) -> DispatchResult {
            ensure_signed(origin)?;
            // Ensure chain is whitelisted
            ensure!(<Chains>::get(&dest_id), "Chain ID not whitelisted");
            Self::deposit_event(RawEvent::AssetTransfer(dest_id, to, token_id));
            Ok(())
        }
    }
);

#[cfg(test)]
mod tests {
    use super::*;

    use sp_core::H256;
    use sp_runtime::{
        testing::Header,
        traits::{BadOrigin, BlakeTwo256, Hash, IdentityLookup},
        Perbill,
    };
    use frame_support::{assert_err, assert_ok, impl_outer_origin, parameter_types, weights::Weight};

    impl_outer_origin! {
        pub enum Origin for Test {}
    }

    #[derive(Clone, Eq, PartialEq)]
    pub struct Test;

    type Bridge = super::Module<Test>;

    parameter_types! {
        pub const BlockHashCount: u64 = 250;
        pub const MaximumBlockWeight: Weight = 1024;
        pub const MaximumBlockLength: u32 = 2 * 1024;
        pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
    }

    impl frame_system::Trait for Test {
        type Origin = Origin;
        type Call = ();
        type Index = u64;
        type BlockNumber = u64;
        type Hash = H256;
        type Hashing = BlakeTwo256;
        type AccountId = u64;
        type Lookup = IdentityLookup<Self::AccountId>;
        type Header = Header;
        type Event = ();
        type BlockHashCount = BlockHashCount;
        type MaximumBlockWeight = MaximumBlockWeight;
        type MaximumBlockLength = MaximumBlockLength;
        type AvailableBlockRatio = AvailableBlockRatio;
        type Version = ();
        type ModuleToIndex = ();
    }

    impl Trait for Test {
        type Event = ();
    }

    fn new_test_ext() -> sp_io::TestExternalities {
        let mut t = frame_system::GenesisConfig::default()
            .build_storage::<Test>()
            .unwrap();

        t.into()
    }

    #[test]
    fn set_get_address() {
        new_test_ext().execute_with(|| {
            assert_ok!(Bridge::set_address(Origin::signed(1), vec![1,2,3,4]));
            assert_eq!(Bridge::emitter_address(), vec![1,2,3,4])
        })
    }

    #[test]
    fn whitelist_chain() {
        new_test_ext().execute_with(|| {
            let chain_id = vec![1];
            assert_ok!(Bridge::whitelist_chain(Origin::signed(1), chain_id.clone()));
            assert!(Bridge::chains(chain_id));
        })
    }

    #[test]
    fn asset_transfer_success() {
        new_test_ext().execute_with(|| {
            let chain_id = vec![1];
            let to = vec![2];
            let token_id = vec![3];

            assert_ok!(Bridge::whitelist_chain(Origin::signed(1), chain_id.clone()));
            assert_ok!(Bridge::transfer_asset(Origin::signed(1), chain_id, to, token_id));
            // TODO: Assert event
        })
    }

    #[test]
    fn asset_transfer_invalid_chain() {
        new_test_ext().execute_with(|| {
            let chain_id = vec![1];
            let to = vec![2];
            let dest_id = vec![3];
            let token_id = vec![4];

            assert_ok!(Bridge::whitelist_chain(Origin::signed(1), chain_id.clone()));
            assert_err!(Bridge::transfer_asset(Origin::signed(1), vec![2], to, token_id), "Chain ID not whitelisted");
        })
    }
}