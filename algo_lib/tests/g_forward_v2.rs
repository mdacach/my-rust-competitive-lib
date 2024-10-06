//{"name":"G. Forward! v2","group":"Codeforces - Treaps","url":"https://codeforces.com/gym/539514/problem/G","interactive":false,"timeLimit":2000,"tests":[{"input":"5 2\n4 5\n2 3\n","output":"5 2\n1 1\n4 1\n2 0\n3 0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GForwardV2"}}}

use algo_lib::collections::treap::{Payload, Treap};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::{TaskType, TestType};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();

    struct Node {
        id: usize,
        value: i32,
        delta: i32,
    }

    impl Node {
        fn new(id: usize) -> Self {
            Node {
                id,
                value: 0,
                delta: 0,
            }
        }
    }

    impl Payload for Node {
        fn reset_delta(&mut self) {
            self.delta = 0;
        }

        fn update(&mut self, _left: Option<&Self>, _right: Option<&Self>) {}

        fn push_delta(&mut self, delta: &Self) {
            self.delta += delta.delta;
            self.value += delta.delta;
        }
    }

    let mut treap = Treap::new();
    for i in 1..=n {
        treap.add(Node::new(i));
    }
    for _ in 0..m {
        let l = input.read_size() - 1;
        let r = input.read_size();
        let (left, mid_right) = treap.split_at(l);
        let (mut mid, right) = mid_right.split_at(r - l);
        mid.push(&Node {
            id: 0,
            value: 0,
            delta: 1,
        });
        treap = Treap::merge(mid, Treap::merge(left, right));
    }
    out.print_per_line(&treap.iter().map(|x| (x.id, x.value)).collect::<Vec<_>>());
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = ();

    match TEST_TYPE {
        TestType::Single => solve(&mut input, &mut output, 1, &mut pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i, &mut pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &mut pre_calc);
                i += 1;
            }
        }
    }
    output.flush();
    match TASK_TYPE {
        TaskType::Classic => {
            input.skip_whitespace();
            input.peek().is_none()
        }
        TaskType::Interactive => true,
    }
}

mod tester {
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

pub(crate) fn run_tests() -> bool {
    let path = "./g_forward_v2";
    let time_limit = 2000;
    let tester = match TASK_TYPE {
        crate::TaskType::Interactive => {
            Tester::new_interactive(
                time_limit,
                PRINT_LIMIT,
                path.to_string(),
                run,
                std_interactor,
            )
            //Tester::new_interactive(time_limit, PRINT_LIMIT, path.to_string(), run, interact)
        }
        crate::TaskType::Classic => {
            Tester::new_classic(
                time_limit,
                PRINT_LIMIT,
                path.to_string(),
                run,
                default_checker,
            )
            //Tester::new_classic(time_limit, PRINT_LIMIT, path.to_string(), run, check)
        }
    };
    let passed = tester.test_samples();
    // tester.test_generated("Stress test", false, StressTest);
    passed
}
}
#[test]
fn g_forward_v2() {
    assert!(tester::run_tests());
}