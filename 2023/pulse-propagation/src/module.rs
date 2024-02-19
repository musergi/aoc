use crate::signal::Signal;

pub enum Module {
    Broadcast(Vec<String>),
    FlipFlop {
        target: Vec<String>,
        state: State,
    },
    Conjunction {
        target: Vec<String>,
        last_signals: Vec<(String, Signal)>,
    },
}
impl Module {
    pub fn process(&mut self, input: &str, signal: Signal) -> Vec<(String, Signal)> {
        match self {
            Module::Broadcast(connections) => connections
                .iter()
                .map(|connection| (connection.to_string(), signal))
                .collect(),
            Module::FlipFlop { target, state } => match signal {
                Signal::Low => {
                    state.swap();
                    let pulse = match state {
                        State::On => Signal::High,
                        State::Off => Signal::Low,
                    };
                    target
                        .iter()
                        .map(|target| (target.to_string(), pulse))
                        .collect()
                }
                Signal::High => Vec::new(),
            },
            Module::Conjunction {
                target,
                last_signals,
            } => {
                last_signals
                    .iter_mut()
                    .find(|last| last.0 == input)
                    .unwrap()
                    .1 = signal;
                let pulse = if last_signals.iter().all(|signal| signal.1 == Signal::High) {
                    Signal::Low
                } else {
                    Signal::High
                };
                target
                    .iter()
                    .map(|target| (target.to_string(), pulse))
                    .collect()
            }
        }
    }
}

pub enum State {
    On,
    Off,
}

impl State {
    fn swap(&mut self) {
        *self = match self {
            State::On => State::Off,
            State::Off => State::On,
        }
    }
}
