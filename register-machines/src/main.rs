use std::{collections::HashMap, vec};
use rug::{Integer, ops::Pow};

type Godel = Integer; 
type Label = usize;
type Register = u128;
type State = (Label, HashMap<Register, u128>);

#[derive(Clone, Copy, Debug, PartialEq)]
enum Instruction {
    Add(Register, Label),
    Sub(Register, Label, Label),
    Halt
}

impl Instruction {
    pub fn encode_instruction(&self) -> Godel {
        match self {
            Add(i, j) => 
                encode_pair1(Godel::from(2 * i), Godel::from(*j)),
            Sub(i, j, k) => 
                encode_pair1(Godel::from(2 * i + 1),
                encode_pair2(Godel::from(*j), Godel::from(*k))
            ),
            Halt => Godel::ZERO
        }
    }
}

use Instruction::*;
fn eval_program(program: &[Instruction], state: &State) -> State {
    let (mut l, mut rs) = state.clone();
    let mut ins = program[l];
    while ins != Halt {
        match ins {
            Add(i, j) => {
                let v = rs.entry(i).or_insert(0);
                *v += 1;
                l = j; 
            },
            Sub(i, j, k) => {
                let v = rs.entry(i).or_insert(0); 
                if *v != 0 {
                    *v -= 1;
                    l = j;
                } else {
                    l = k; 
                }
            },
            _ => panic!("unreachable")
        }
        ins = if l >= program.len() { Halt } else { program[l] };
    }
    (l, rs)
}
// <<x,y>> = (2^x)*(2y+1)
fn encode_pair1(x: Godel, y: Godel) -> Godel {
    let left: Godel = Godel::from(2).pow(x.to_u32_wrapping());
    let right: Godel = Godel::from(2) * y + Godel::from(1);
    left * right
}
// <x,y> = (2^x)*(2y+1)-1
fn encode_pair2(x: Godel, y: Godel) -> Godel {
    encode_pair1(x, y) - Godel::from(1)
}
fn encode_list_to_godel(l: &[Godel]) -> Godel {
    if l.is_empty() { return Godel::ZERO; } 
    encode_pair1(l[0].clone(), encode_list_to_godel(&l[1..])) 
}
fn encode_program_to_list(program: &[Instruction]) -> Vec<Godel> {
    program.iter().map(Instruction::encode_instruction).collect()
}
fn trailing_zeros_in_binary(x: Godel) -> Godel {
    let mut b = x; 
    let mut c = Godel::new();
    if b != 0 {
        b = (b.clone() ^ (b - 1)) >> 1; 
        while b != 0 {
            c += 1;
            b >>= 1;
        }
    }
    c
    // in theory it should be infinity, but rug does not support such calls 
}
fn decode_instruction(ins: Godel) -> Instruction {
    if ins == 0 { return Halt; }
    let (x, y) = decode_pair1(ins); 
    let i: Godel = x.clone() / 2;
    if x % 2 != 0 {
        let (j, k) = decode_pair2(y);
        Sub(i.try_into().unwrap(), j.try_into().unwrap(), k.try_into().unwrap())
    } else {
        Add(i.try_into().unwrap(), y.try_into().unwrap())
    }
}
// a = (2^x)*(2y+1)
fn decode_pair1(a: Godel) -> (Godel, Godel) {
    let x: Godel = trailing_zeros_in_binary(a.clone()); 
    let z: Godel = a / Godel::from(2).pow(x.to_u32_wrapping()); 
    let y: Godel = (z - 1) / 2;
    (x, y)
}
// a = (2^x)*(2y+1)-1
fn decode_pair2(a: Godel) -> (Godel, Godel) {
    decode_pair1(a + 1)
}
fn decode_godel_to_list(g: Godel) -> Vec<Godel> {
    if g == 0 { return Vec::new(); }
    let (x, xs) = decode_pair1(g);
    let mut gs = vec![x]; 
    gs.splice(gs.len().., decode_godel_to_list(xs));
    gs 
}
fn decode_list_to_program(program: &[Godel]) -> Vec<Instruction> {
    program.iter().map(|x| decode_instruction(x.clone())).collect()
}

