use faf_replay_parser::lua::LuaObject;
use pyo3::prelude::*;
use pyo3::types::PyBytes;

impl IntoPy<PyObject> for LuaObject {
    fn into_py(self, py: Python) -> PyObject {
        use LuaObject::*;

        match self {
            Float(f) => f.into_py(py),
            String(s) => PyBytes::new(py, s.as_bytes()).into_py(py),
            Unicode(s) => s.into_py(py),
            Nil => ().into_py(py),
            Bool(b) => b.into_py(py),
            Table(t) => t.into_py(py),
        }
    }
}
