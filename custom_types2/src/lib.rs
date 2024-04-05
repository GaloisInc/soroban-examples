#![no_std]
use soroban_sdk::{contractimpl, contracttype, Env, Symbol};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct State (
    u32, /* count */
    u32, /* last_incr */
);

const STATE: Symbol = Symbol::short("STATE");

pub struct IncrementContract;

#[contractimpl]
impl IncrementContract {
    /// Increment increments an internal counter, and returns the value.
    pub fn increment(env: Env, incr: u32) -> u32 {
        // Get the current count.
        let mut state = Self::get_state(env.clone());

        // Increment the count.
        state.0 += incr; /* count */
        state.1 = incr; /* last_incr */

        // Save the count.
        env.storage().set(&STATE, &state);

        // Return the count to the caller.
        state.0
    }
    /// Return the current state.
    pub fn get_state(env: Env) -> State {
        env.storage()
            .get(&STATE)
            .unwrap_or(Ok(State (
                0, /* count */
                0, /* last_incr */
            ))) // If no value set, assume 0.
            .unwrap() // Panic if the value of COUNTER is not a State.
    }
}

mod test;
