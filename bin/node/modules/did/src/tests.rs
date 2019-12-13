#![cfg(test)]

use super::*;

use support::{assert_ok, assert_noop, impl_outer_origin, impl_outer_event, parameter_types};
use primitives::H256;
// The testing primitives are very useful for avoiding having to work with signatures
// or public keys. `u64` is used as the `AccountId` and no `Signature`s are required.
use sp_runtime::{
  Perbill, testing::Header, traits::{BlakeTwo256, IdentityLookup},
};
use system::{EventRecord, Phase};

impl_outer_origin! {
  pub enum Origin for Test {}
}

mod did {
  pub use super::super::*;
}

impl_outer_event! {
  pub enum Event for Test {
    did<T>, balances<T>,
  }
}
// For testing the module, we construct most of a mock runtime. This means
// first constructing a configuration type (`Test`) which `impl`s each of the
// configuration traits of modules we want to use.
#[derive(Clone, Eq, PartialEq)]
pub struct Test;
parameter_types! {
  pub const BlockHashCount: u64 = 250;
  pub const MaximumBlockWeight: u32 = 1024;
  pub const MaximumBlockLength: u32 = 2 * 1024;
  pub const AvailableBlockRatio: Perbill = Perbill::one();
}
impl system::Trait for Test {
  type Origin = Origin;
  type Index = u64;
  type BlockNumber = u64;
  type Hash = H256;
  type Call = ();
  type Hashing = BlakeTwo256;
  type AccountId = u64;
  type Lookup = IdentityLookup<Self::AccountId>;
  type Header = Header;
  type Event = Event;
  type BlockHashCount = BlockHashCount;
  type MaximumBlockWeight = MaximumBlockWeight;
  type MaximumBlockLength = MaximumBlockLength;
  type AvailableBlockRatio = AvailableBlockRatio;
  type Version = ();
}
parameter_types! {
  pub const ExistentialDeposit: u64 = 0;
  pub const TransferFee: u64 = 0;
  pub const CreationFee: u64 = 0;
}
impl balances::Trait for Test {
  type Balance = u64;
  type OnFreeBalanceZero = ();
  type OnNewAccount = ();
  type Event = Event;
  type TransferPayment = ();
  type DustRemoval = ();
  type ExistentialDeposit = ExistentialDeposit;
  type TransferFee = TransferFee;
  type CreationFee = CreationFee;
}

parameter_types! {
  pub const MinimumPeriod: u64 = 1;
}

impl timestamp::Trait for Test {
  type Moment = u64;
  type OnTimestampSet = ();
  type MinimumPeriod = MinimumPeriod;
}

parameter_types! {
  pub const ReservationFee: u64 = 2;
  pub const MinLength: usize = 3;
  pub const MaxLength: usize = 16;
  pub const One: u64 = 1;
}

impl Trait for Test {
  type Event = Event;
}

const EOS_ADDRESS: &[u8; 12] = b"praqianchang";
const BTC_ADDRESS: &[u8; 34] = b"1N75dvASxn1CCjaeguyqvwXLXJun9e54mM";
const ETH_ADDRESS: &[u8; 40] = b"cb222a32df146ef7e3ac63725dad0fd978d33ce2";

type DidModule = Module<Test>;
type System = system::Module<Test>;
type Balances = balances::Module<Test>;
type Timestamp = timestamp::Module<Test>;

// This function basically just builds a genesis storage key/value store according to
// our desired mockup.
fn new_test_ext() -> runtime_io::TestExternalities {
  let mut t = system::GenesisConfig::default().build_storage::<Test>().unwrap();
  // We use default for brevity, but you can configure as desired if needed.
  balances::GenesisConfig::<Test> {
    balances: vec![
      (1, 10000),
      (2, 10000),
      (3, 10000),
      (4, 10000),
      (5, 10000),
    ],
    vesting: vec![],
  }.assimilate_storage(&mut t).unwrap();

  GenesisConfig::<Test> {
    genesis_account: 1u64,
    min_deposit: 50,
    base_quota: 250,
    fee_to_previous: 25,
  }.assimilate_storage(&mut t).unwrap();

  t.into()
}

