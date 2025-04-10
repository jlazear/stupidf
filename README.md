# stupidf


`stupidf` is a library for limited parsing of STDF files. The `STDF` structure can be used
directly in rust, or alternatively sent out to Python using the `parse_stdf` function.

STDF is the [Standard Test Data Format](https://en.wikipedia.org/wiki/Standard_Test_Data_Format) and is commonly used for high-volume test of semiconductors in Automated Test Equipment (ATE) systems. 

The purpose of the library is to quickly and efficiently parse STDF files (which are a fairly unfriendly binary linked list-based format) into more friendly [polars](https://pola.rs/) [DataFrame](https://docs.pola.rs/user-guide/concepts/data-types-and-structures/#dataframe) format. 

Not all record types are implemented because they're not relevant for my purposes. Implementing new records is straight-forward, following the others. 

# Example

In rust

```
use stupidf::data::STDF;
use polars::prelude::*;

let verbose = false;
if let Ok(stdf) = STDF::from_fname(&fname, verbose) {
    let df: DataFrame = (&stdf.test_data).into();
    let df_fmti: DataFrame = (&stdf.test_data.test_information).into();
    println!("{df:#?}");
    println!("{df_fmti}");
    }
```

Also contains Python bindings to this functionality, e.g.

```
   import stupidf as sf
   stdf = sf.parse_stdf("my_stdf.stdf")
   stdf['df']
````

# Installation

To install the rust CLI binary:

```cargo install stupidf```

To add the `stupidf` library to a rust project, add to the `Cargol.toml`:

```
[dependencies]
<... snip ...>
stupidf = "0.1.0"
```

or execute from the command line in your rust project

```cargo add stupidf```

To install the Python bindings and pre-built wheel (linux and win currently):

```pip install stupidf```

## Building from source

The rust library can be compiled simply with

```cargo build```

The Python bindings can be made using [`maturin`](https://www.maturin.rs/). Activate the desired virtualenv, then install `maturin` and use it to build the bindings

```
pip install maturin
maturin develop
```

# Development

If you're seeing issues with `pyo3` recompiling on every build, even when there are no `pyo3`-related changes, then you're most likely running into [this issue](https://github.com/PyO3/pyo3/issues/1708). 

Consider setting the `PYO3_PYTHON` env variable adding to your `Cargo.toml` or terminal:

```
[env]
PYO3_PYTHON = /path/to/python
```

and also ensuring this is the Python interpreter used by your IDE. E.g. if using nvim, activate the venv before starting nvim. 
