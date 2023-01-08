use super::GBCState;

struct Action {
    cycle_delay: u8,
    func: fn(&mut GBCState),
}

pub struct DelayedActions {
    actions: Vec<Action>,
}

impl DelayedActions {
    pub fn new() -> Self {
        Self {
            actions: Vec::new(),
        }
    }
}

pub fn tick(state: &mut GBCState) {
    let mut idx = 0;
    while idx < state.delayed_actions.actions.len() {
        let action = &mut state.delayed_actions.actions[idx];
        if action.cycle_delay == 0 {
            (action.func)(state);
            state.delayed_actions.actions.remove(idx);
            continue;
        }

        action.cycle_delay -= 1;
        idx += 1;
    }
}

/**
 * Delay execution of closure by x cycles
 */
pub fn schedule(state: &mut GBCState, action: fn(&mut GBCState), cycle_delay: u8) {
    state.delayed_actions.actions.push(Action {
        cycle_delay,
        func: action,
    });
}
