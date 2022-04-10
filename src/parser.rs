use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyLong};

use crate::convert_result;
use crate::replay::{Replay, ReplayBody, ReplayHeader};

use faf_replay_parser::scfa::{Parser, ParserBuilder};

#[pyclass(name = "Parser")]
pub struct ParserWrap {
    parser: Parser,
}

#[pymethods]
impl ParserWrap {
    #[new]
    fn new(
        limit: Option<usize>,
        commands: Option<&PyAny>,
        save_commands: Option<bool>,
        stop_on_desync: Option<bool>,
    ) -> PyResult<ParserWrap> {
        // Configure ParserBuilder from arguments
        let mut builder = ParserBuilder::new().limit(limit);
        if let Some(seq) = commands {
            let mut commands = match seq.len() {
                Ok(len) => Vec::with_capacity(len),
                Err(_) => Vec::new(),
            };
            for any in seq
                .iter()
                .map_err(|_| PyTypeError::new_err("'commands' must be iterable"))?
            {
                commands.push(
                    any?.downcast::<PyLong>()
                        .map_err(|_| PyTypeError::new_err("command must be an integer"))?
                        .extract()?,
                );
            }
            builder = builder.commands(&commands);
        } else {
            builder = builder.commands_default();
        }

        // Default `save_commands` to false for the python bindings as converting thousands of
        // commands to python dictionaries can be quite expensive.
        builder = builder
            .save_commands(save_commands.unwrap_or(false))
            .stop_on_desync(stop_on_desync.unwrap_or(true));

        Ok(ParserWrap {
            parser: builder.build(),
        })
    }

    /// Parse a replay
    #[pyo3(text_signature = "(data)")]
    fn parse(&self, py: Python, data: &PyBytes) -> PyResult<Replay> {
        let mut bytes = data.as_bytes();
        Ok(Replay(py.allow_threads(|| {
            convert_result(self.parser.parse(&mut bytes))
        })?))
    }

    /// Parse a replay header
    #[pyo3(text_signature = "(data)")]
    fn parse_header(&self, py: Python, data: &PyBytes) -> PyResult<ReplayHeader> {
        let mut bytes = data.as_bytes();
        Ok(ReplayHeader(py.allow_threads(|| {
            convert_result(self.parser.parse_header(&mut bytes))
        })?))
    }
    /// Parse a replay body. This implies that the header has already been parsed in order for
    /// `data` to be at the correct offset.
    #[pyo3(text_signature = "(data)")]
    fn parse_body(&self, py: Python, data: &PyBytes) -> PyResult<ReplayBody> {
        let mut bytes = data.as_bytes();
        Ok(ReplayBody(py.allow_threads(|| {
            convert_result(self.parser.parse_body(&mut bytes))
        })?))
    }
}
