extern crate faf_replay_parser;
extern crate pyo3;

use pyo3::conversion::AsPyPointer;
use pyo3::create_exception;
use pyo3::exceptions;
use pyo3::ffi;
use pyo3::prelude::*;
use pyo3::type_object::PyTypeInfo;
use pyo3::types::{PyByteArray, PyBytes};
use pyo3::wrap_pyfunction;
use std::io::ErrorKind;

use faf_replay_parser::scfa;

mod constants;
mod lua;
mod parser;
mod replay;

create_exception!(fafreplay, ReplayReadError, exceptions::Exception);
create_exception!(fafreplay, ReplayDesyncedError, ReplayReadError);

impl From<faf_replay_parser::ReplayReadError> for PyErr {
    fn from(obj: faf_replay_parser::ReplayReadError) -> PyErr {
        use faf_replay_parser::ReplayReadError::*;

        match obj {
            IO(ref e) if e.kind() == ErrorKind::UnexpectedEof => {
                PyErr::new::<exceptions::EOFError, _>(format!("{}", e))
            }
            IO(e) => PyErr::from(e),
            MalformedUtf8(e) => {
                let gil = Python::acquire_gil();
                let py = gil.python();
                PyErr::from_instance(
                    exceptions::UnicodeDecodeError::new_utf8(py, e.as_bytes(), e.utf8_error())
                        .unwrap(),
                )
            }
            Desynced(tick) => PyErr::new::<ReplayDesyncedError, _>(tick),
            Malformed(msg) => PyErr::new::<ReplayReadError, _>(msg),
        }
    }
}

/// body_offset(replay: Union[bytes, bytearray]) -> int
///
/// Find the offset at which the body starts by parsing the header.
/// Raises `ReplayReadError` if the header data is malformed.
#[pyfunction]
#[text_signature = "(replay)"]
fn body_offset(_py: Python, any: &PyAny) -> PyResult<usize> {
    if PyBytes::is_instance(any) {
        let bytes = any.downcast::<PyBytes>().unwrap();
        return Ok(scfa::body_offset(bytes.as_bytes())?);
    } else if PyByteArray::is_instance(any) {
        // Sadly PyByteArray doesn't expose this API
        let slice = unsafe {
            let buffer = ffi::PyByteArray_AsString(any.as_ptr()) as *mut u8;
            let length = ffi::PyByteArray_Size(any.as_ptr()) as usize;
            std::slice::from_raw_parts_mut(buffer, length)
        };
        return Ok(scfa::body_offset(slice)?);
    }

    Err(PyErr::new::<exceptions::TypeError, _>(
        "'replay' must be bytes or bytearray",
    ))
}

/// body_ticks(body: Union[bytes, bytearray]) -> int
///
/// Count the number of ticks in the replay body without checking for desyncs.
/// Raises `ReplayReadError` if the body data is malformed.
#[pyfunction]
#[text_signature = "(body)"]
fn body_ticks(_py: Python, any: &PyAny) -> PyResult<u32> {
    if PyBytes::is_instance(any) {
        let bytes = any.downcast::<PyBytes>().unwrap();
        return Ok(scfa::body_ticks(bytes.as_bytes())?);
    } else if PyByteArray::is_instance(any) {
        let bytearray = any.downcast::<PyByteArray>().unwrap();
        // Sadly PyByteArray doesn't expose this API
        let slice = unsafe {
            let buffer = ffi::PyByteArray_AsString(any.as_ptr()) as *mut u8;
            let length = ffi::PyByteArray_Size(any.as_ptr()) as usize;
            std::slice::from_raw_parts_mut(buffer, length)
        };
        return Ok(scfa::body_ticks(slice)?);
    }

    Err(PyErr::new::<exceptions::TypeError, _>(
        "'body' must be bytes or bytearray",
    ))
}

/// Supreme Commander Forged Alliance replay parser
#[pymodule]
fn fafreplay(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<parser::ParserWrap>()?;
    m.add("ReplayReadError", py.get_type::<ReplayReadError>())?;
    m.add("ReplayDesyncedError", py.get_type::<ReplayDesyncedError>())?;
    m.add_wrapped(wrap_pyfunction!(body_offset))?;
    m.add_wrapped(wrap_pyfunction!(body_ticks))?;

    constants::add_constants(m)?;

    Ok(())
}
