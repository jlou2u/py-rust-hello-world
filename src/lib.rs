#[macro_use] extern crate cpython;
use cpython::{PyResult, Python};

py_module_initializer!(libhello_world, initlibhello_world, PyInit_libhello_world, |py, m| {
    try!(m.add(py, "__doc__", "Module docstring"));
    try!(m.add(py, "hello_int", py_fn!(py, hello_int_py(i: i64))));
    try!(m.add(py, "hello_numpy", py_fn!(py, hello_numpy_py(i: Vec<f64>))));
    try!(m.add(py, "hello_numpy2d", py_fn!(py, hello_numpy2d_py(i: Vec<Vec<f64>>))));
    Ok(())
});

fn hello_numpy(ia:Vec<f64>) -> String {
    format!("Hello {:?}", ia)
}

fn hello_numpy_py(_: Python, ia:Vec<f64>) -> PyResult<String> {
    let result = hello_numpy(ia);
    Ok(result)
}

fn hello_numpy2d(ia:Vec<Vec<f64>>) -> String {
    format!("Hello {:?}", ia)
}

fn hello_numpy2d_py(_: Python, ia:Vec<Vec<f64>>) -> PyResult<String> {
    let result = hello_numpy2d(ia);
    Ok(result)
}

fn hello_int(i:i64) -> String {
    format!("Hello {}", i)
}

fn hello_int_py(_: Python, i:i64) -> PyResult<String> {
    let result = hello_int(i);
    Ok(result)
}
