#![cfg_attr(not(feature = "std"), no_std)]


pub use pallet::*;


#[frame_support::pallet]
pub mod pallet{
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        #[pallet::constant]
        type MaxClaimLength: Get<u32>;
    }



    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    pub type Proofs<T:Config> = StorageMap<_,Blake2_128Concat,BoundedVec<u8,T::MaxClaimLength>,(T::AccountId,BlockNumberFor<T>)>;


    #[pallet::event]
    #[pallet::generate_deposit(pub(crate) fn deposit_event)]
    pub enum Event<T: Config> {
        ClaimCreated(T::AccountId, BoundedVec<u8, T::MaxClaimLength>),
        ClaimRevoked(T::AccountId, BoundedVec<u8, T::MaxClaimLength>),
    }

    #[pallet::call]
    impl<T:Config> Pallet<T> {

        #[pallet::weight({0})]
        #[pallet::call_index(0)]
        pub fn create_claim(origin:OriginFor<T>,claim:BoundedVec<u8,T::MaxClaimLength>)->DispatchResult{
            let sender = ensure_signed(origin)?;
            ensure!(!Proofs::<T>::contains_key(&claim),Error::<T>::ProofAlreadySubmitted);
            Proofs::<T>::insert(&claim,(&sender.clone(),frame_system::Pallet::<T>::block_number()));
            Self::deposit_event(Event::ClaimCreated(sender,claim));
            Ok(())
        }
        
        #[pallet::weight({0})]
        #[pallet::call_index(1)]
        pub fn revoke_claim(origin:OriginFor<T>,claim:BoundedVec<u8,T::MaxClaimLength>)->DispatchResult{
            let sender = ensure_signed(origin)?;
            let (owner,_) = Proofs::<T>::get(&claim).ok_or(Error::<T>::ProofNotSubmitted)?;
            ensure!(owner == sender,Error::<T>::NotClaimOwner);
            Proofs::<T>::remove(&claim);
            Self::deposit_event(Event::ClaimRevoked(sender,claim));
            Ok(())
        }

    }
    

    #[pallet::error]
    pub enum Error<T>{
        ProofAlreadySubmitted,
        ProofNotSubmitted,
        ProofTooLong,
        NotClaimOwner,
    }

}