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
    pub name: String,
    pub id: u8,
    pub animation_info: AnimationInterTransitionInfo,
    pub current_frame: usize,
    pub time_since_frame_change: f32,
}

impl AnimationState {
    pub fn new(name: String, id: u8,  texture_order: Vec<[u8; 2]>, timings: Vec<f32>) -> Self {
        let animation_info = AnimationInterTransitionInfo::new(texture_order, timings);
        Self {
            name,
            id,
            animation_info,
            current_frame: 0,
            time_since_frame_change: 0.0,
        }
    }

    pub fn update_frame(&mut self, time_elapsed: f32) -> Option<[u8; 2]> {
        if self.time_since_frame_change > self.animation_info.timings[self.current_frame] {
            if self.current_frame < self.animation_info.order.len() - 1{
                self.current_frame = self.current_frame + 1;
            } else {
                self.current_frame = 0;
            }
            self.time_since_frame_change = 0.0;
            return Some(self.animation_info.order[self.current_frame]);
        } else {
            self.time_since_frame_change = self.time_since_frame_change + time_elapsed;
            return None
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct AnimationInterTransitionInfo {
    pub order: Vec<[u8; 2]>,
    pub timings: Vec<f32>,
}

impl AnimationInterTransitionInfo {
    pub fn new(order: Vec<[u8; 2]>, timings: Vec<f32>) -> Self {
        Self {
            order,
            timings
        }
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