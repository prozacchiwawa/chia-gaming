use clvm_traits::{clvm_curried_args, ToClvm};
use clvm_utils::CurriedProgram;
use rand::prelude::*;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

use clvmr::NodePtr;
use clvm_tools_rs::classic::clvm_tools::binutils::{assemble, disassemble};

use crate::channel_handler::game_handler::GameHandler;
use crate::channel_handler::types::{GameStartInfo, ReadableMove, ValidationProgram};
use crate::common::standard_coin::{read_hex_puzzle, ChiaIdentity};
use crate::common::types::{
    Aggsig, AllocEncoder, Amount, Error, GameID, Node, PrivateKey, PuzzleHash, Sha256tree, Timeout, Puzzle
};
use crate::referee::{RefereeMaker, ValidatorMoveArgs, GameMoveDetails};

pub struct DebugGamePrograms {
    pub my_validation_program: NodePtr,
    pub their_validation_program: NodePtr,
    pub my_turn_handler: GameHandler,
    pub their_turn_handler: GameHandler,
}

pub fn make_debug_game_handler(
    allocator: &mut AllocEncoder,
    identity: &ChiaIdentity,
    amount: &Amount,
    timeout: &Timeout
) -> DebugGamePrograms {
    let debug_game_handler =
        read_hex_puzzle(allocator, "resources/debug_game_handler.hex").expect("should be readable");
    let game_handler_mod_hash = debug_game_handler.sha256tree(allocator);
    let make_curried_game_handler = |my_turn: bool| {
        let aggsig = Aggsig::default();
        CurriedProgram {
            program: debug_game_handler.clone(),
            args: clvm_curried_args!(
                (game_handler_mod_hash.clone(),
                 (debug_game_handler.clone(),
                  (timeout.clone(),
                   (amount.clone(),
                    (my_turn,
                     (((), (aggsig, ())),
                      (identity.puzzle_hash.clone(), ()) // slash info
                     )
                    )
                   )
                  )
                 )
                )
            ),
        }
    };

    let my_turn_handler = GameHandler::my_driver_from_nodeptr(
        make_curried_game_handler(true)
            .to_clvm(allocator)
            .expect("should curry"),
    );
    let my_validation_program = CurriedProgram {
        program: Node(my_turn_handler.to_nodeptr()),
        args: clvm_curried_args!(1337)
    }.to_clvm(allocator).expect("should curry");

    let their_turn_handler = GameHandler::their_driver_from_nodeptr(
        make_curried_game_handler(false)
            .to_clvm(allocator)
            .expect("should curry"),
    );
    let their_validation_program = CurriedProgram {
        program: Node(their_turn_handler.to_nodeptr()),
        args: clvm_curried_args!(1337)
    }.to_clvm(allocator).expect("should curry");

    DebugGamePrograms {
        my_validation_program,
        my_turn_handler,
        their_validation_program,
        their_turn_handler
    }
}

pub struct RefereeTest {
    pub my_identity: ChiaIdentity,
    pub their_identity: ChiaIdentity,

    pub my_referee: RefereeMaker,
    pub their_referee: RefereeMaker,

    pub referee_coin_puzzle: Puzzle,
    pub referee_coin_puzzle_hash: PuzzleHash,
}

impl RefereeTest {
    pub fn new(
        allocator: &mut AllocEncoder,

        my_identity: ChiaIdentity,
        their_identity: ChiaIdentity,

        their_game_handler: GameHandler,

        game_start: &GameStartInfo
    ) -> RefereeTest {
        // Load up the real referee coin.
        let referee_coin_puzzle = read_hex_puzzle(allocator, "onchain/referee.hex").expect("should be readable");
        let referee_coin_puzzle_hash: PuzzleHash = referee_coin_puzzle.sha256tree(allocator);
        let my_referee = RefereeMaker::new(
            allocator,
            referee_coin_puzzle.clone(),
            referee_coin_puzzle_hash.clone(),
            &game_start,
            my_identity.clone(),
            &their_identity.puzzle_hash,
            1,
        )
            .expect("should construct");

        let their_game_start_info = GameStartInfo {
            is_my_turn: !game_start.is_my_turn,
            initial_mover_share: game_start.amount.clone() - game_start.initial_mover_share.clone(),
            game_handler: their_game_handler,
            .. game_start.clone()
        };

        let mut their_referee = RefereeMaker::new(
            allocator,
            referee_coin_puzzle.clone(),
            referee_coin_puzzle_hash.clone(),
            &their_game_start_info,
            their_identity.clone(),
            &my_identity.puzzle_hash,
            1,
        )
            .expect("should construct");


        RefereeTest {
            my_identity,
            their_identity,

            my_referee,
            their_referee,

            referee_coin_puzzle,
            referee_coin_puzzle_hash,
        }
    }
}

