use std::{collections::HashMap, hash::Hash};

#[derive(Debug, Clone, Copy)]
pub enum StateType {
    Regular,
    Accepting,
}

#[derive(Debug, Clone, Copy)]
pub struct State(usize);

pub trait StateTransformer<Item: Sized, StackItem: Sized> {
    fn transition(&self, input: &mut Vec<Item>, stack: &mut Vec<StackItem>) -> Option<State>;
}

pub struct StateMachine<Item, StackItem> {
    pub starting_state: State,
    transitions: Vec<Option<Box<dyn StateTransformer<Item, StackItem>>>>,
    state_types: Vec<StateType>,
}

impl<Item, StackItem> StateMachine<Item, StackItem> {
    pub fn new(starting_state: StateType) -> Self {
        let transitions = vec![None];

        let state_types = vec![starting_state];
        let starting_node_idx = State(0);

        StateMachine {
            transitions,
            starting_state: starting_node_idx,
            state_types,
        }
    }

    pub fn len(&self) -> usize {
        self.state_types.len()
    }

    pub fn add_state(&mut self, state_type: StateType) -> State {
        self.transitions.push(None);
        self.state_types.push(state_type);

        State(self.len() - 1)
    }

    pub fn set_transition(
        &mut self,
        state: State,
        transition: Box<dyn StateTransformer<Item, StackItem>>,
    ) {
        self.transitions[state.0] = Some(transition);
    }

    pub fn remove_transition(&mut self, state: State) {
        self.transitions[state.0] = None;
    }

    pub fn run(&self, mut input: Vec<Item>) -> bool {
        input.reverse(); // Reverse the input so that it will

        let mut stack = Vec::new();

        let mut cur_state = self.starting_state;

        while !input.is_empty() {
            let transition = match &self.transitions[cur_state.0] {
                Some(transition) => transition,
                None => return false,
            };

            cur_state = match transition.transition(&mut input, &mut stack) {
                Some(next_state) => next_state,
                None => return false,
            }
        }

        match self.state_types[cur_state.0] {
            StateType::Accepting => true,
            StateType::Regular => false,
        }
    }
}

pub struct SimpleTransition<Item> {
    transitions: HashMap<Item, State>,
}

impl<Item> SimpleTransition<Item>
where
    Item: Eq + Hash,
{
    pub fn new() -> Self {
        SimpleTransition {
            transitions: HashMap::new(),
        }
    }

    pub fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = (Item, State)>,
    {
        SimpleTransition {
            transitions: HashMap::from_iter(iter),
        }
    }

    pub fn add_transition(&mut self, inp_letter: Item, next_state: State) {
        self.transitions.insert(inp_letter, next_state);
    }
}

impl<Item> StateTransformer<Item, ()> for SimpleTransition<Item>
where
    Item: Eq + Hash,
{
    fn transition(&self, input: &mut Vec<Item>, _: &mut Vec<()>) -> Option<State> {
        let inp = input.pop()?;
        self.transitions.get(&inp).map(|&state| state)
    }
}
