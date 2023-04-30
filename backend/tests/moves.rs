use std::thread;

use backend::{
    board_setup::models::{Board, FenNotation},
    move_generator::models::Moves,
    move_register::models::MoveType,
};

fn start_test(board: &Board, max_depth: u8) -> (u64, u64, u64, u64) {
    let Moves(move_set) = Moves::get_all_moves(&board, board.turn);
    let mut res = (0, 0, 0, 0);
    let mut threads = Vec::new();
    let mut batches = vec![vec![]; 4];
    let batches_len = batches.len();
    for (i, m) in move_set.into_iter().enumerate() {
        batches[i % batches_len].push(m);
    }
    for batch in batches {
        let new_board = *board;
        let part_res = thread::spawn(move || {
            let mut res = (0, 0, 0, 0);
            for elem in batch {
                let mut another_board = new_board;
                (&mut another_board).register_move(elem).unwrap();
                let part_res = test_count_moves(&another_board, 1, max_depth);
                res = (
                    res.0 + part_res.0,
                    res.1 + part_res.1,
                    res.2 + part_res.2,
                    res.3 + part_res.3,
                );
            }
            res
        });
        threads.push(part_res);
    }
    for elem in threads {
        let part_res = elem.join().unwrap();
        res = (
            res.0 + part_res.0,
            res.1 + part_res.1,
            res.2 + part_res.2,
            res.3 + part_res.3,
        );
    }
    res
}

fn test_count_moves(board: &Board, depth: u8, max_depth: u8) -> (u64, u64, u64, u64) {
    let move_set = Moves::get_all_moves(&board, board.turn);
    if depth == max_depth - 1 {
        let en_passants = move_set
            .0
            .iter()
            .filter(|&x| x.move_type == MoveType::EnPassantMove)
            .count();
        let castles = move_set
            .0
            .iter()
            .filter(|&x| {
                if let MoveType::CastleMove(_) = x.move_type {
                    true
                } else {
                    false
                }
            })
            .count();
        let promotions = move_set
            .0
            .iter()
            .filter(|&x| match x.move_type {
                MoveType::Move(_) => false,
                MoveType::Capture(_) => false,
                MoveType::EnPassantMove => false,
                MoveType::CastleMove(_) => false,
                MoveType::PromotionMove(_) => true,
                MoveType::PromotionCapture(_) => true,
            })
            .count();
        return (
            move_set.0.len() as u64,
            en_passants as u64,
            castles as u64,
            promotions as u64,
        );
    }

    move_set
        .0
        .into_iter()
        .map(|test_move| {
            let mut new_board = *board;
            let res = (&mut new_board).register_move(test_move);
            if res.is_err() {
                println!("{:?}\n{}\n{:?}", res, board, test_move);
                panic!("wtf is this?");
            }
            test_count_moves(&new_board, depth + 1, max_depth)
        })
        .fold((0, 0, 0, 0), |a, b| {
            (a.0 + b.0, a.1 + b.1, a.2 + b.2, a.3 + b.3)
        })
}

#[test]
fn position_1() {
    let board = Board::try_from(FenNotation(
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".into(),
    ))
    .unwrap();
    assert_eq!(test_count_moves(&board, 0, 1), (20, 0, 0, 0));
    assert_eq!(start_test(&board, 2), (400, 0, 0, 0));
    assert_eq!(start_test(&board, 3), (8902, 0, 0, 0));
    assert_eq!(start_test(&board, 4), (197281, 0, 0, 0));
    assert_eq!(start_test(&board, 5), (4865609, 258, 0, 0));
    assert_eq!(start_test(&board, 6), (119060324, 5248, 0, 0));
}

#[test]
fn position_2() {
    let board = Board::try_from(FenNotation(
        "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1".into(),
    ))
    .unwrap();
    assert_eq!(test_count_moves(&board, 0, 1), (48, 0, 2, 0));
    assert_eq!(start_test(&board, 2), (2039, 1, 91, 0));
    assert_eq!(start_test(&board, 3), (97862, 45, 3162, 0));
    assert_eq!(start_test(&board, 4), (4085603, 1929, 128013, 15172));
    assert_eq!(start_test(&board, 5), (193690690, 73365, 4993637, 8392));
}

#[test]
fn position_3() {
    let board = Board::try_from(FenNotation(
        "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1".into(),
    ))
    .unwrap();
    assert_eq!(test_count_moves(&board, 0, 1), (14, 0, 0, 0));
    assert_eq!(start_test(&board, 2), (191, 0, 0, 0));
    assert_eq!(start_test(&board, 3), (2812, 2, 0, 0));
    assert_eq!(start_test(&board, 4), (43238, 123, 0, 0));
    assert_eq!(start_test(&board, 5), (674624, 1165, 0, 0));
    assert_eq!(start_test(&board, 6), (11030083, 33325, 0, 7552));
    assert_eq!(start_test(&board, 7), (178633661, 294874, 0, 140024));
}

#[test]
fn position_4() {
    let board = Board::try_from(FenNotation(
        "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1".into(),
    ))
    .unwrap();
    assert_eq!(test_count_moves(&board, 0, 1), (6, 0, 0, 0));
    assert_eq!(start_test(&board, 2), (264, 0, 6, 48));
    assert_eq!(start_test(&board, 3), (9467, 4, 0, 120));
    assert_eq!(start_test(&board, 4), (422333, 0, 7795, 60032));
    assert_eq!(start_test(&board, 5), (15833292, 6512, 0, 329464));
}

#[test]
fn position_5() {
    let board = Board::try_from(FenNotation(
        "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8".into(),
    ))
    .unwrap();
    assert_eq!(test_count_moves(&board, 0, 1).0, 44);
    assert_eq!(start_test(&board, 2).0, 1486);
    assert_eq!(start_test(&board, 3).0, 62379);
    assert_eq!(start_test(&board, 4).0, 2103487);
    assert_eq!(start_test(&board, 5).0, 89941194);
}

#[test]
fn position_6() {
    let board = Board::try_from(FenNotation(
        "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10".into(),
    ))
    .unwrap();
    assert_eq!(test_count_moves(&board, 0, 1).0, 46);
    assert_eq!(start_test(&board, 2).0, 2079);
    assert_eq!(start_test(&board, 3).0, 89890);
    assert_eq!(start_test(&board, 4).0, 3894594);
    assert_eq!(start_test(&board, 5).0, 164075551);
}
