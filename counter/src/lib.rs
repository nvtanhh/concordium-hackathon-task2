//! # A Concordium V1 smart contract
use concordium_std::*;
use core::fmt::Debug;

/// Your smart contract state.
#[derive(Serialize, SchemaType, Clone)]
pub struct State {
    counter: i32,
}

#[derive(Serialize, SchemaType)]
struct SetCounterParam {
    new_counter: i32,
}

/// Your smart contract errors.
#[derive(Debug, PartialEq, Eq, Reject, Serial, SchemaType)]
enum Error {
    /// Failed parsing the parameter.
    #[from(ParseError)]
    ParseParamsError,
}

/// Init function that creates a new smart contract.
#[init(contract = "counter")]
fn init<S: HasStateApi>(
    _ctx: &impl HasInitContext,
    _state_builder: &mut StateBuilder<S>,
) -> InitResult<State> {
    // Your code

    Ok(State { counter: 0 })
}

#[receive(
    contract = "counter",
    name = "set_counter",
    parameter = "SetCounterParam",
    mutable
)]
fn set_counter<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    _host: &mut impl HasHost<State, StateApiType = S>,
) -> Result<(), Error> {
    let param: SetCounterParam = ctx.parameter_cursor().get()?;
    _host.state_mut().counter = param.new_counter;

    Ok(())
}

/// View function that returns the content of the state.
#[receive(contract = "counter", name = "view", return_value = "State")]
fn view<'b, S: HasStateApi>(
    _ctx: &impl HasReceiveContext,
    host: &'b impl HasHost<State, StateApiType = S>,
) -> ReceiveResult<&'b State> {
    Ok(host.state())
}

#[concordium_cfg_test]
mod tests {
    use super::*;
    use test_infrastructure::*;

    type ContractResult<A> = Result<A, Error>;

    #[concordium_test]
    /// Test that initializing the contract succeeds with some state.
    fn test_init() {
        let ctx = TestInitContext::empty();

        let mut state_builder = TestStateBuilder::new();

        let state_result = init(&ctx, &mut state_builder);
        state_result.expect_report("Contract initialization results in error");
    }

    #[concordium_test]
    fn test_set_counter() {
        let ctx = TestInitContext::empty();

        let mut state_builder = TestStateBuilder::new();

        // Initializing state
        let initial_state = init(&ctx, &mut state_builder).expect("Initialization should pass");

        let mut ctx = TestReceiveContext::empty();

        let new_counter = 10;

        let param = SetCounterParam {
            new_counter: new_counter,
        };
        let parameter_bytes = to_bytes(&param);
        ctx.set_parameter(&parameter_bytes);

        let mut host = TestHost::new(initial_state, state_builder);

        // Call the contract function.
        let result: ContractResult<()> = set_counter(&ctx, &mut host);

        // Check the result.
        claim!(result.is_ok(), "Results in rejection");

        claim_eq!(host.state().counter, new_counter, "Didn't set count");
    }
}
