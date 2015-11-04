#![feature(plugin)]
#![plugin(interpolate_idents)]

#[macro_use] extern crate cpython;
use cpython::{PyResult, Python, PyTuple, PyObject, PyErr, exc, ToPyObject, PythonObject};

#[macro_use(bson, doc)]
extern crate bson;
extern crate mongodb;

use bson::Bson;
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;

mod foo {
    pub fn hello_world() -> () {
        println!("Hello, world!");
    }

	pub fn hello_int(i: i64) -> () {
		println!("Hello, {}", i);
	}
}

py_module_initializer!(libhello_world, |_py, m| {
    try!(m.add("__doc__", "Module docstring"));
    try!(m.add("hello_world", py_fn!(hello_world)));
    try!(m.add("hello_int", py_fn!(hello_int)));
    try!(m.add("hello_string", py_fn!(hello_string)));
    try!(m.add("hello_object", py_fn!(hello_object)));
    Ok(())
});

fn hello_world<'p>(py: Python<'p>, args: &PyTuple<'p>) -> PyResult<'p, PyObject<'p>> {
    foo::hello_world();
    Ok(py.None())
}

fn err_expected<'p>(py: Python<'p>, what: &str) -> PyErr<'p> {
	let msg = format!("expected {}", &what);
	return PyErr::new_lazy_init(py.get_type::<exc::ValueError>(), Some(msg.to_py_object(py).into_object()));
}

fn hello_int<'p>(py: Python<'p>, args: &PyTuple<'p>) -> PyResult<'p, PyObject<'p>> {
	if args.len() == 0 { return Err(err_expected(py, "integer")); }
	let arg0 = match args.get_item(0).extract::<i64>() {
		Ok(x) => x,
		Err(_) => return Err(err_expected(py, "integer")),
	};
	foo::hello_int(arg0);
	Ok(py.None())
}

fn hello_string<'p>(py: Python<'p>, args: &PyTuple<'p>) -> PyResult<'p, PyObject<'p>> {
	if args.len() == 0 { return Err(err_expected(py, "string")); }
    let arg0string = args.get_item(0).to_string();
    let arg0: &str = &arg0string[1..arg0string.len()-1];

    println!("arg0string = {}", arg0string);
    println!("arg0 = {}", arg0);

    Ok(py.None())
}

fn hello_object<'p>(py: Python<'p>, args: &PyTuple<'p>) -> PyResult<'p, PyObject<'p>> {
	if args.len() == 0 { return Err(err_expected(py, "object")); }
	let arg0 = args.get_item(0);
	println!("Hello, {}", arg0);
	Ok(py.None())
}

fn hello_numpy<'p>(py: Python<'p>, args: &PyTuple<'p>) -> PyResult<'p, PyObject<'p>> {
	if args.len() == 0 { return Err(err_expected(py, "numpy")); }
	let arg0 = args.get_item(0);
	println!("Hello, {:?}", arg0);
	Ok(py.None())
}
