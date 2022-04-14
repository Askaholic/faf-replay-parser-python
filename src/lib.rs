extern crate faf_replay_parser;
extern crate pyo3;

use pyo3::create_exception;
use pyo3::exceptions;
use pyo3::prelude::*;
use pyo3::types::{PyByteArray, PyBytes};
use pyo3::wrap_pyfunction;
use std::io::ErrorKind;

use faf_replay_parser::scfa;

mod constants;
mod lua;
mod parser;
mod replay;

struct ReplayReadError(faf_replay_parser::ReplayReadError);

create_exception!(fafreplay, PyReplayReadError, exceptions::PyException);
create_exception!(fafreplay, PyReplayDesyncedError, PyReplayReadError);

fn convert_error(obj: faf_replay_parser::ReplayReadError) -> PyErr {
    use faf_replay_parser::ReplayReadError::*;

    match obj {
        IO(ref e) if e.kind() == ErrorKind::UnexpectedEof => {
            PyErr::new::<exceptions::PyEOFError, _>(format!("{}", e))
        }
        IO(e) => PyErr::from(e),
        MalformedUtf8(e) => {
            let gil = Python::acquire_gil();
            let py = gil.python();
            PyErr::from_value(
                exceptions::PyUnicodeDecodeError::new_utf8(py, e.as_bytes(), e.utf8_error())
                    .unwrap(),
            )
        }
        Desynced(tick) => PyErr::new::<PyReplayDesyncedError, _>(tick),
        Malformed(msg) => PyErr::new::<PyReplayReadError, _>(msg),
    }
}

fn convert_result<T>(res: Result<T, faf_replay_parser::ReplayReadError>) -> PyResult<T> {
    match res {
        Ok(item) => Ok(item),
        Err(err) => Err(convert_error(err)),
    }
}

impl From<faf_replay_parser::ReplayReadError> for ReplayReadError {
    fn from(obj: faf_replay_parser::ReplayReadError) -> ReplayReadError {
        ReplayReadError(obj)
    }
}

/// body_offset(replay: Union[bytes, bytearray]) -> int
///
/// Find the offset at which the body starts by parsing the header.
/// Raises `ReplayReadError` if the header data is malformed.
#[pyfunction]
#[pyo3(text_signature = "(replay)")]
fn body_offset(any: &PyAny) -> PyResult<usize> {
    if any.is_instance_of::<PyBytes>()? {
        let bytes = any.downcast::<PyBytes>().unwrap();
        return Ok(convert_result(scfa::body_offset(bytes.as_bytes()))?);
    } else if any.is_instance_of::<PyByteArray>()? {
        let _py = Python::acquire_gil();
        let bytearray = any.downcast::<PyByteArray>().unwrap();
        return unsafe { Ok(convert_result(scfa::body_offset(bytearray.as_bytes()))?) };
    }

    Err(PyErr::new::<exceptions::PyTypeError, _>(
        "'replay' must be bytes or bytearray",
    ))
}

/// body_ticks(body: Union[bytes, bytearray]) -> int
///
/// Count the number of ticks in the replay body without checking for desyncs.
/// Raises `ReplayReadError` if the body data is malformed.
#[pyfunction]
#[pyo3(text_signature = "(body)")]
fn body_ticks(any: &PyAny) -> PyResult<u32> {
    if any.is_instance_of::<PyBytes>()? {
        let bytes = any.downcast::<PyBytes>().unwrap();
        return Ok(convert_result(scfa::body_ticks(bytes.as_bytes()))?);
    } else if any.is_instance_of::<PyByteArray>()? {
        let _py = Python::acquire_gil();
        let bytearray = any.downcast::<PyByteArray>().unwrap();
        return unsafe { Ok(convert_result(scfa::body_ticks(bytearray.as_bytes()))?) };
    }

    Err(PyErr::new::<exceptions::PyTypeError, _>(
        "'body' must be bytes or bytearray",
    ))
}

/// Supreme Commander Forged Alliance replay parser
#[pymodule]
fn _fafreplay(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<parser::ParserWrap>()?;
    m.add("ReplayReadError", py.get_type::<PyReplayReadError>())?;
    m.add(
        "ReplayDesyncedError",
        py.get_type::<PyReplayDesyncedError>(),
    )?;
    m.add_wrapped(wrap_pyfunction!(body_offset))?;
    m.add_wrapped(wrap_pyfunction!(body_ticks))?;

    constants::add_constants(m)?;

    Ok(())
}
