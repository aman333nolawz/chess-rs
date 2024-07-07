const CLEAR_RANK: [u64; 8] = [
    18446744073709551360,
    18446744073709486335,
    18446744073692839935,
    18446744069431361535,
    18446742978492891135,
    18446463698244468735,
    18374967954648334335,
    72057594037927935,
];

const CLEAR_FILE: [u64; 8] = [
    9187201950435737471,
    13816973012072644543,
    16131858542891098079,
    17289301308300324847,
    17868022691004938231,
    18157383382357244923,
    18302063728033398269,
    18374403900871474942,
];

// #[derive(PartialEq, Eq, PartialOrd, Ord)]
// pub enum PieceColor {
//     BLACK,
//     WHITE,
// }

// pub enum Piece {
//     PAWN(PieceColor),
//     BISHOP(PieceColor),
//     KNIGHT(PieceColor),
//     ROOK(PieceColor),
//     QUEEN(PieceColor),
//     KING(PieceColor),
//     BLANK,
// }

pub struct Side;
impl Side {
    pub const WHITE: usize = 1;
    pub const BLACK: usize = 0;
}

pub struct PieceType;
impl PieceType {
    pub const PAWN: usize = 0;
    pub const BISHOP: usize = 1;
    pub const KNIGHT: usize = 2;
    pub const ROOK: usize = 3;
    pub const QUEEN: usize = 4;
    pub const KING: usize = 5;
}

pub struct Piece {
    piece_type: usize,
    side: usize,
}

impl Piece {
    pub fn new(side: usize, piece_type: usize) -> Self {
        Piece { piece_type, side }
    }

    pub fn get_char(&self) -> char {
        let mut character = match self.piece_type {
            PieceType::PAWN => 'p',
            PieceType::KNIGHT => 'n',
            PieceType::BISHOP => 'b',
            PieceType::ROOK => 'r',
            PieceType::QUEEN => 'q',
            PieceType::KING => 'k',
            _ => '-',
        };
        if self.side == Side::WHITE {
            character = character.to_ascii_uppercase();
        }
        // println!("{}, {}, {}", character, self.side, self.piece_type);
        character
    }
}

pub fn get_nth_bit(num: u64, n: u8) -> u8 {
    return ((num >> (n)) & 1) as u8;
}

#[derive(Clone, Copy)]
pub struct Chess {
    pub board: [[u64; 6]; 2],
    pub turn: usize,
}

impl Chess {
    pub fn new() -> Self {
        let mut board: [[u64; 6]; 2] = [[0; 6]; 2];
        board[Side::WHITE][PieceType::PAWN] = 65280;
        board[Side::WHITE][PieceType::BISHOP] = 36;
        board[Side::WHITE][PieceType::KNIGHT] = 66;
        board[Side::WHITE][PieceType::ROOK] = 129;
        board[Side::WHITE][PieceType::QUEEN] = 16;
        board[Side::WHITE][PieceType::KING] = 8;

        board[Side::WHITE][PieceType::BISHOP] = 0;
        board[Side::WHITE][PieceType::ROOK] = 0;
        board[Side::WHITE][PieceType::QUEEN] = 0;

        board[Side::BLACK][PieceType::PAWN] = 71776119061217280;
        board[Side::BLACK][PieceType::BISHOP] = 2594073385365405696;
        board[Side::BLACK][PieceType::KNIGHT] = 4755801206503243776;
        board[Side::BLACK][PieceType::ROOK] = 9295429630892703744;
        board[Side::BLACK][PieceType::QUEEN] = 1152921504606846976;
        // let black_king = 576460752303423488;
        board[Side::BLACK][PieceType::KING] = 549755813888;

        Self {
            board,
            turn: Side::WHITE,
        }
    }

    pub fn get_piece_at(self, i: u8) -> Option<Piece> {
        for (_i, side) in self.board.iter().enumerate() {
            for (j, piece) in side.iter().enumerate() {
                if (piece >> i) & 1 == 1 {
                    return Some(Piece::new(_i, j));
                }
            }
        }
        None
    }

    pub fn legal_moves(self, i: u8, turn: usize) -> u64 {
        match self.get_piece_at(i) {
            Some(piece) => {
                if piece.side != turn {
                    0
                } else if piece.piece_type == PieceType::KING {
                    self.get_king_pseudo_moves(Some(i), piece.side)
                } else if piece.piece_type == PieceType::KNIGHT {
                    self.get_knight_pseudo_moves(Some(i), piece.side)
                } else if piece.piece_type == PieceType::PAWN && piece.side == Side::WHITE {
                    self.get_white_pawn_pseudo_moves(Some(i))
                } else if piece.piece_type == PieceType::PAWN && piece.side == Side::BLACK {
                    self.get_black_pawn_pseudo_moves(Some(i))
                } else {
                    0
                }
            }
            _ => 0,
        }
    }

    fn get_king_pseudo_moves(&self, i: Option<u8>, side: usize) -> u64 {
        let king_pos: u64;
        let mut own_side: u64 = 0;
        for pieces in self.board[side] {
            own_side = own_side | pieces;
        }

        match i {
            Some(n) => king_pos = 0 | (1 << n),
            None => king_pos = self.board[side][PieceType::KING],
        }

        let king_clip1 = king_pos & CLEAR_FILE[0];
        let king_clip2 = king_pos & CLEAR_FILE[7];

        let s1 = king_clip2 << 7;
        let s2 = king_pos << 8;
        let s3 = king_clip1 << 9;
        let s4 = king_clip1 << 1;

        let s5 = king_clip1 >> 7;
        let s6 = king_pos >> 8;
        let s7 = king_clip2 >> 9;
        let s8 = king_clip2 >> 1;

        let king_moves = s1 | s2 | s3 | s4 | s5 | s6 | s7 | s8;
        return king_moves & !own_side;
    }

