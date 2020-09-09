use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyDict};
use std::collections::HashMap;

pub struct LuaObject(pub faf_replay_parser::lua::LuaObject);
pub struct LuaTable(
    pub HashMap<faf_replay_parser::lua::LuaObject, faf_replay_parser::lua::LuaObject>,
);

impl IntoPy<PyObject> for LuaObject {
    fn into_py(self, py: Python) -> PyObject {
        use faf_replay_parser::lua::LuaObject::*;

        match self.0 {
            Float(f) => f.into_py(py),
            String(s) => PyBytes::new(py, s.as_bytes()).into_py(py),
            Unicode(s) => s.into_py(py),
            Nil => ().into_py(py),
            Bool(b) => b.into_py(py),
            Table(t) => LuaTable(t).into_py(py),
        }
    }
}

impl IntoPy<PyObject> for LuaTable {
    fn into_py(self, py: Python) -> PyObject {
        let res = PyDict::new(py);

        for (k, v) in self.0 {
            res.set_item::<PyObject, PyObject>(LuaObject(k).into_py(py), LuaObject(v).into_py(py))
                .unwrap();
        }

        res.into_py(py)
    }
}

pub fn table_into_py<K: IntoPy<PyObject>>(
    table: HashMap<K, faf_replay_parser::lua::LuaObject>,
    py: Python,
) -> PyObject {
    let res = PyDict::new(py);

    for (k, v) in table {
        res.set_item::<PyObject, PyObject>(k.into_py(py), LuaObject(v).into_py(py))
            .unwrap();
    }

    res.into_py(py)
}
