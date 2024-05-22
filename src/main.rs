use schnorrkel::*;
use rand::{Rng, rngs::OsRng};

struct Player {
    keypair: Keypair
}

struct Game {
    current_card: u32
}

impl Game {
    fn new() -> Self {
        Self {
            current_card: 1
        }
    }

    fn get_card(&mut self) -> u32 {
        let card = self.current_card;
        self.current_card += 1;
        card
    }
}

impl Player {
    fn new() -> Self {
        Self {
            keypair: Keypair::generate_with(OsRng)
        }
    }

    fn sign(&self, current_card: u32) -> Signature {
        let context = signing_context(b"example context");
        let card_as_string = current_card.to_string();
        // Message to be signed
        let message: &[u8] = &card_as_string.as_bytes();
        // Sign the message
        self.keypair.sign(context.bytes(message)) 
    }
}

fn main() {
let mut game = Game::new();
let player1 = Player::new();
let player2 = Player::new();
//get first card

player1.sign(game.get_card());
player2.sign(game.get_card());



//Players need to have keypairs - their own secret and public keys. When a player or players draw a card, 
//we need to choose an input for the VRFs of players who draw cards.
//One good way to get an input is for all players to do a commit-reveal and combine the results, h
//owever you could choose whatever technique you'd like.
//Players know their own VRF output (i.e. the cards in their hand), 
//but other players don't until the game calls for them to reveal their card, by publishing a VRF output.

}
