#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]

use crate::{run, TASK_TYPE};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use tester::classic::default_checker;
use tester::interactive::std_interactor;
use tester::test_set::GeneratedTestSet;
use tester::Tester;

const PRINT_LIMIT: usize = 1000;

fn interact(mut sol_input: Input, mut sol_output: Output, mut input: Input) -> Result<(), String> {
    Ok(())
}

fn check(mut input: Input, expected: Option<Input>, mut output: Input) -> Result<(), String> {
    Ok(())
}

struct StressTest;

impl GeneratedTestSet for StressTest {
    type TestId = usize;

    fn tests(&self) -> impl Iterator<Item = Self::TestId> {
        1..
    }

    fn input(&self, test: &Self::TestId, out: &mut Output) {
    }

    fn output(&self, test: &Self::TestId, input: &mut Input, out: &mut Output) -> bool {
        false
    }
}

struct MaxTest;

impl GeneratedTestSet for MaxTest {
    type TestId = usize;

    fn tests(&self) -> impl Iterator<Item = Self::TestId> {
        1..=1
    }

    fn input(&self, test: &Self::TestId, out: &mut Output) {
    }

    fn output(&self, test: &Self::TestId, input: &mut Input, out: &mut Output) -> bool {
        false
    }
}

pub(crate) fn run_tests() -> bool {
    let path = "./$TASK";
    let tl = $TIME_LIMIT;
    let tester = match TASK_TYPE {
        crate::TaskType::Interactive => {
            Tester::new_interactive(tl, PRINT_LIMIT, path.to_string(), run, std_interactor)
            // Tester::new_interactive(tl, PRINT_LIMIT, path.to_string(), run, interact)
        }
        crate::TaskType::Classic => {
            Tester::new_classic(tl, PRINT_LIMIT, path.to_string(), run, default_checker)
            // Tester::new_classic(tl, PRINT_LIMIT, path.to_string(), run, check)
        }
    };
    let passed = tester.test_samples();
    // tester.test_generated("Max test", true, MaxTest);
    // tester.test_generated("Stress test", false, StressTest);
    passed
}
