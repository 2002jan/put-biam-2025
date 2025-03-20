use std::marker::PhantomData;
use qap_utils::problem::QapProblem;
use crate::local_search::search_types::LocalSearchType;
use crate::local_search::starting_solution::StartingSolution;
use crate::TspAlgorithm;

pub struct LocalSearch<
    T: LocalSearchType,
    SS: StartingSolution
> {
    t: PhantomData<T>,
    ss: PhantomData<SS>,
}


impl<
    T: LocalSearchType,
    SS: StartingSolution
> TspAlgorithm for LocalSearch<T, SS> {
    fn run(problem: &QapProblem) -> Vec<usize> {
        let staring_solution = SS::get_string_solution(problem);

        T::run(problem, staring_solution)
    }

    fn name() -> String {
        SS::name() + " " + T::name().as_str() + " " + "Local Search"
    }

    fn snaked_name() -> String {
        SS::snaked_name() + "_" + T::snaked_name().as_str() + "_" + "local_search"
    }
}
