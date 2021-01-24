#[derive(Clone, PartialEq)]
pub struct AnimationStateMachine {
    pub current_state: AnimationState,
    states: Vec<AnimationState>,
    rules: Vec<Vec<StateChangeRule>>
}


impl AnimationStateMachine {
    pub fn new(states: Vec<AnimationState>, rules: Vec<Vec<StateChangeRule>>) -> Self {
        Self {
            current_state: states[0].clone(),
            states: states,
            rules: rules,
        }
    }

    pub fn update(&mut self, input: PhysicalInput) -> bool {
        let current_state_index = self.states.iter().position(|state| state.id == self.current_state.id).unwrap();
        
        for state_rule in self.rules[current_state_index].iter() {
            if state_rule.input == input {
                self.current_state = state_rule.next_state.clone();
                return true;
            }
        }
        return false;
    }
}

#[derive(Clone, PartialEq)]
pub struct AnimationState {
    pub current_frame: usize,
    pub name: String,
    pub id: u8,
    pub texture_order: Vec<[u8; 2]>
}

impl AnimationState {

    pub fn new(name: String, id: u8,  texture_order: Vec<[u8; 2]>) -> Self {
        Self {
            name,
            id,
            texture_order,
            current_frame: 0
        }
    }

    pub fn update(&mut self) -> [u8; 2] {
        if self.current_frame < self.texture_order.len() - 1{
            self.current_frame = self.current_frame + 1;
        } else {
            self.current_frame = 0;
        }

        return self.texture_order[self.current_frame];
    }
}

#[derive(Clone, PartialEq)]
pub struct StateChangeRule {
    state: AnimationState, // EX. Standing.
    input: PhysicalInput, // -> right directional is pressed.
    next_state: AnimationState, // -> RunRight
}

impl StateChangeRule {
    pub fn new(state: AnimationState, input: PhysicalInput, next_state: AnimationState) -> Self {
        Self {
            state,
            input,
            next_state,
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum PhysicalInput {
    W,
    A,
    S,
    D
}