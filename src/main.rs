struct Board{
    board: Vec<Square>,
}
struct Game{
    board: Board,
    player_to_move: Color,
    white_can_castle_kingside: bool,
    white_can_castle_queenside: bool,
    black_can_castle_kingside: bool,
    black_can_castle_queenside: bool,
    en_passant_target: Option<usize>,
    half_move_counter: u32,
    full_move_couner: u32,
}
#[derive(Debug)]
enum Color{
    White,
    Black
}
#[derive(Debug)]
struct Square{
    square: Option<Piece>
}
#[derive(Debug)]
enum Piece {
    WhiteKing,
    WhitePawn,
    WhiteRook,
    WhiteKnight,
    WhiteBishop,
    WhiteQueen,
    BlackKing,
    BlackPawn,
    BlackRook,
    BlackKnight,
    BlackBishop,
    BlackQueen,
}
impl Board{
    fn show(&self){
        for i in self.board.iter(){
            println!("{:?}", i);
        }
        println!("{}", self.board.len())
    }
}
impl From<&str> for Game{
    fn from(fen: &str) -> Game{
        // Naming is hell. Especially when I do it.
        let mut white_can_castle_kingside = false;
        let mut white_can_castle_queenside = false;
        let mut black_can_castle_kingside = false;
        let mut black_can_castle_queenside = false;
    
        let split_fen = fen.split_once(" ").expect("invalid fen");
        let piece_positions_string = split_fen.0;
        let to_be_parsed_fen = split_fen.1;
        let split_fen = to_be_parsed_fen.split_once(" ").expect("invalid fen");
        let player_to_move_string = split_fen.0;
        let to_be_parsed_fen = split_fen.1;
        let split_fen = to_be_parsed_fen.split_once(" ").expect("invalid fen");
        let castling_rights_string = split_fen.0;
        let to_be_parsed_fen = split_fen.1;
        let split_fen = to_be_parsed_fen.split_once(" ").expect("invalid fen");
        let possible_en_passant_targets_string = split_fen.0;
        let to_be_parsed_fen = split_fen.1;
        let split_fen = to_be_parsed_fen.split_once(" ").expect("invalid fen");
        let half_move_counter: u32 = split_fen.0.parse().unwrap();
        let full_move_counter: u32 = split_fen.1.parse().unwrap();
    
        for i in castling_rights_string.chars(){
            match i {
                'K' => white_can_castle_kingside = true,
                'Q' => white_can_castle_queenside = true,
                'k' => black_can_castle_kingside = true,
                'q' => black_can_castle_queenside = true,
                '-' => (),
                x => panic!("invalid fen {}", x)
            }
        }
        let possible_en_passant_targets: Option<usize>;
        if possible_en_passant_targets_string == "-"{
            possible_en_passant_targets = None
        }
        else{
            possible_en_passant_targets = Some(square_string_to_index(possible_en_passant_targets_string));
        }
        let player_to_move: Color;
        match player_to_move_string{
            "w" => player_to_move = Color::White,
            "b" => player_to_move = Color::Black,
            _ => panic!("invalid fen")
        }
        let mut piece_positions: Vec<Square> = vec![];
        for i in piece_positions_string.chars(){
            match i {
                'p' => &piece_positions.push(Square{square: Some(Piece::BlackPawn)}),
                'r' => &piece_positions.push(Square{square: Some(Piece::BlackRook)}),
                'n' => &piece_positions.push(Square{square: Some(Piece::BlackKnight)}),
                'b' => &piece_positions.push(Square{square: Some(Piece::BlackBishop)}),
                'q' => &piece_positions.push(Square{square: Some(Piece::BlackQueen)}),
                'k' => &piece_positions.push(Square{square: Some(Piece::BlackKing)}),
                'P' => &piece_positions.push(Square{square: Some(Piece::WhitePawn)}),
                'R' => &piece_positions.push(Square{square: Some(Piece::WhiteRook)}),
                'N' => &piece_positions.push(Square{square: Some(Piece::WhiteKnight)}),
                'B' => &piece_positions.push(Square{square: Some(Piece::WhiteBishop)}),
                'Q' => &piece_positions.push(Square{square: Some(Piece::WhiteQueen)}),
                'K' => &piece_positions.push(Square{square: Some(Piece::WhiteKing)}),
                '/' => &(),
                x => {&for _ in 1..x.to_digit(10).unwrap() + 1{
                    piece_positions.push(Square{square: None});
                }}
            };
        }
        return Game{
            board: Board{board: piece_positions},
            player_to_move: player_to_move,
            white_can_castle_kingside: white_can_castle_kingside,
            white_can_castle_queenside: white_can_castle_queenside,
            black_can_castle_kingside: black_can_castle_kingside,
            black_can_castle_queenside: black_can_castle_queenside,
            en_passant_target: possible_en_passant_targets,
            half_move_counter: half_move_counter,
            full_move_couner: full_move_counter
        }
    
    }
}
fn square_string_to_index(square: &str) -> usize{
    let column_list = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
    let alphabetic_charector = square.chars().nth(0).unwrap();
    let integer_charector = square.chars().nth(1).unwrap().to_digit(10).unwrap();
    return ((integer_charector - 1) * 8) as usize + column_list.iter().position(|x| *x == alphabetic_charector).unwrap()
}


fn main() {
    let game = Game::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    let board = game.board;
    board.show();
}
