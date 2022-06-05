use state_machine::{StateMachine, StateType};

use crate::state_machine::SimpleTransition;

pub mod state_machine;

fn main() {
    let mut state_machine = StateMachine::<char, ()>::new(StateType::Regular);

    let starting = state_machine.starting_state;
    let s1 = state_machine.add_state(StateType::Regular);
    let s2 = state_machine.add_state(StateType::Accepting);

    state_machine.set_transition(
        starting,
        Box::new(SimpleTransition::from_iter([('a', starting), ('b', s1)])),
    );

    state_machine.set_transition(
        s1,
        Box::new(SimpleTransition::from_iter([('b', s1), ('c', s2)])),
    );

    state_machine.set_transition(s2, Box::new(SimpleTransition::from_iter([('c', s2)])));

    println!("{}", state_machine.run(vec!['c', 'b', 'a', 'a']));
    println!("{}", state_machine.run(vec!['a', 'b', 'b', 'c', 'c']));
    println!("{}", state_machine.run(vec!['a', 'b', 'c']));
    println!("{}", state_machine.run(vec!['b', 'c']));
    println!("{}", state_machine.run(vec!['a', 'c']));
}
