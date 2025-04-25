use super::transition::{StateTransition, StateTransitions};
use syn::{braced, parse, token, Attribute, Ident, Token};

#[derive(Debug)]
pub struct StateMachine {
    pub transitions: Vec<StateTransition>,
    pub name: Option<Ident>,
    pub states_attr: Vec<Attribute>,
    pub events_attr: Vec<Attribute>,
    pub entry_exit_async: bool,
}

impl StateMachine {
    pub fn new() -> Self {
        StateMachine {
            transitions: Vec::new(),
            name: None,
            states_attr: Vec::new(),
            events_attr: Vec::new(),
            entry_exit_async: false,
        }
    }

    pub fn add_transitions(&mut self, transitions: StateTransitions) {
        for in_state in transitions.in_states {
            let transition = StateTransition {
                in_state,
                event: transitions.event.clone(),
                guard: transitions.guard.clone(),
                action: transitions.action.clone(),
                out_state: transitions.out_state.clone(),
            };
            self.transitions.push(transition);
        }
    }
}

impl parse::Parse for StateMachine {
    fn parse(input: parse::ParseStream) -> parse::Result<Self> {
        let mut statemachine = StateMachine::new();

        loop {
            // If the last line ends with a comma this is true
            if input.is_empty() {
                break;
            }

            match input.parse::<Ident>()?.to_string().as_str() {
                "transitions" => {
                    input.parse::<Token![:]>()?;
                    if input.peek(token::Brace) {
                        let content;
                        braced!(content in input);
                        loop {
                            if content.is_empty() {
                                break;
                            }

                            let transitions: StateTransitions = content.parse()?;
                            statemachine.add_transitions(transitions);

                            // No comma at end of line, no more transitions
                            if content.is_empty() {
                                break;
                            }

                            if content.parse::<Token![,]>().is_err() {
                                break;
                            };
                        }
                    }
                }
                "name" => {
                    input.parse::<Token![:]>()?;
                    statemachine.name = Some(input.parse::<Ident>()?);
                }

                "states_attr" => {
                    input.parse::<Token![:]>()?;
                    statemachine.states_attr = Attribute::parse_outer(input)?;
                }

                "events_attr" => {
                    input.parse::<Token![:]>()?;
                    statemachine.events_attr = Attribute::parse_outer(input)?;
                }

                "entry_exit_async" => {
                    input.parse::<Token![:]>()?;
                    let entry_exit_async: syn::LitBool = input.parse()?;
                    if entry_exit_async.value {
                        statemachine.entry_exit_async = true;
                    }
                }

                keyword => {
                    return Err(parse::Error::new(
                        input.span(),
                        format!(
                            "Unknown keyword {}. Support keywords: [\"name\", \
                                \"transitions\", \
                                \"states_attr\", \
                                \"events_attr\", \
                                \"entry_exit_async\"
                                ]",
                            keyword
                        ),
                    ))
                }
            }

            // No comma at end of line, no more transitions
            if input.is_empty() {
                break;
            }

            if input.parse::<Token![,]>().is_err() {
                break;
            };
        }

        Ok(statemachine)
    }
}
