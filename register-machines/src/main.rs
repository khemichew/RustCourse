use std::collections::HashMap;

type Label = usize;
type Register = u128;
type State = (Label, HashMap<Register, u128>);

#[derive(Clone, Copy, Debug, PartialEq)]
enum Instruction {
    Add(Register, Label),
    Sub(Register, Label, Label),
    Halt
}

use Instruction::*;
fn eval_program(program: &[Instruction], state: &State) -> State {
    unimplemented!();
}
// <<x,y>> = (2^x)*(2y+1)
fn encode_pair1(x: u128, y: u128) -> u128 {
    unimplemented!();
}
// <x,y> = (2^x)*(2y+1)-1
fn encode_pair2(x: u128, y: u128) -> u128 {
    unimplemented!();
}
fn encode_list_to_godel(l: &[u128]) -> u128 {
    unimplemented!();
}
fn encode_program_to_list(program: &[Instruction]) -> Vec<u128> {
    unimplemented!();
}
// a = (2^x)*(2y+1)
fn decode_pair1(a: u128) -> (u128, u128) {
    unimplemented!();
}
// a = (2^x)*(2y+1)-1
fn decode_pair2(a: u128) -> (u128, u128) {
    unimplemented!();
}
fn decode_godel_to_list(g: u128) -> Vec<u128> {
    unimplemented!();
}
fn decode_list_to_program(program: &[u128]) -> Vec<Instruction> {
    unimplemented!();
}

fn main() {
}

mod test {
    use crate::*;
    #[test]
    fn godel_num_to_godel_list() {
        let n = 2u128.pow(46) * 20483;
        let godel_list = decode_godel_to_list(n);
        let true_godel_list = vec![46, 0, 10, 1];
        assert_eq!(godel_list, true_godel_list)
    }

    #[test]
    fn godel_list_to_godel_num() {
        let godel_num = encode_list_to_godel(&[46, 0, 10, 1]);
        assert_eq!(godel_num, 2u128.pow(46) * 20483)
    }

    #[test]
    fn godel_list_to_program() {
        let program = decode_list_to_program(&vec![46, 0, 10, 1]);
        assert_eq!(program, vec![Sub(0, 2, 1), Halt, Sub(0, 0, 1), Add(0, 0)])
    }

    #[test]
    fn program_to_godel_list() {
        let program = encode_program_to_list(&[Sub(0, 2, 1), Halt, Sub(0, 0, 1), Add(0, 0)]);
        assert_eq!(program, [46, 0, 10, 1])
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