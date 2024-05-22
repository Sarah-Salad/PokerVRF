use schnorrkel::{vrf::{VRFInOut, VRFOutput, VRFPreOut, VRFProof, VRFProofBatchable}, *};
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

    fn sign(&self, current_card: u32) -> VRFInOut {
        let context = signing_context(b"example context");
        let card_as_string = current_card.to_string();
        // Message to be signed
        let message: &[u8] = &card_as_string.as_bytes();
        // Sign the message
        self.keypair.vrf_sign(context.bytes(message)).0
    }
}

fn main() {
let mut game = Game::new();
let player1 = Player::new();
let player2 = Player::new();

let player1_sign = player1.sign(game.get_card());
let player2_sign = player2.sign(game.get_card());
let winner = match player1_sign.cmp(&player2_sign) {
    std::cmp::Ordering::Less => "Player 2",
    _ => "Player 1",
};
println!("The winner is {}", winner);

}