fn main() {
}

mod test {
    use crate::*;
    #[test]
    fn halt_encodes_to_godel_zero_num() {
        let g = Halt.encode_instruction();
        assert_eq!(g, Godel::ZERO); 
    }

    #[test] 
    fn godel_zero_num_decodes_to_halt() {
        let ins = decode_instruction(Godel::ZERO);
        assert_eq!(ins, Halt);
    }

    #[test]
    fn godel_num_to_godel_list() {
        let n = Godel::from(2).pow(46) * 20483;
        let godel_list: Vec<Godel> = decode_godel_to_list(n);
        let true_godel_list: Vec<Godel> = vec![46, 0, 10, 1].iter().map(|x| Godel::from(x.clone())).collect();
        assert_eq!(godel_list, true_godel_list)
    }

    #[test]
    fn godel_num_to_godel_list_large_num() {
        let n = Godel::from(2).pow(216) * 833;
        let godel_list: Vec<Godel> = decode_godel_to_list(n);
        let true_godel_list: Vec<Godel> = vec![216, 5, 1, 0].iter().map(|x| Godel::from(x.clone())).collect();
        assert_eq!(godel_list, true_godel_list) 
    }

    #[test]
    fn godel_list_to_godel_num() {
        let true_godel_list: Vec<Godel> = [46, 0, 10, 1].iter().map(|x| Godel::from(x.clone())).collect();
        let godel_num: Godel = encode_list_to_godel(&true_godel_list);
        assert_eq!(godel_num, 2u128.pow(46) * 20483)
    }

    #[test]
    fn godel_list_to_program() {
        let true_godel_list: Vec<Godel> = [46, 0, 10, 1].iter().map(|x| Godel::from(x.clone())).collect();
        let program = decode_list_to_program(&true_godel_list);
        assert_eq!(program, vec![Sub(0, 2, 1), Halt, Sub(0, 0, 1), Add(0, 0)])
    }

    #[test]
    fn godel_list_to_program_2() {
        let true_godel_list: Vec<Godel> = [408, 2272, 7, 192, 8064, 144, 0].iter().map(|x| Godel::from(x.clone())).collect();
        let program = decode_list_to_program(&true_godel_list);
        assert_eq!(program, vec![Sub(1, 1, 6), Sub(2, 2, 4), Add(0, 3), Add(3, 1), Sub(3, 5, 0), Add(2, 4), Halt])
    }

    #[test]
    fn program_to_godel_list() {
        let program: Vec<Godel> = encode_program_to_list(&[Sub(0, 2, 1), Halt, Sub(0, 0, 1), Add(0, 0)]);
        let true_godel_list: Vec<Godel> = [46, 0, 10, 1].iter().map(|x| Godel::from(x.clone())).collect();
        assert_eq!(program, true_godel_list)
    }

    #[test]
    fn program_to_godel_list_2() {
        let program: Vec<Godel> = encode_program_to_list(&[Sub(1, 1, 6), Sub(2, 2, 4), Add(0, 3), Add(3, 1), Sub(3, 5, 0), Add(2, 4), Halt]);
        let true_godel_list: Vec<Godel> = [408, 2272, 7, 192, 8064, 144, 0].iter().map(|x| Godel::from(x.clone())).collect();
        assert_eq!(program, true_godel_list)
    }

    #[test]
    fn program_produces_correct_state() {
        use std::array::IntoIter;
        let program = vec![
            Sub(1, 2, 1),
            Halt,
            Sub(1, 3, 4),
            Sub(1, 5, 4),
            Halt,
            Add(0, 0)
        ];
        let final_state = eval_program(
            &program,
            &(
                0,
                HashMap::<_, _>::from_iter(IntoIter::new([(0, 0), (1, 7)]))
            ),
        );
        assert_eq!(
            final_state,
            (
                4,
                HashMap::<_, _>::from_iter(IntoIter::new([(0, 2), (1, 0)]))
            )
        )
    }
}