#[test]
fn test_referee_smoke() {
    let seed: [u8; 32] = [0; 32];
    let mut rng = ChaCha8Rng::from_seed(seed);
    let mut allocator = AllocEncoder::new();

    // Generate keys and puzzle hashes.
    let my_private_key: PrivateKey = rng.gen();
    let my_identity = ChiaIdentity::new(&mut allocator, my_private_key).expect("should generate");

    let their_private_key: PrivateKey = rng.gen();
    let their_identity = ChiaIdentity::new(&mut allocator, their_private_key).expect("should generate");

    let amount = Amount::new(100);
    let timeout = Timeout::new(1000);

    let debug_game = make_debug_game_handler(
        &mut allocator,
        &my_identity,
        &amount,
        &timeout
    );
    let init_state =
        assemble(
            allocator.allocator(),
            "(0 . 0)"
        ).expect("should assemble");
    let initial_validation_program = ValidationProgram::new(
        &mut allocator,
        debug_game.my_validation_program,
    );

    let amount = Amount::new(100);
    let game_start_info = GameStartInfo {
        game_id: GameID::from_bytes(b"test"),
        amount: amount.clone(),
        game_handler: debug_game.my_turn_handler,
        timeout: timeout.clone(),
        is_my_turn: true,
        initial_validation_program,
        initial_state: init_state,
        initial_move: vec![],
        initial_max_move_size: 0,
        initial_mover_share: Amount::default(),
    };

    let their_validation_program_hash =
        Node(debug_game.their_validation_program).sha256tree(&mut allocator);

    let mut reftest = RefereeTest::new(
        &mut allocator,
        my_identity,
        their_identity,
        debug_game.their_turn_handler,
        &game_start_info,
    );

    let readable_move = assemble(allocator.allocator(), "(0 . 0)").expect("should assemble");
    let my_move_wire_data = reftest.my_referee
        .my_turn_make_move(
            &mut rng,
            &mut allocator,
            &ReadableMove::from_nodeptr(readable_move),
        )
        .expect("should move");

    assert!(my_move_wire_data.details.move_made.is_empty());
    let mut off_chain_slash_gives_error = reftest.my_referee.clone();
    let their_move_result = off_chain_slash_gives_error.their_turn_move_off_chain(
        &mut allocator,
        &GameMoveDetails {
            move_made: vec![1],
            validation_info_hash: my_move_wire_data.details.validation_info_hash.clone(),
            max_move_size: 100,
            mover_share: Amount::default(),
        },
    );
    eprintln!("their move result {their_move_result:?}");
    if let Err(Error::StrErr(s)) = their_move_result {
        assert!(s.contains("slash"));
        assert!(s.contains("off chain"));
    } else {
        assert!(false);
    }

    let their_move_local_update = reftest.their_referee.their_turn_move_off_chain(
        &mut allocator,
        &my_move_wire_data.details,
    ).expect("should move");

    eprintln!("their_move_wire_data {their_move_local_update:?}");

    let mover_puzzle = reftest.my_identity.puzzle.clone();

    let validator_move_args = ValidatorMoveArgs {
        game_move: my_move_wire_data.details.clone(),
        mover_puzzle: reftest.my_identity.puzzle.to_program(),
        solution: reftest.my_identity.standard_solution(
            &mut allocator,
            &[(reftest.my_identity.puzzle_hash.clone(), Amount::default())]
        ).expect("should create")
    };

    let validator_result = reftest.their_referee.run_validator_for_their_move(
        &mut allocator,
        &validator_move_args
    ).expect("should run");

    assert!(!reftest.my_referee.is_my_turn);
    let their_move_result = reftest.my_referee
        .their_turn_move_off_chain(
            &mut allocator,
            &my_move_wire_data.details,
        )
        .expect("should run");
    assert_eq!(their_move_result.message, b"message data");
    assert_eq!(
        disassemble(allocator.allocator(), their_move_result.readable_move, None),
        "(())"
    );
    assert!(reftest.my_referee.is_my_turn);
}
