use rand::prelude::*;
use crate::common::types::{Amount, CoinString, Error, IntoErr, Timeout};
use crate::common::standard_coin::ChiaIdentity;
use crate::channel_handler::game::Game;
use crate::channel_handler::types::ChannelHandlerEnv;

use crate::tests::channel_handler::ChannelHandlerGame;
use crate::tests::simulator::Simulator;

pub fn new_channel_handler_game<R: Rng>(
    simulator: &Simulator,
    env: &mut ChannelHandlerEnv<R>,
    game: &Game,
    identities: &[ChiaIdentity; 2],
    contributions: [Amount; 2],
) -> Result<(ChannelHandlerGame, [CoinString; 2]), Error> {
    let mut party = ChannelHandlerGame::new(
        env,
        game.id.clone(),
        contributions.clone()
    );

    // Get at least one coin for the first identity
    simulator.farm_block(&identities[0].puzzle_hash);
    // Get at least one coin for the second identity
    simulator.farm_block(&identities[1].puzzle_hash);

    let get_sufficient_coins = |i: usize| -> Result<Vec<CoinString>, Error> {
        Ok(simulator.get_my_coins(&identities[i].puzzle_hash).into_gen()?.into_iter().filter(|c| {
            if let Some((_, _, amt)) = c.to_parts() {
                return amt >= contributions[i].clone();
            }
            false
        }).collect())
    };
    let player_coins: [Vec<CoinString>; 2] = [
        get_sufficient_coins(0)?,
        get_sufficient_coins(1)?
    ];

    let init_results = party.handshake(env, &player_coins[0][0].to_coin_id())?;

    let _finish_hs_result1 = party
        .finish_handshake(
            env,
            1,
            &init_results[0].my_initial_channel_half_signature_peer,
        )
        .expect("should finish handshake");
    let _finish_hs_result2 = party
        .finish_handshake(
            env,
            0,
            &init_results[1].my_initial_channel_half_signature_peer,
        )
        .expect("should finish handshake");

    let amount = contributions[0].clone() + contributions[1].clone();
    let timeout = Timeout::new(10);

    let (our_game_start, their_game_start) = game.symmetric_game_starts(
        &game.id,
        &contributions[0].clone(),
        &contributions[1].clone(),
        &timeout
    );
    let start_potato = party.player(0).ch.send_potato_start_game(
        env,
        &[our_game_start]
    )?;

    party.player(1).ch.received_potato_start_game(
        env,
        &start_potato,
        &[their_game_start]
    )?;

    Ok((party, [player_coins[0][0].clone(), player_coins[1][0].clone()]))
}