    fn get_knight_pseudo_moves(&self, i: Option<u8>, side: usize) -> u64 {
        let knights_pos: u64;
        let mut own_side: u64 = 0;
        for pieces in self.board[side] {
            own_side = own_side | pieces;
        }
        match i {
            Some(n) => knights_pos = 0 | (1 << n),
            None => knights_pos = self.board[side][PieceType::KNIGHT],
        }

        let s1_clip = CLEAR_FILE[7] & CLEAR_FILE[6];
        let s2_clip = CLEAR_FILE[7];
        let s3_clip = CLEAR_FILE[0];
        let s4_clip = CLEAR_FILE[0] & CLEAR_FILE[1];

        let s5_clip = CLEAR_FILE[0] & CLEAR_FILE[1];
        let s6_clip = CLEAR_FILE[0];
        let s7_clip = CLEAR_FILE[7];
        let s8_clip = CLEAR_FILE[7] & CLEAR_FILE[6];

        let s1 = (knights_pos & s1_clip) << 6;
        let s2 = (knights_pos & s2_clip) << 15;
        let s3 = (knights_pos & s3_clip) << 17;
        let s4 = (knights_pos & s4_clip) << 10;

        let s5 = (knights_pos & s5_clip) >> 6;
        let s6 = (knights_pos & s6_clip) >> 15;
        let s7 = (knights_pos & s7_clip) >> 17;
        let s8 = (knights_pos & s8_clip) >> 10;

        let knight_valid = s1 | s2 | s3 | s4 | s5 | s6 | s7 | s8;
        return knight_valid & !own_side;
    }

    fn get_white_pawn_pseudo_moves(&self, i: Option<u8>) -> u64 {
        let pawns_pos: u64;
        let mut white_pieces = 0;
        let mut black_pieces = 0;
        for (index, pieces) in self.board[Side::WHITE].iter().enumerate() {
            white_pieces = white_pieces | pieces;
            black_pieces = black_pieces | self.board[Side::BLACK][index];
        }

        match i {
            Some(n) => pawns_pos = 0 | (1 << n),
            None => pawns_pos = self.board[Side::WHITE][PieceType::PAWN],
        };

        let one_step = pawns_pos << 8 & !white_pieces;
        let two_step = (one_step & !CLEAR_RANK[2]) << 8;
        let valid_moves = (one_step | two_step) & !black_pieces;

        // Attacks
        let left_attack = (pawns_pos & CLEAR_FILE[0]) << 7;
        let right_attack = (pawns_pos & CLEAR_FILE[7]) << 9;
        let attacks = (left_attack | right_attack) & black_pieces;

        valid_moves | attacks
    }

    fn get_black_pawn_pseudo_moves(&self, i: Option<u8>) -> u64 {
        let pawns_pos: u64;
        let mut white_pieces = 0;
        let mut black_pieces = 0;
        for (index, pieces) in self.board[Side::WHITE].iter().enumerate() {
            white_pieces = white_pieces | pieces;
            black_pieces = black_pieces | self.board[Side::BLACK][index];
        }

        match i {
            Some(n) => pawns_pos = 0 | (1 << n),
            None => pawns_pos = black_pieces,
        };

        let one_step = pawns_pos >> 8 & !black_pieces;
        let two_step = (one_step & !CLEAR_RANK[5]) >> 8;
        let valid_moves = (one_step | two_step) & !white_pieces;

        // Attacks
        let right_attack = (pawns_pos & CLEAR_FILE[0]) >> 7;
        let left_attack = (pawns_pos & CLEAR_FILE[7]) >> 9;
        let attacks = (left_attack | right_attack) & white_pieces;

        valid_moves | attacks
    }

    pub fn move_piece(&mut self, from: u8, to: u8) {
        if get_nth_bit(self.legal_moves(from, self.turn), to) == 0 {
            return;
        }
        let piece = self.get_piece_at(from);
        let side;
        let piece_type;
        match piece {
            Some(piece) => {
                side = piece.side;
                piece_type = piece.piece_type;
            }
            _ => return,
        };

        let mut white_pieces = 0;
        let mut black_pieces = 0;
        for (index, pieces) in self.board[Side::WHITE].iter().enumerate() {
            white_pieces = white_pieces | pieces;
            black_pieces = black_pieces | self.board[Side::BLACK][index];
        }
        if side == Side::WHITE {
            white_pieces =
                white_pieces | ((self.board[side][piece_type] & !(1 << from)) | (1 << (to)));
        } else {
            black_pieces =
                black_pieces | ((self.board[side][piece_type] & !(1 << from)) | (1 << (to)));
        }

        // Checking removal of pieces
        if get_nth_bit(white_pieces, to) == get_nth_bit(black_pieces, to) {
            let removed_piece = self.get_piece_at(to).unwrap();
            self.board[removed_piece.side][removed_piece.piece_type] =
                self.board[removed_piece.side][removed_piece.piece_type] & !(1 << to);
        };

        self.board[side][piece_type] = (self.board[side][piece_type] & !(1 << from)) | (1 << (to));
        self.turn = if self.turn == Side::WHITE {
            Side::BLACK
        } else {
            Side::WHITE
        }
    }
}
