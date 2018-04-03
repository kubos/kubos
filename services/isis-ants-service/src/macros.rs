//
// Copyright (C) 2018 Kubos Corporation
//
// Licensed under the Apache License, Version 2.0 (the "License")
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
#![macro_use]

/// Iterate through a failure::Error and concatenate the error
/// and all its causes into a single string
///
/// # Examples
///
/// ```
/// #[macro_use]
/// extern crate failure;
///
/// use failure::Fail;
///
/// #[derive(Fail, Display, Debug)]
/// pub enum RootError {
///     #[display(fmt = "RootError: {}", message)]
///     RootError { message: String },
/// }
///
/// #[derive(Fail, Display, Debug)]
/// pub enum TopError {
///     #[display(fmt = "TopError: {}", message)]
///     Error {
///         #[fail(cause)]
///         cause: RootError,
///         message: String,
///     },
/// }
///
/// fn main() {
///     let chain: TopError = TopError::Error {
///         cause: RootError::RootError { message: "root".to_owned() },
///         message: "top".to_owned(),
///     };
///
///     let errors = process_errors!(chain);
///     assert_eq!(errors, "TopError: top, RootError: root");
///
///     let errors = process_errors!(chain, '\n');
///     assert_eq!(errors, "TopError: top\nRootError: root");
/// }
/// ```
///
macro_rules! process_errors {
    ($err:ident) => (process_errors!($err, ", "));
    ($err:ident, $delim:expr) => {{    
        {
            let mut results = String::new();
            let mut chain = $err.causes();
            
            if let Some(err) = chain.next() {
                results.push_str(&format!("{}", err));
        
                for err in chain {
                    results.push_str(&format!("{}{}", $delim, err));
                }
            }

            results
        }
    }};
}

/// Convenience macro to push an error string onto the master errors vector
///
/// # Examples
///
/// ```
/// # fn main() {
/// let master_err = RefCell::new(vec![]);
///
/// push_err!(master_err, "Message1".to_owned());
/// push_err!(master_err, "Message2".to_owned());
///
/// assert_eq!(
///     vec!["Message1".to_owned(), "Message2".to_owned()],
///     master_err.borrow().clone()
/// );
/// # }
/// ```
macro_rules! push_err {
    ($master:expr, $err:expr) => {{
            if let Ok(mut master_vec) = $master.try_borrow_mut() {
                master_vec.push($err);
            }
        }}
}

/// Execute a function and return a tuple containing:
///   a) A String with any errors which were encountered
///   b) A boolean to indicate whether the function ran successfully
///   c) Any data returned by the function
/// Optionally:
///   Add the error string to the master error string for later consumption,
///   prefixed with the name of the function being called
///
/// # Examples
///
/// ```
/// #[macro_use]
/// extern crate failure;
///
/// use failure::Fail;
///
/// #[derive(Fail, Display, Debug)]
/// pub enum RootError {
///     #[display(fmt = "RootError: {}", message)]
///     RootError { message: String },
/// }
///
/// #[derive(Fail, Display, Debug)]
/// pub enum TopError {
///     #[display(fmt = "TopError: {}", message)]
///     Error {
///         #[fail(cause)]
///         cause: RootError,
///         message: String,
///     },
/// }
///
/// fn test_func(fail: bool, output: String) -> Result<String, Error> {
///     match fail {
///         true => {
///             let chain: TopError = TopError::Error {
///                 cause: RootError::RootError { message: "root".to_owned() },
///                 message: "top".to_owned(),
///             };
///
///             Err(chain.into())
///         }
///         false => Ok(output),
///     }
/// }
///
/// fn main() {
///     let master_err = RefCell::new(vec![]);
///     let (errors, success, data) = run!(test_func(true, "test".to_owned()), master_err);
///
///     assert_eq!(errors, "TopError: top, RootError: root");
///     assert_eq!(success, false);
///     assert_eq!(data, None);
///     assert_eq!(
///         vec!["test_func: TopError: top, RootError: root".to_owned()],
///         master_err.borrow().clone()
///     );
/// }
/// ```
macro_rules! run {
    ($func:expr) => {{
            let (errors, success, data) = match $func {
                Ok(v) => (String::from(""), true, Some(v)),
                Err(e) => (process_errors!(e, ", "), false, None),
            };
            
            (errors, success, data)
        }};
    ($func:expr, $master:expr) => {{
        {
            let (errors, success, data) = run!($func);
            
            if !errors.is_empty() {
                // We want to know which function threw these particular errors, 
                // but we don't want to print the entire expression, so using split
                // to go from
                //     self.my.func(arg1, arg2)
                // to this
                //     func
                // TODO: This isn't perfect or particularly pretty. Is there a better way?
                let mut name = stringify!($func).split('(').next().unwrap();
                name = name.split(&[':','.'][..]).last().unwrap();
                push_err!($master, format!("{}: {}", name, errors));
            }
            
            (errors, success, data)
        }
    }};
}

