#[derive(Debug, Clone, Copy)]
pub enum StateType {
    Regular,
    Accepting,
}

#[derive(Debug, Clone, Copy)]
pub struct State(usize);

pub trait StateTransformer<Item: Sized, StackItem: Sized> {
    fn transition(&self, input: &mut Vec<Item>, stack: &mut Vec<StackItem>) -> State;
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

        State(self.len())
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

    pub fn run(&self, input: &mut Vec<Item>) -> bool {
        let mut stack = Vec::new();

        let mut cur_state = self.starting_state;

        while !input.is_empty() {
            let transition = match &self.transitions[cur_state.0] {
                Some(transition) => transition,
                None => return false,
            };

            cur_state = transition.transition(input, &mut stack);
        }

        match self.state_types[cur_state.0] {
            StateType::Accepting => true,
            StateType::Regular => false,
        }
    }
}
