#![feature(prelude_import)]
#![no_std]

#[macro_use]
extern crate sgx_tstd as std;

extern crate libcontract_common;
#[macro_use]
extern crate libcontract_trusted;

#[macro_use]
// extern crate dummy_api;

extern crate protobuf;

#[allow(unused)]
#[prelude_import]
use std::prelude::v1::*;

// use dummy_api::{HelloWorldRequest, HelloWorldResponse};

use libcontract_common::ContractError;

create_enclave_api!();

fn hello_world(request: &HelloWorldRequest) -> Result<HelloWorldResponse, ContractError> {
    println!("hello world called");

    let mut response = HelloWorldResponse::new();
    response.set_world(format!("enclave says {}", request.hello));

    Ok(response)
}


pub struct Poll<'a> {

	desc: String,
	open: bool,
	options: Vec<Option>,
	max_votes: u64, // Max votes per party
	parties: Vec<Party>

    // game_id: u64,
    // blind: u64,
    // max_players: u64,
    // time_per_turn: u64,
    // players: Vec<Player>, 
    // on_deck: Vec<Player>,
    // index: HashMap<String, i32>,
    // cards: Vec<Card>,
    // deck: &'a Iter<'a, Card>, 
    // pot: u64,
    // min_bet: u64,
    // dealer: i32,
    // next_player: i32,
    // last_player: i32,
    // stage: GameStage,
    // seed: [u8; 32],
}          

impl<'a> Poll<'a> {

    //Creates a new instance of a poll with all values set to default save for provided parameters
    pub fn new(desc: String, max_votes: u64) {
    	return Ok(Poll {
    		desc,
    		open: false,
    		options: Vec::new(),
    		max_votes,
    		parties: Vec::new()
    	});
    }

    // Add option to poll
    pub fn add_option(option: Option) {
    	self.options.push(option);
    }

    // Add option to poll at index index
    pub fn add_option(option: Option, index: i32) {
    	self.options.insert(index, option);
    }

    // Remove option from poll
    pub fn remove_option(option: Option) {
    	i32 index = -1;
    	for i in 0..len(self.options) {
    		if self.options[i] == option {
    			index = i;
    		}
    	}
    	if index == -1 {
    		// TODO: Handle option not existing
    	}
    	option.remove(index);
    }

    // Remove option at index index from poll
    pub fn remove_option(index: i32) {
    	if index < 0 || index >= len(self.options) {
    		// TODO: Handle invalid index
    	}
    	option.remove(index);
    }

    // Open the poll to responses
    pub fn open_poll() {
    	this.open = true;
    }

    // Close poll from receiving responses
    pub fn close_poll() {
    	this.open = false;
    }

    // Allows a party to join poll
    pub fn join_poll(&mut self, msg_sender: &Address) -> Result<bool, ContractError> {

    }

    //+++++++++++++++++++++++++++++++++++++++++++++++++++++
    // FUNCTIONS TO REQUEST AND FORMAT STATE
    //+++++++++++++++++++++++++++++++++++++++++++++++++++++
    // TODO: this

    /*
    pub fn get_public_state(&mut self) -> Result<PublicState, ContractError> {
        let mut state = PublicState::new();

        state.set_game_id(self.game_id);
        state.set_blind(self.blind);
        state.set_max_players(self.max_players);
        state.set_players(self.serialize_players(&self.players));
        state.set_on_deck(self.serialize_players(&self.on_deck));
        state.set_pot(self.pot);
        state.set_min_bet(self.min_bet);
        state.set_dealer(self.dealer);
        state.set_next_player(self.next_player);
        state.set_last_player(self.set_last_player);
        state.set_stage(self.stage);

        Ok(state)
    }

    fn get_player_state(&mut self, msg_sender: &Address) -> Result<PlayerState, ContractError> {
        let player = self.players[0];
        let mut state = PlayerState::new();

        state.set_addr(player.addr.to_string());
        state.set_action(player.action.to_string());
        state.set_cards(serde_cbor::to_vec(&player.cards).expect("Unable to serialize cards."));
        state.set_playing(player.playing);
        state.set_bet(player.bet);
        state.set_balance(player.balance);
        
        Ok(state)
    }

    pub fn from_player_state(state: &PlayerState) -> Player {

    }


    fn serialize_players(&mut self, players: &Vec<Player>) -> Vec<PlayerState> {
        let formatted_players: Vec<PlayerState> = Vec::new();
        for player in players {
            let state = self.get_player_state(&player.addr).unwrap();
            formatted_players.push(state);
        }
        return formatted_players;
    }
    */

}

// Poll option
pub struct Option<'a> {

	text: String,
	num_votes: u64,
	voted_for: Vec<Party>
}

// Party: someone that can vote in the poll
pub struct Party<'a> {

}
