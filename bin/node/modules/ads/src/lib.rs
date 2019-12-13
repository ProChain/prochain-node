#![cfg_attr(not(feature = "std"), no_std)]

mod tests;

use codec::{Decode, Encode};
use rstd::vec::Vec;
use support::{
	decl_event, decl_module, decl_storage, ensure,
};
use sp_runtime::traits::{Zero, CheckedSub, CheckedAdd};
use system::ensure_signed;

pub trait Trait: balances::Trait + timestamp::Trait + did::Trait {
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Encode, Decode, Default, Clone, PartialEq)]
pub struct AdsMetadata<Balance, Moment> {
	advertiser: Vec<u8>,
  topic: Vec<u8>,
  total_amount: Balance,
  surplus: Balance,
  gas_fee_used: Balance,
  single_click_fee: Balance,
  create_time: Moment,
  period: Moment,
}

decl_storage! {
    trait Store for Module<T: Trait> as AdsModule {
			Contract get(contract) config(): T::AccountId;
			MinDeposit get(min_deposit) config(): T::Balance;

			AdsRecords get(ads_records): map T::Hash => AdsMetadata<T::Balance, T::Moment>;
			AllAdsCount get(all_ads_count): u64;
    }
}

decl_event! {
  pub enum Event<T>
  where
    <T as system::Trait>::Hash,
    <T as balances::Trait>::Balance,
		<T as timestamp::Trait>::Moment,
    {
      Published(Hash, Hash, Balance),
			Deposited(Hash, Hash, Balance),
			Withdrawl(Hash, Balance),
			Distributed(Hash, Hash, Balance),
			AdsUpdated(Hash, Balance, Moment),
    }
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event() = default;
    
    fn publish(origin, name: Vec<u8>, topic: Vec<u8>, total_amount: T::Balance, single_click_fee: T::Balance, period: T::Moment) {
      let sender = ensure_signed(origin)?;

      ensure!(<did::Identity<T>>::exists(sender.clone()), "did does not exists");
			ensure!(total_amount >= Self::min_deposit(), "min deposit 500 pra");

      let from_did = <did::Module<T>>::identity(sender);
      let create_time = <timestamp::Module<T>>::get();

			let contract = <did::Module<T>>::identity(Self::contract());
			<did::Module<T>>::transfer_by_did(from_did, contract, total_amount, "开户广告费".as_bytes().to_vec())?;

      let ads_metadata = AdsMetadata {
        advertiser: name,
        topic,
        total_amount,
        surplus: total_amount,
        gas_fee_used: Zero::zero(),
        single_click_fee,
        create_time,
        period
      };

			<AdsRecords<T>>::insert(from_did, ads_metadata);

			// update count
			let all_ads_count = Self::all_ads_count();
			let new_count = all_ads_count.checked_add(1)
					.ok_or("Overflow adding new ads")?;
			<AllAdsCount>::put(new_count);

			Self::deposit_event(RawEvent::Published(from_did, contract, total_amount));
    }

    fn deposit(origin, value: T::Balance, memo: Vec<u8>) {
      let sender = ensure_signed(origin)?;

			let from_did = <did::Module<T>>::identity(sender.clone());

      ensure!(<did::Identity<T>>::exists(sender), "did does not exists");
			ensure!(value >= Self::min_deposit(), "min deposit 100 pra");
			ensure!(<AdsRecords<T>>::exists(from_did), "you haven't published ads");
			
			let contract_did = <did::Module<T>>::identity(Self::contract());

			<did::Module<T>>::transfer_by_did(from_did, contract_did, value, memo)?;

			// update ads records
			let mut ads_metadata = Self::ads_records(from_did);
			ads_metadata.total_amount = ads_metadata.total_amount.checked_add(&value).ok_or("overflow")?;
			ads_metadata.surplus = ads_metadata.surplus.checked_add(&value).ok_or("overflow")?;

			<AdsRecords<T>>::insert(from_did, ads_metadata);

			Self::deposit_event(RawEvent::Deposited(from_did, contract_did, value));
    }

    fn withdraw(origin, value: T::Balance, memo: Vec<u8>) {
      let sender = ensure_signed(origin)?;

      let from_did = <did::Module<T>>::identity(sender.clone());

      ensure!(<did::Identity<T>>::exists(sender), "did does not exists");
			ensure!(<AdsRecords<T>>::exists(from_did), "you haven't published ads");
			
			let mut ads_metadata = Self::ads_records(from_did);

			ensure!(ads_metadata.surplus >= value, "withdrawl money is larger than your surplus");

			let contract_did = <did::Module<T>>::identity(Self::contract());

			<did::Module<T>>::transfer_by_did(contract_did, from_did, value, memo)?;

			// update ads metadata
			ads_metadata.total_amount = ads_metadata.total_amount.checked_sub(&value).ok_or("overflow")?;
			ads_metadata.surplus = ads_metadata.surplus.checked_sub(&value).ok_or("overflow")?;

			<AdsRecords<T>>::insert(from_did, ads_metadata);

			Self::deposit_event(RawEvent::Withdrawl(from_did, value));
    }

		fn distribute(origin, publisher: T::Hash, user: T::Hash, value: T::Balance) {
			let sender = ensure_signed(origin)?;

			ensure!(sender == Self::contract(), "you have no access to use the funds");

			let contract_did = <did::Module<T>>::identity(Self::contract());

			ensure!(<AdsRecords<T>>::exists(publisher), "the account hadn't published ads yet");
      ensure!(<did::Metadata<T>>::exists(user), "the user does not have did yet");
			
			let mut ads_metadata = Self::ads_records(publisher);

			ensure!(ads_metadata.surplus >= value, "your surplus is not enough");

			<did::Module<T>>::transfer_by_did(contract_did, user, value, "看广告收益".as_bytes().to_vec())?;

			// update ads metadata
			ads_metadata.surplus = ads_metadata.surplus.checked_sub(&value).ok_or("overflow")?;

			<AdsRecords<T>>::insert(publisher, ads_metadata);

			Self::deposit_event(RawEvent::Distributed(publisher, user, value));
		}

		fn update_ads(origin, single_click_fee: T::Balance, period: T::Moment) {
			let sender = ensure_signed(origin)?;

			let from_did = <did::Module<T>>::identity(sender);

			ensure!(<AdsRecords<T>>::exists(from_did), "you haven't published ads");

			// update ads records
			let mut ads_metadata = Self::ads_records(from_did);
			ads_metadata.single_click_fee = single_click_fee;
			ads_metadata.period = period;

			<AdsRecords<T>>::insert(from_did, ads_metadata);

			Self::deposit_event(RawEvent::AdsUpdated(from_did, single_click_fee, period));
		}
	}
}
