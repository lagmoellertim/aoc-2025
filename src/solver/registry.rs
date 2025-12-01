use lazy_static::lazy_static;
use std::collections::HashMap;

type SolverFn = fn(&str) -> String;

pub struct SolverEntry {
    pub day: usize,
    pub part: usize,
    pub solver_fn: SolverFn,
}

inventory::collect!(SolverEntry);

macro_rules! register_solver {
    {day: $day:expr, part: $part:expr, solver_fn: $solver_fn:expr} => {
        inventory::submit! {
            crate::solver::registry::SolverEntry {
                day: $day,
                part: $part,
                solver_fn: $solver_fn
            }
        }
    };
}

pub(crate) use register_solver;

lazy_static! {
    static ref REGISTERED_SOLVERS: HashMap<usize, HashMap<usize, SolverFn>> = {
        let mut registered_solvers = HashMap::new();

        for solver_entry in inventory::iter::<SolverEntry> {
            registered_solvers
                .entry(solver_entry.day)
                .or_insert_with(HashMap::new)
                .entry(solver_entry.part)
                .insert_entry(solver_entry.solver_fn);
        }

        registered_solvers
    };
}

pub fn get_available_days() -> Vec<usize> {
    REGISTERED_SOLVERS.keys().cloned().collect()
}

pub fn get_available_parts_for_day(day: usize) -> Vec<usize> {
    let Some(values) = REGISTERED_SOLVERS.get(&day) else {
        return vec![];
    };
    

    values.keys().cloned().collect()
}

pub fn get_solver_fn(day: usize, part: usize) -> Option<&'static SolverFn> {
    REGISTERED_SOLVERS.get(&day)?.get(&part)
}
