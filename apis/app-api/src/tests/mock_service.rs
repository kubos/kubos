use juniper::{FieldError, FieldResult, Value};
use kubos_service;

pub struct Subsystem;
type Context = kubos_service::Context<Subsystem>;

pub struct QueryRoot;

/// Base GraphQL query model
graphql_object!(QueryRoot: Context as "Query" |&self| {
    field ping(fail = false: bool) -> FieldResult<String>
    {
        match fail {
            true => Err(FieldError::new("Query failed", Value::null())),
            false => Ok(String::from("query"))
        }
    }
});

pub struct MutationRoot;

/// Base GraphQL mutation model
graphql_object!(MutationRoot: Context as "Mutation" |&self| {
    field ping() -> FieldResult<String>
        {
            Ok(String::from("mutation"))
        }
});
