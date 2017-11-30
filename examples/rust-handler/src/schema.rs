use model::{Subsystem};
use juniper::{Context as JuniperContext};

pub struct Context {
    pub subsystem : Subsystem
}

impl JuniperContext for Context { }

graphql_object!(Subsystem: Context as "Subsystem" |&self| {
    description: "Handler subsystem"

    field power() -> bool as "Power state of subsystem" {
        self.power()
    }
});

pub struct QueryRoot;

graphql_object!(QueryRoot : Context as "Query" |&self| {
    field subsystem(&executor) -> Option<&Subsystem>
        as "Get power status"
    {
        Some(executor.context().subsystem.me())
    }
});
