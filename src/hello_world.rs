#![feature(plugin)]
#![plugin(interpolate_idents)]

#[macro_use] extern crate cpython;

use cpython::{PyResult, Python, PyTuple};

mod foo {
    pub fn hello_world() -> () {
        println!("Hello, world!");
    }
}

py_module_initializer!(libhello_world, |_py, m| {
    try!(m.add("__doc__", "Module docstring"));
    try!(m.add("hello_world", py_fn!(hello_world)));
    Ok(())
});

fn hello_world<'p>(py: Python<'p>, args: &PyTuple<'p>) -> PyResult<'p, u64> {
    foo::hello_world();
    Ok(0)
}