#[cfg(test)]
mod tests {
    use failure::{Fail, Error};
    use std::cell::RefCell;

    #[derive(Fail, Display, Debug)]
    pub enum RootError {
        #[display(fmt = "RootError: {}", message)]
        RootError { message: String },
    }

    #[derive(Fail, Display, Debug)]
    pub enum TopError {
        #[display(fmt = "TopError: {}", message)]
        Error {
            #[fail(cause)]
            cause: RootError,
            message: String,
        },
    }

    fn test_func(fail: bool, output: String) -> Result<String, Error> {
        match fail {
            true => {
                let chain: TopError = TopError::Error {
                    cause: RootError::RootError { message: "root".to_owned() },
                    message: "top".to_owned(),
                };

                Err(chain.into())
            }
            false => Ok(output),
        }
    }

    #[test]
    fn process_errors_default() {
        let chain: TopError = TopError::Error {
            cause: RootError::RootError { message: "root".to_owned() },
            message: "top".to_owned(),
        };

        let errors = process_errors!(chain);
        assert_eq!(errors, "TopError: top, RootError: root");
    }

    #[test]
    fn process_errors_delim() {
        let chain: TopError = TopError::Error {
            cause: RootError::RootError { message: "root".to_owned() },
            message: "top".to_owned(),
        };

        let errors = process_errors!(chain, '\n');
        assert_eq!(errors, "TopError: top\nRootError: root");
    }

    #[test]
    fn push_err() {
        let master_err = RefCell::new(vec![]);

        push_err!(master_err, "Message".to_owned());

        assert_eq!(vec!["Message".to_owned()], master_err.borrow().clone());
    }

    #[test]
    fn push_err_mult() {
        let master_err = RefCell::new(vec![]);

        push_err!(master_err, "Message1".to_owned());
        push_err!(master_err, "Message2".to_owned());

        assert_eq!(
            vec!["Message1".to_owned(), "Message2".to_owned()],
            master_err.borrow().clone()
        );
    }

    #[test]
    fn run_default() {
        let (errors, success, data) = run!(test_func(true, "test".to_owned()));

        assert_eq!(errors, "TopError: top, RootError: root");
        assert_eq!(success, false);
        assert_eq!(data, None);
    }

    #[test]
    fn run_push() {
        let master_err = RefCell::new(vec![]);
        let (errors, success, data) = run!(test_func(true, "test".to_owned()), master_err);

        assert_eq!(errors, "TopError: top, RootError: root");
        assert_eq!(success, false);
        assert_eq!(data, None);
        assert_eq!(
            vec!["test_func: TopError: top, RootError: root".to_owned()],
            master_err.borrow().clone()
        );
    }

    #[test]
    fn run_push_good() {
        let master_err = RefCell::new(vec![]);
        let (errors, success, data) = run!(test_func(false, "test".to_owned()), master_err);

        assert_eq!(errors, "");
        assert_eq!(success, true);
        assert_eq!(data.unwrap(), "test");
        let test_vec: Vec<String> = vec![];
        assert_eq!(test_vec, master_err.borrow().clone());
    }
}
