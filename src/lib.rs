#[crate_id = "fsm#0.1"];
#[crate_type = "lib"];
#[feature(macro_rules)];

//!
//! Rust-FSM provides a Finite State Machine implementation.
//!
//! ```rust
//! extern mod fsm;
//!
//! fn main() {
//!     // Create a new set of states.
//!     // `State` defines a new module where each state is defined in.
//!     defstates! (State -> Unlocked, Locked);
//!
//!     // Create a new FSM and pass an initial state:
//!     let mut machine = fsm::StateMachine::new(State::Unlocked);
//!
//!     // Do something when we lock the machine:
//!     machine.when(State::Locked, || {
//!         println!("We have locked it again.");
//!     });
//!
//!     machine.switch(State::Locked);
//! }
//! ```

/// Create a new module that will contain an enum for each State and
/// also implement the `Eq` trait for simple comparisons.
///
/// ```rust
/// defstates! (State -> Locked, Unlocked, Moved, Wrong, Weird)
/// assert_eq!(State::Locked, 0);
/// assert_eq!(State::Unlocked, 1);
/// // ...
/// ```
macro_rules! defstates(
    ($namespace:ident -> $($name:ident),+) => (
        mod $namespace {
            pub enum State {
                $(
                    $name,
                )+
            }

            impl Eq for State {
                fn eq(&self, rs: &State) -> bool {
                    *self as int == *rs as int
                }
            }
        }
    );
)

/// A representation of a state machine that holds the current state,
/// as well as an owned vector of tuple elements. The tuple contains the
/// state and a lambda, specified with a named lifetime.
pub struct StateMachine<'a, T> {
    /// Store the currently selected state
    currentState: T,
    exprs: ~[(T, 'a ||)]
}

/// Establish two generic types parameters: `'a` which defines the lifetime
/// of the closure/lambda to `.when` methods; and `T` which defines the type
/// of state object.
impl<'a, T: Eq> StateMachine<'a, T> {

    /// Creates a new instance of the `StateMachine` struct. We begin
    /// with an empty set of expressions and an initial state.
    pub fn new(initialState: T) -> StateMachine<T> {
        StateMachine { currentState: initialState, exprs: ~[] }
    }

    /// Transition/switch the current state to another one. This will trigger
    /// any `.when` expressions that match.
    pub fn switch(&mut self, nextState: T) {
        self.currentState = nextState;
        for expr in self.exprs.iter() {
            match *expr {
                (ref state, ref func) => {
                    if *state == self.currentState {
                        (*func)();
                    }
                }
            }
        }
    }

    /// Pass a lambda/closure whenever a specific state is triggered. This is
    /// typically how and where the logic goes. `'a` defines a named lifetime
    /// based on the lambda, because lambda's capture their environment.
    pub fn when(&mut self, state: T, func: 'a ||) {
        self.exprs.push((state, func));
    }
}

#[cfg(test)]
mod test {


    #[test]
    fn test_sm_new() {
        defstates! (State -> One);
        let sm = ::StateMachine::new(State::One);
        assert_eq!(sm.currentState, State::One);
    }

    // FIXME: Replace with the `defstates!` macro.
    #[test]
    fn test_sm_switch() {

        enum State {
            Unlocked = 0x01,
            Locked
        }

        impl Eq for State {
            fn eq(&self, rs: &State) -> bool {
                *self as int == *rs as int
            }
        }

        let mut sm = ::StateMachine::new(Unlocked);
        sm.switch(Locked);
        assert_eq!(sm.currentState as int, Locked as int);
    }

    // FIXME: Replace with the `defstates!` macro.
    #[test]
    fn test_when() {

        enum State {
            Unlocked = 0x01,
            Locked
        }

        /// FIXME: This model is **super** akward to present as an API.
        ///        Perhaps move away from enums and into custom types?
        impl Eq for State {
            fn eq(&self, rs: &State) -> bool {
                *self as int == *rs as int
            }
        }

        let mut sm = ::StateMachine::new(Unlocked);
        let mut called = false;

        sm.when(Locked, || {
            called = true;
            println!("Hello from Locked!");
        });

        assert_eq!(called, false);
        sm.switch(Locked);
        assert_eq!(called, true);
    }

    #[test]
    fn test_defstates_macro() {
        defstates! (State -> Woot, Wolf);
        assert_eq!(State::Woot as int, 0);
        assert_eq!(State::Wolf as int, 1);
    }
}