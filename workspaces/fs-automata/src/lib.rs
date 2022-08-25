use std::collections::HashMap;

type Result<T> = std::result::Result<T, AutomataError>;

#[derive(Debug, Clone, Eq, PartialEq)]
struct AutomataError;

struct Automata {
    initial_state: Option<String>,
    current_state: Option<String>,
    states: HashMap<String, State>,
    transitions: HashMap<State, Vec<Transition>>
}

impl Automata {
    fn new() -> Automata {
        Automata{
            initial_state: None,
            current_state: None,
            states: HashMap::new(),
            transitions: HashMap::new()
        }
    }

    fn add_state(&mut self, initial: bool, state_name: &str, transitions: HashMap<State, Vec<Transition>>) -> Result<()> {
        let state = State::new(state_name);
        if initial {
            if self.initial_state.is_none() {
                self.initial_state = Some(state_name.to_string());
                self.current_state = Some(state_name.to_string())
            } else {
                return Err(AutomataError)
            }
        }
        self.states.insert(state_name.to_string(), state);
        let new_transitions = self.transitions.clone().into_iter().chain(transitions).collect();
        self.transitions = new_transitions;
        Ok(())
    }

    fn add_transition(&mut self, start_state: &str, end_state: &str, trigger: &str) -> Result<()>{
        // Check for start and end states and error if they don't exist
        let potential_start_state = self.states.get(start_state);
        if potential_start_state.is_none() {
            return Err(AutomataError)
        }
        if self.states.get(end_state).is_none() {
            return Err(AutomataError)
        }

        let start_state = potential_start_state.unwrap().clone();
        let end_state = self.states.get(end_state).unwrap().clone();

        self.transitions.entry(start_state.clone()).or_insert(vec![]);
        self.transitions.entry(start_state.clone()).and_modify(|trans| trans.push(
            Transition{
                trigger: trigger.to_string(),
                start_state,
                end_state
            }
        ));

        Ok(())
    }

    fn transition(&mut self, trigger: &str) {

    }

    fn get_states(&self) -> Vec<&State> {
        return self.states.values().collect()
    }

    fn get_current_state(&self) -> Option<String> {
        return self.current_state.clone()
    }


}

#[derive(Eq, Hash, Debug, Clone)]
struct State {
    name: String,
}


impl State {
    fn new(name: &str) -> State {
        State{
            name: name.to_string()
        }
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name)
    }
}

#[derive(Clone)]
struct Transition {
    trigger: String,
    start_state: State,
    end_state: State,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_state() {
        let mut automata = Automata::new();
        automata.add_state(true, "Open", HashMap::new()).expect("Could not add new state");

        assert_eq!(automata.get_states(), vec![&State::new("Open")])
    }

    #[test]
    fn test_add_initial_state_sets_current_state() {
        let mut automata = Automata::new();
        automata.add_state(true, "Open", HashMap::new()).expect("Could not add new state");

        assert_eq!(automata.get_current_state().unwrap(), "Open")
    }

    #[test]
    fn test_cannot_add_two_initial_states() {
        let mut automata = Automata::new();
        automata.add_state(true, "Open", HashMap::new()).expect("Could not add new state");
        let error = automata.add_state(true, "Open", HashMap::new());

        assert_eq!(error.err().unwrap(), AutomataError)
    }

    #[test]
    fn test_add_multiple_states() {
        let mut automata = Automata::new();
        automata.add_state(true, "Open", HashMap::new()).expect("Could not add new state");
        automata.add_state(false, "Working", HashMap::new()).expect("Could not add new state");
        automata.add_state(false, "Closed", HashMap::new()).expect("Could not add new state");

        let mut actual_states = automata.get_states();
        actual_states.sort_by(|s, t| s.name.partial_cmp(&t.name).unwrap());

        assert_eq!(
            actual_states,
            vec![&State::new("Closed"), &State::new("Open"), &State::new("Working")]
        )
    }

    #[test]
    fn test_add_transition() {
        let mut automata = Automata::new();
        automata.add_state(true, "Open", HashMap::new()).expect("Could not add new state");
        automata.add_state(false, "Closed", HashMap::new()).expect("Could not add new state");

        automata.add_transition("Open", "Closed", "Close Door");

        automata.transition("Close Door");

        assert_eq!(automata.get_current_state().unwrap(), "Closed")
    }
}