#[test]
fn should_pass_create() {
  new_test_ext().execute_with(|| {
    System::set_block_number(0);

    // genesis account
    assert_ok!(DidModule::create(
      Origin::signed(1),
      b"0x22df4b685df33f070ae6e5ee27f745de078adff099d3a803ec67afe1168acd4f".to_vec(),
      1u64,
      "1".as_bytes().to_vec(),
      H256::zero(),
      Some("f".as_bytes().to_vec()),
      None
    ));

    // second account
    assert_ok!(DidModule::create(
      Origin::signed(1),
      b"0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d".to_vec(),
      2u64,
      "1".as_bytes().to_vec(),
      H256::zero(),
      Some("s".as_bytes().to_vec()),
      Some("f".as_bytes().to_vec())
    ));

  });
}

#[test]
fn should_pass_update() {
  new_test_ext().execute_with(|| {
    System::set_block_number(0);

    assert_ok!(DidModule::create(
      Origin::signed(1),
      b"0x22df4b685df33f070ae6e5ee27f745de078adff099d3a803ec67afe1168acd4f".to_vec(),
      1u64,
      "1".as_bytes().to_vec(),
      H256::zero(),
      Some("f".as_bytes().to_vec()),
      None
    ));

    assert_ok!(DidModule::update(Origin::signed(1), 2u64));
    assert_eq!(Balances::free_balance(&1), 0);
    assert_eq!(Balances::free_balance(&2), 20000);
  });
}

#[test]
fn should_pass_lock() {
  new_test_ext().execute_with(|| {
    System::set_block_number(0);

    assert_ok!(DidModule::create(
      Origin::signed(1),
      b"0x22df4b685df33f070ae6e5ee27f745de078adff099d3a803ec67afe1168acd4f".to_vec(),
      1u64,
      "1".as_bytes().to_vec(),
      H256::zero(),
      Some("init".as_bytes().to_vec()),
      None
    ));

    assert_ok!(DidModule::create(
      Origin::signed(1),
      b"0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d".to_vec(),
      2u64,
      "1".as_bytes().to_vec(),
      H256::zero(),
      Some("w1".as_bytes().to_vec()),
      Some("init".as_bytes().to_vec())
    ));

    assert_noop!(DidModule::lock(Origin::signed(2), 10, 5), "you must lock at least 50 pra first time");

    assert_ok!(DidModule::lock(Origin::signed(2), 100, 5));

    assert_ok!(DidModule::lock(Origin::signed(2), 10, 5));

    assert_eq!(Balances::free_balance(&2), 9890);
    assert_eq!(Balances::free_balance(&1), 10025); // get 25 from locked funds

    assert_ok!(DidModule::create(
      Origin::signed(1),
      b"0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48".to_vec(),
      3u64,
      "1".as_bytes().to_vec(),
      H256::zero(),
      Some("w2".as_bytes().to_vec()),
      Some("w1".as_bytes().to_vec())
    ));

    assert_ok!(DidModule::lock(Origin::signed(3), 100, 5));

    assert_ok!(DidModule::create(
      Origin::signed(1),
      b"0x306721211d5404bd9da88e0204360a1a9ab8b87c66c1bc2fcdd37f3c2222cc20".to_vec(),
      4u64,
      "1".as_bytes().to_vec(),
      H256::zero(),
      Some("w3".as_bytes().to_vec()),
      Some("w2".as_bytes().to_vec())
    ));

    assert_ok!(DidModule::lock(Origin::signed(4), 100, 5));

    let did_hash = DidModule::identity(&4);
    let metadata = DidModule::metadata(did_hash);
    println!("metadata4 is{:?}", metadata);

  });
}

#[test]
fn should_pass_unlock() {
  new_test_ext().execute_with(|| {

    assert_ok!(DidModule::create(
      Origin::signed(1),
      b"0x22df4b685df33f070ae6e5ee27f745de078adff099d3a803ec67afe1168acd4f".to_vec(),
      1u64,
      "1".as_bytes().to_vec(),
      H256::zero(),
      Some("f".as_bytes().to_vec()),
      None
    ));

    assert_ok!(DidModule::create(
      Origin::signed(1),
      b"0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d".to_vec(),
      2u64,
      "1".as_bytes().to_vec(),
      H256::zero(),
      Some("s".as_bytes().to_vec()),
      Some("f".as_bytes().to_vec())
    ));

    Timestamp::set_timestamp(42);
    assert_ok!(DidModule::lock(Origin::signed(2), 100, 5));

    Timestamp::set_timestamp(50);
    assert_ok!(DidModule::unlock(Origin::signed(2), 10));

    assert_eq!(Balances::free_balance(&2), 9910);
  });
}

