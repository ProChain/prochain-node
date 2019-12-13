#![cfg(test)]

use super::*;

use support::{assert_ok, assert_noop, impl_outer_origin, impl_outer_event, parameter_types};
use primitives::H256;
// The testing primitives are very useful for avoiding having to work with signatures
// or public keys. `u64` is used as the `AccountId` and no `Signature`s are required.
use sp_runtime::{
  Perbill, testing::Header, traits::{BlakeTwo256, IdentityLookup},
};

impl_outer_origin! {
  pub enum Origin for Test {}
}

mod ads {
  pub use super::super::*;
}

impl_outer_event! {
  pub enum Event for Test {
    did<T>, ads<T>, balances<T>,
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

impl did::Trait for Test {
  type Event = Event;
}

impl Trait for Test {
  type Event = Event;
}

type AdsModule = Module<Test>;
type Balances = balances::Module<Test>;
type DidModule = did::Module<Test>;

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
    ],
    vesting: vec![],
  }.assimilate_storage(&mut t).unwrap();

  did::GenesisConfig::<Test> {
    genesis_account: 1u64,
    min_deposit: 50,
    base_quota: 250,
    fee_to_previous: 25,
  }.assimilate_storage(&mut t).unwrap();

  GenesisConfig::<Test> {
    contract: 2u64,
    min_deposit: 500,
  }.assimilate_storage(&mut t).unwrap();

  t.into()
}

fn prepare_dids_for_test() {
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

  // first account
  assert_ok!(DidModule::create(
    Origin::signed(1),
    b"0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d".to_vec(),
    2u64,
    "1".as_bytes().to_vec(),
    H256::zero(),
    Some("s".as_bytes().to_vec()),
    Some("f".as_bytes().to_vec())
  ));

  // lock funds
  assert_ok!(DidModule::lock(Origin::signed(2), 1000, 5));

  // second account
  assert_ok!(DidModule::create(
    Origin::signed(1),
    b"0x5e9c79234b5e55348fc60f38b28c2cc60d8bb4bd2862eae2179a05ec39e62658".to_vec(),
    3u64,
    "1".as_bytes().to_vec(),
    H256::zero(),
    Some("n".as_bytes().to_vec()),
    Some("s".as_bytes().to_vec())
  ));
}

#[test]
fn should_pass_publish() {
  new_test_ext().execute_with(|| {
    prepare_dids_for_test();

    // publish ads
    assert_ok!(AdsModule::publish(
      Origin::signed(3),
      b"huawei".to_vec(),
      b"p20 pro".to_vec(),
      1000,
      1,
      30
    ));

    assert_eq!(Balances::free_balance(&3), 9000);
    assert_eq!(Balances::free_balance(&2), 10000);

    let did = DidModule::identity(3);
    assert_eq!(AdsModule::ads_records(did), AdsMetadata {
      advertiser: b"huawei".to_vec(),
      topic: b"p20 pro".to_vec(),
      total_amount: 1000,
      surplus: 1000,
      gas_fee_used: 0,
      single_click_fee: 1,
      create_time: 0,
      period: 30,
    });

  });
}

#[test]
fn less_than_min_deposit_should_not_pass() {
  new_test_ext().execute_with(|| {
    prepare_dids_for_test();

    // publish ads
    assert_noop!(AdsModule::publish(
      Origin::signed(3),
      b"huawei".to_vec(),
      b"p20 pro".to_vec(),
      100,
      1,
      30
    ), "min deposit 500 pra");

    assert_eq!(Balances::free_balance(&3), 10000);
    assert_eq!(Balances::free_balance(&2), 9000);

  });
}

#[test]
fn should_pass_deposit() {
  new_test_ext().execute_with(|| {
    prepare_dids_for_test();

    // publish ads
    assert_ok!(AdsModule::publish(
      Origin::signed(3),
      b"huawei".to_vec(),
      b"p20 pro".to_vec(),
      1000,
      1,
      30
    ));

    assert_ok!(AdsModule::deposit(
      Origin::signed(3),
      500,
      b"new deposit".to_vec()
    ));

    assert_eq!(Balances::free_balance(&3), 8500);
    assert_eq!(Balances::free_balance(&2), 10500);

  });
}

#[test]
fn should_pass_withdraw() {
  new_test_ext().execute_with(|| {
    prepare_dids_for_test();

    // publish ads
    assert_ok!(AdsModule::publish(
      Origin::signed(3),
      b"huawei".to_vec(),
      b"p20 pro".to_vec(),
      1000,
      1,
      30
    ));

    assert_ok!(AdsModule::withdraw(
      Origin::signed(3),
      200,
      b"withdraw money".to_vec()
    ));

    assert_eq!(Balances::free_balance(&3), 9200);
    assert_eq!(Balances::free_balance(&2), 9800);

  });
}

#[test]
fn should_pass_distribute() {
  new_test_ext().execute_with(|| {
    prepare_dids_for_test();

    // publish ads
    assert_ok!(AdsModule::publish(
      Origin::signed(3),
      b"huawei".to_vec(),
      b"p20 pro".to_vec(),
      1000,
      1,
      30
    ));

    let publisher = DidModule::identity(3);
    let user = DidModule::identity(1);
    assert_ok!(AdsModule::distribute(
      Origin::signed(2),
      publisher,
      user,
      200
    ));

    assert_eq!(Balances::free_balance(&3), 9000);
    assert_eq!(Balances::free_balance(&2), 9800);
    assert_eq!(Balances::free_balance(&1), 10225);

  });
}

#[test]
fn with_no_permission_should_not_pass_distribute() {
  new_test_ext().execute_with(|| {
    prepare_dids_for_test();

    // publish ads
    assert_ok!(AdsModule::publish(
      Origin::signed(3),
      b"huawei".to_vec(),
      b"p20 pro".to_vec(),
      1000,
      1,
      30
    ));

    let publisher = DidModule::identity(3);
    let user = DidModule::identity(1);
    assert_noop!(AdsModule::distribute(
      Origin::signed(3),
      publisher,
      user,
      200
    ), "you have no access to use the funds");

    assert_eq!(Balances::free_balance(&3), 9000);
    assert_eq!(Balances::free_balance(&2), 10000);
    assert_eq!(Balances::free_balance(&1), 10025);

  });
}