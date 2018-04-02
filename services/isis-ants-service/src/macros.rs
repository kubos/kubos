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
// TODO: Is there a good way to enforce delimiter formatting?
// (ie must be String, str, or char)
macro_rules! process_errors {
	($err:ident) => (process_errors!($err, '\n'));
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

/// Another macro
/// :)
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
/// Optionally:
///   Add the error string to the master error string for later consumption
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
	        
			(errors, success, data)
		}
	}};
}

#[cfg(test)]
mod tests {

    #[test]
    fn test1() {
        assert_eq!(2 + 2, 4);
    }
}
