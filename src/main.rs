use arrayvec::ArrayVec;
use rand::Rng;
use horsie::{chessboard, magic::*, movegen::*};

fn main() {
    println!("Horsie v{}", env!("CARGO_PKG_VERSION"));
    println!("By Rift");
    if let Ok(art) = std::fs::read_to_string("assets/art.txt") {
        println!("{art}");
    }

    let mut rng = rand::rng();

    // for _ in 0..100_000_000 {
    //     let x = rng.gen_range(0..8);
    //     let y = rng.gen_range(0..8);

    //     let mut blockers: u64 = u64::MAX;
    //     for _ in 0..3 {
    //         let r: u64 = rng.random();
    //         blockers = blockers & r;
    //     }
    //     let span = gen_blocked_diagonal(x, y, blockers);
    //     let span = gen_blocked_straight(x, y, blockers);
    // }

    // this looks ugly as fuck
    let mut straight_magics: Vec<[ArrayVec<u64, 4096>; 8]> = (0..8).map(|_| std::array::from_fn(|_| ArrayVec::new())).collect();
    let mut diagonal_magics: Vec<[ArrayVec<u64, 4096>; 8]> = (0..8).map(|_| std::array::from_fn(|_| ArrayVec::new())).collect();

    // let x = 3;
    // let y = 4;
    // for x in 0..8 {
    //     for y in 0..8 {
    //         let (table, magic) = gen_magic_table(x as u8, y as u8, true);
    //         straight_magics[x][y] = table;
    //         let (table, magic) = gen_magic_table(x as u8, y as u8, false);
    //         diagonal_magics[x][y] = table;
    //     }
    // }


    // print_bitboard(35321813161472);
    print_bitboard(7936);
    print_bitboard(65820);

    // boo! hello thereeeeee.......
}
