#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod wordrush_arena {
    use ink::prelude::vec::Vec;
    use ink::storage::Mapping;

    pub type RoomId = u64;

    #[derive(scale::Decode, scale::Encode, Clone, Debug, PartialEq, Eq)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct Room {
        host: AccountId,
        entry_fee: Balance,
        treasury_fee_bps: u16,
        active: bool,
        settled: bool,
        total_score: u128,
        joined_count: u32,
        claims_count: u32,
        total_pot: Balance,
        reward_pool: Balance,
    }

    #[derive(scale::Decode, scale::Encode, Clone, Debug, PartialEq, Eq)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct JoinReceipt {
        room_id: RoomId,
        player: AccountId,
        amount: Balance,
        index: u32,
    }

    #[derive(scale::Decode, scale::Encode, Clone, Debug, PartialEq, Eq)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct SettlementInfo {
        room_id: RoomId,
        total_score: u128,
        reward_pool: Balance,
    }

    #[derive(scale::Decode, scale::Encode, Debug, PartialEq, Eq)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        RoomNotFound,
        RoomNotActive,
        RoomAlreadySettled,
        AlreadyJoined,
        InvalidPayment,
        NotHost,
        EmptySettlement,
        LengthMismatch,
        DuplicatePlayerInSettlement,
        PlayerNotJoined,
        NoRewardAvailable,
        AlreadyClaimed,
        TransferFailed,
    }

    #[ink(storage)]
    pub struct WordrushArena {
        owner: AccountId,
        treasury_wallet: AccountId,
        next_room_id: RoomId,
        rooms: Mapping<RoomId, Room>,
        joined: Mapping<(RoomId, AccountId), bool>,
        scores: Mapping<(RoomId, AccountId), u128>,
        claimed: Mapping<(RoomId, AccountId), bool>,
    }

    #[ink(event)]
    pub struct RoomCreated {
        #[ink(topic)]
        room_id: RoomId,
        #[ink(topic)]
        host: AccountId,
        entry_fee: Balance,
        treasury_fee_bps: u16,
    }

    #[ink(event)]
    pub struct RoomJoined {
        #[ink(topic)]
        room_id: RoomId,
        #[ink(topic)]
        player: AccountId,
        amount: Balance,
    }

    #[ink(event)]
    pub struct RoomSettled {
        #[ink(topic)]
        room_id: RoomId,
        total_score: u128,
        reward_pool: Balance,
    }

    #[ink(event)]
    pub struct RewardClaimed {
        #[ink(topic)]
        room_id: RoomId,
        #[ink(topic)]
        player: AccountId,
        amount: Balance,
    }

    impl WordrushArena {
        #[ink(constructor)]
        pub fn new(treasury_wallet: AccountId) -> Self {
            let caller = Self::env().caller();
            Self {
                owner: caller,
                treasury_wallet,
                next_room_id: 1,
                rooms: Mapping::default(),
                joined: Mapping::default(),
                scores: Mapping::default(),
                claimed: Mapping::default(),
            }
        }

        #[ink(message)]
        pub fn create_room(
            &mut self,
            entry_fee: Balance,
            treasury_fee_bps: u16,
        ) -> Result<RoomId, Error> {
            let caller = self.env().caller();
            let room_id = self.next_room_id;
            self.next_room_id = self.next_room_id.saturating_add(1);

            let room = Room {
                host: caller,
                entry_fee,
                treasury_fee_bps,
                active: true,
                settled: false,
                total_score: 0,
                joined_count: 0,
                claims_count: 0,
                total_pot: 0,
                reward_pool: 0,
            };

            self.rooms.insert(room_id, &room);
            self.env().emit_event(RoomCreated {
                room_id,
                host: caller,
                entry_fee,
                treasury_fee_bps,
            });
            Ok(room_id)
        }

        #[ink(message, payable)]
        pub fn join_room(&mut self, room_id: RoomId) -> Result<JoinReceipt, Error> {
            let caller = self.env().caller();
            let transferred = self.env().transferred_value();
            let mut room = self.rooms.get(room_id).ok_or(Error::RoomNotFound)?;

            if !room.active || room.settled {
                return Err(Error::RoomNotActive);
            }

            if self.joined.get((room_id, caller)).unwrap_or(false) {
                return Err(Error::AlreadyJoined);
            }

            if transferred != room.entry_fee {
                return Err(Error::InvalidPayment);
            }

            self.joined.insert((room_id, caller), &true);
            room.joined_count = room.joined_count.saturating_add(1);
            room.total_pot = room.total_pot.saturating_add(transferred);
            self.rooms.insert(room_id, &room);

            self.env().emit_event(RoomJoined {
                room_id,
                player: caller,
                amount: transferred,
            });

            Ok(JoinReceipt {
                room_id,
                player: caller,
                amount: transferred,
                index: room.joined_count,
            })
        }

        #[ink(message)]
        pub fn settle_room(
            &mut self,
            room_id: RoomId,
            players: Vec<AccountId>,
            scores: Vec<u128>,
        ) -> Result<SettlementInfo, Error> {
            let caller = self.env().caller();
            let mut room = self.rooms.get(room_id).ok_or(Error::RoomNotFound)?;

            if caller != room.host && caller != self.owner {
                return Err(Error::NotHost);
            }
            if !room.active || room.settled {
                return Err(Error::RoomAlreadySettled);
            }
            if players.is_empty() {
                return Err(Error::EmptySettlement);
            }
            if players.len() != scores.len() {
                return Err(Error::LengthMismatch);
            }

            let mut total_score: u128 = 0;
            for (idx, player) in players.iter().enumerate() {
                for prev in players.iter().take(idx) {
                    if prev == player {
                        return Err(Error::DuplicatePlayerInSettlement);
                    }
                }

                if !self.joined.get((room_id, *player)).unwrap_or(false) {
                    return Err(Error::PlayerNotJoined);
                }

                let score = scores[idx];
                total_score = total_score.saturating_add(score);
                self.scores.insert((room_id, *player), &score);
            }

            room.total_score = total_score;
            room.active = false;
            room.settled = true;

            let treasury_cut = room
                .total_pot
                .saturating_mul(room.treasury_fee_bps as u128)
                / 10_000u128;
            let reward_pool = room.total_pot.saturating_sub(treasury_cut);
            room.reward_pool = reward_pool;
            self.rooms.insert(room_id, &room);

            if treasury_cut > 0 {
                if self.env().transfer(self.treasury_wallet, treasury_cut).is_err() {
                    return Err(Error::TransferFailed);
                }
            }

            self.env().emit_event(RoomSettled {
                room_id,
                total_score,
                reward_pool,
            });

            Ok(SettlementInfo {
                room_id,
                total_score,
                reward_pool,
            })
        }

        #[ink(message)]
        pub fn claim_reward(&mut self, room_id: RoomId) -> Result<Balance, Error> {
            let caller = self.env().caller();
            let mut room = self.rooms.get(room_id).ok_or(Error::RoomNotFound)?;

            if !room.settled {
                return Err(Error::RoomNotActive);
            }
            if !self.joined.get((room_id, caller)).unwrap_or(false) {
                return Err(Error::PlayerNotJoined);
            }
            if self.claimed.get((room_id, caller)).unwrap_or(false) {
                return Err(Error::AlreadyClaimed);
            }

            let player_score = self.scores.get((room_id, caller)).unwrap_or(0);
            if player_score == 0 || room.total_score == 0 {
                return Err(Error::NoRewardAvailable);
            }

            let amount = room
                .reward_pool
                .saturating_mul(player_score as u128)
                / room.total_score as u128;
            if amount == 0 {
                return Err(Error::NoRewardAvailable);
            }

            self.claimed.insert((room_id, caller), &true);
            room.claims_count = room.claims_count.saturating_add(1);
            self.rooms.insert(room_id, &room);

            if self.env().transfer(caller, amount).is_err() {
                return Err(Error::TransferFailed);
            }

            self.env().emit_event(RewardClaimed {
                room_id,
                player: caller,
                amount,
            });

            Ok(amount)
        }

        #[ink(message)]
        pub fn cancel_room(&mut self, room_id: RoomId, players: Vec<AccountId>) -> Result<(), Error> {
            let caller = self.env().caller();
            let mut room = self.rooms.get(room_id).ok_or(Error::RoomNotFound)?;

            if caller != room.host && caller != self.owner {
                return Err(Error::NotHost);
            }
            if !room.active || room.settled {
                return Err(Error::RoomNotActive);
            }

            for player in players.iter() {
                if !self.joined.get((room_id, *player)).unwrap_or(false) {
                    continue;
                }
                if self.claimed.get((room_id, *player)).unwrap_or(false) {
                    continue;
                }

                self.claimed.insert((room_id, *player), &true);
                if self.env().transfer(*player, room.entry_fee).is_err() {
                    return Err(Error::TransferFailed);
                }
            }

            room.active = false;
            room.settled = true;
            self.rooms.insert(room_id, &room);
            Ok(())
        }

        #[ink(message)]
        pub fn get_room(&self, room_id: RoomId) -> Option<Room> {
            self.rooms.get(room_id)
        }

        #[ink(message)]
        pub fn get_player_score(&self, room_id: RoomId, player: AccountId) -> u128 {
            self.scores.get((room_id, player)).unwrap_or(0)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn create_room_works() {
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            let mut contract = WordrushArena::new(accounts.bob);
            let room_id = contract.create_room(1_000_000_000_000_000, 1000).unwrap();
            let room = contract.get_room(room_id).unwrap();
            assert_eq!(room.entry_fee, 1_000_000_000_000_000);
            assert_eq!(room.treasury_fee_bps, 1000);
            assert!(room.active);
        }

        #[ink::test]
        fn join_room_rejects_wrong_amount() {
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            let mut contract = WordrushArena::new(accounts.bob);
            let room_id = contract.create_room(100, 1000).unwrap();

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.charlie);
            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(99);

            let result = contract.join_room(room_id);
            assert_eq!(result, Err(Error::InvalidPayment));
        }
    }
}

