#![crate_type = "dylib"]

use cpython::{PyObject, PyResult, py_module_initializer, py_class};

py_module_initializer!(custom_class, |py, m| {
    m.add(py, "__doc__", "Module documentation string")?;
    m.add_class::<MyType>(py)?;
    Ok(())
});

py_class!(class MyType |py| {
    data data: i32;
    def __new__(_cls, arg: i32) -> PyResult<MyType> {
        MyType::create_instance(py, arg)
    }
    def a(&self) -> PyResult<PyObject> {
        println!("a() was called with self={:?}", self.data(py));
        Ok(py.None())
    }
});

