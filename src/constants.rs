use faf_replay_parser::scfa::replay::replay_command;
use pyo3::prelude::*;
use pyo3::wrap_pymodule;

#[rustfmt::skip]
pub fn add_constants(m: &PyModule) -> PyResult<()> {
    #[pymodule]
    fn commands(_py: Python, m: &PyModule) -> PyResult<()> {
        use replay_command::*;

        m.add(NAMES[ADVANCE as usize], ADVANCE)?;
        m.add(NAMES[SET_COMMAND_SOURCE as usize], SET_COMMAND_SOURCE)?;
        m.add(NAMES[COMMAND_SOURCE_TERMINATED as usize], COMMAND_SOURCE_TERMINATED)?;
        m.add(NAMES[VERIFY_CHECKSUM as usize], VERIFY_CHECKSUM)?;
        m.add(NAMES[REQUEST_PAUSE as usize], REQUEST_PAUSE)?;
        m.add(NAMES[RESUME as usize], RESUME)?;
        m.add(NAMES[SINGLE_STEP as usize], SINGLE_STEP)?;
        m.add(NAMES[CREATE_UNIT as usize], CREATE_UNIT)?;
        m.add(NAMES[CREATE_PROP as usize], CREATE_PROP)?;
        m.add(NAMES[DESTROY_ENTITY as usize], DESTROY_ENTITY)?;
        m.add(NAMES[WARP_ENTITY as usize], WARP_ENTITY)?;
        m.add(NAMES[PROCESS_INFO_PAIR as usize], PROCESS_INFO_PAIR)?;
        m.add(NAMES[ISSUE_COMMAND as usize], ISSUE_COMMAND)?;
        m.add(NAMES[ISSUE_FACTORY_COMMAND as usize], ISSUE_FACTORY_COMMAND)?;
        m.add(NAMES[INCREASE_COMMAND_COUNT as usize], INCREASE_COMMAND_COUNT)?;
        m.add(NAMES[DECREASE_COMMAND_COUNT as usize], DECREASE_COMMAND_COUNT)?;
        m.add(NAMES[SET_COMMAND_TARGET as usize], SET_COMMAND_TARGET)?;
        m.add(NAMES[SET_COMMAND_TYPE as usize], SET_COMMAND_TYPE)?;
        m.add(NAMES[SET_COMMAND_CELLS as usize], SET_COMMAND_CELLS)?;
        m.add(NAMES[REMOVE_COMMAND_FROM_QUEUE as usize], REMOVE_COMMAND_FROM_QUEUE)?;
        m.add(NAMES[DEBUG_COMMAND as usize], DEBUG_COMMAND)?;
        m.add(NAMES[EXECUTE_LUA_IN_SIM as usize], EXECUTE_LUA_IN_SIM)?;
        m.add(NAMES[LUA_SIM_CALLBACK as usize], LUA_SIM_CALLBACK)?;
        m.add(NAMES[END_GAME as usize], END_GAME)?;
        m.add("MAX", MAX)?;

        m.add("NAMES", NAMES)?;

        Ok(())
    }

    m.add_wrapped(wrap_pymodule!(commands))?;

    Ok(())
}
