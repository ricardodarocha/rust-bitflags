use rand::Rng;
use std::ops::BitOr;

struct Bitboard(usize);

impl BitOr<usize> for Bitboard {
    type Output = usize;

    fn bitor(self, rhs: usize) -> Self::Output {
       self.0 | rhs
    }
}

impl BitOr for Bitboard {
    type Output = Bitboard;

    fn bitor(self, rhs: Self) -> Self::Output {
       Bitboard(self.0 | rhs.0)
    }
}


//Marca o enésimo bit como true
fn fill(num: usize, offset: u8) -> usize {
  if offset > 8 {panic!("Passou do limite")};
    num | (1 << offset)
}

//Verifica se o enésimo bit é true
fn is_filled(num: usize, offset: u8) -> bool {
    (num & (1 << offset)) != 0
}

//representa
fn render(alfa: usize, beta: usize) -> String {
    let mut result: Vec<char> = [].to_vec();
    for x in 0..9 {
        if is_filled(alfa, x) {result.push('❌')} else 
        if is_filled(beta, x) {result.push('⭕')} else
        {result.push('⬜')};
        if (x+1)%3 == 0 {result.push('\n')};
    }
    
    result.into_iter().collect()
}

fn proxima_jogada_valida(board: usize) -> u8 {
    loop {
    let mut rng = rand::thread_rng();
    let jogada: u8 = rng.gen_range(0..9);
    if !is_filled(board, jogada)  {
        return jogada;
    };
    }
}

fn proxima_jogada(alfa: usize, beta: usize, jogada: u8) -> (usize, usize) { 
    let proxima_jogada = proxima_jogada_valida(alfa | beta);
    let mut new_alfa = alfa;
    let mut new_beta = beta;
    if jogada % 2 == 0 {
        new_alfa = fill(alfa, proxima_jogada);
    } else
    {   new_beta = fill(beta, proxima_jogada);
    };
    (new_alfa, new_beta)
}

fn main() {
  let mut alfa = 0;
  let mut beta = 0;
  let mut jogada = 0;
  while jogada < 9 {
      (alfa, beta) = proxima_jogada(alfa, beta, jogada);
      jogada+=1;
      render(alfa, beta);
   print!("{}\n{jogada}\n\n", render(alfa, beta));
  };
   
}