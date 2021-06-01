#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://substrate.dev/docs/en/knowledgebase/runtime/frame>

pub use pallet::*;
#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::DispatchResult, pallet_prelude::*,ensure };
	use frame_system::{pallet_prelude::*,ensure_signed};
	

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://substrate.dev/docs/en/knowledgebase/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn something)]
	// Learn more about declaring storage items:
	// https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items
	pub type Something<T> = StorageValue<_, u32>;
	
	#[pallet::type_value]
	pub(super) fn TotalSupply<T: Config>() -> u64 { 21000000 }

	#[pallet::storage]
	#[pallet::getter(fn is_init)]
	pub(super) type Init<T:Config> = StorageValue<_,bool>;

	#[pallet::storage]
	#[pallet::getter(fn get_balance)]
	pub(super) type Balances<T:Config> = StorageMap<_,Blake2_128Concat,T::AccountId,u64>;


	// Pallets use events to inform users when important changes are made.
	// https://substrate.dev/docs/en/knowledgebase/runtime/events
	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		/// Token was initialized by user
        Initialized(T::AccountId),
        /// Tokens successfully transferred between users
        Transfer(T::AccountId, T::AccountId, u64), // (from, to, value)
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Attempted to initialize the token after it had already been initialized.
        AlreadyInitialized,
        /// Attempted to transfer more funds than were available
        InsufficientFunds,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T:Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn transfer(_origin: OriginFor<T>, to: T::AccountId, value: u64) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://substrate.dev/docs/en/knowledgebase/runtime/origin
			let sender = ensure_signed(_origin)?;
			let sender_balance = Self::get_balance(&sender);
			let receiver_balance = Self::get_balance(&to);
			let updated_from_balance = sender_balance.unwrap().checked_sub(value).ok_or(<Error<T>>::InsufficientFunds)?;
			
			let updated_to_balance = receiver_balance.unwrap().checked_add(value).expect("Entire supply fits in u64; qed");
			// Calculate new balances
			//let updated_from_balance = sender_balance.checked_add(value).ok_or(<Error<T>>::InsufficientFunds)?;
			//let updated_to_balance = receiver_balance.checked_sub(value).expect("Entire supply fits in u64; qed");
			// Write new balances to storage
			//<Balances<T>>::insert(&sender, updated_from_balance);
			//<Balances<T>>::insert(&to, updated_to_balance);
			Balances::<T>::insert(&sender, updated_from_balance);
			Balances::<T>::insert(&to, updated_to_balance);
			
			//Self::deposit_event(RawEvent::Transfer(sender, to, value));
			

			Ok(())
		}

		/// An example init
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn init(origin: OriginFor<T>) ->  DispatchResult{
			let sender = ensure_signed(origin)?;
			ensure!(Self::is_init() == None , Error::<T>::AlreadyInitialized);

			<Balances<T>>::insert(sender, TotalSupply::<T>::get());

			Init::<T>::put(true);

			Ok(())
		}



	}
}