#[test]
fn should_pass_transfer() {
  new_test_ext().execute_with(|| {
    System::set_block_number(1);

    assert_ok!(DidModule::create(
      Origin::signed(1),
      b"0x22df4b685df33f070ae6e5ee27f745de078adff099d3a803ec67afe1168acd4f".to_vec(),
      1u64,
      "1".as_bytes().to_vec(),
      H256::zero(),
      Some("f".as_bytes().to_vec()),
      None
    ));

    assert_ok!(DidModule::create(
      Origin::signed(1),
      b"0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d".to_vec(),
      2u64,
      "1".as_bytes().to_vec(),
      H256::zero(),
      Some("s".as_bytes().to_vec()),
      Some("f".as_bytes().to_vec())
    ));

    let memo =b"transfer test";
    assert_ok!(DidModule::transfer(
      Origin::signed(2), 
      DidModule::identity(&1), 
      100, 
      memo.to_vec()
    ));

    let events = System::events();
    let from_did = DidModule::identity(&2);
    assert_eq!(
      events[events.len() - 1],
      EventRecord {
          phase: Phase::ApplyExtrinsic(0),
          event: Event::did(RawEvent::Transfered(from_did, DidModule::identity(&1), 100, memo.to_vec())),
          topics: vec![],
      }
    );

    assert_eq!(Balances::free_balance(&2), 9900);
    assert_eq!(Balances::free_balance(&1), 10100);

    assert_ok!(DidModule::lock(Origin::signed(2), 100, 5));
    assert_eq!(Balances::free_balance(&1), 10125);

    assert_ok!(DidModule::create(
      Origin::signed(1),
      b"0x5e9c79234b5e55348fc60f38b28c2cc60d8bb4bd2862eae2179a05ec39e62658".to_vec(),
      3u64,
      "1".as_bytes().to_vec(),
      H256::zero(),
      Some("n".as_bytes().to_vec()),
      Some("s".as_bytes().to_vec())
    ));

    // test ads fee split
    assert_ok!(DidModule::transfer(
      Origin::signed(1), 
      DidModule::identity(&3), 
      1000, 
      b"ads fee".to_vec()
    ));
    assert_eq!(Balances::free_balance(&3), 10800);
    assert_eq!(Balances::free_balance(&2), 10000);
  });
}

#[test]
fn should_pass_add_external_address() {
  new_test_ext().execute_with(|| {
    System::set_block_number(0);

    assert_ok!(DidModule::create(
      Origin::signed(1),
      b"0x22df4b685df33f070ae6e5ee27f745de078adff099d3a803ec67afe1168acd4f".to_vec(),
      1u64,
      "1".as_bytes().to_vec(),
      H256::zero(),
      Some("f".as_bytes().to_vec()),
      None
    ));

    assert_ok!(DidModule::add_external_address(Origin::signed(1), b"eos".to_vec(), EOS_ADDRESS.to_vec()));
    assert_ok!(DidModule::add_external_address(Origin::signed(1), b"eth".to_vec(), ETH_ADDRESS.to_vec()));
    assert_ok!(DidModule::add_external_address(Origin::signed(1), b"btc".to_vec(), BTC_ADDRESS.to_vec()));
  });
}

#[test]
fn should_pass_set_group_name() {
  new_test_ext().execute_with(|| {
    System::set_block_number(0);

    assert_ok!(DidModule::create(
      Origin::signed(1),
      b"0x22df4b685df33f070ae6e5ee27f745de078adff099d3a803ec67afe1168acd4f".to_vec(),
      1u64,
      "1".as_bytes().to_vec(),
      H256::zero(),
      Some("f".as_bytes().to_vec()),
      None
    ));

    assert_ok!(DidModule::create(
      Origin::signed(1),
      b"0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d".to_vec(),
      2u64,
      "1".as_bytes().to_vec(),
      H256::zero(),
      Some("s".as_bytes().to_vec()),
      Some("f".as_bytes().to_vec())
    ));

    assert_ok!(DidModule::lock(Origin::signed(2), 100, 5));
    assert_ok!(DidModule::set_group_name(Origin::signed(2), b"btc group".to_vec()));

  });
}
