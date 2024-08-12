//{"name":"A. Mr. Wow and Lucky Array","group":"Codeforces - TheForces Round #33(Wow-Forces)","url":"https://codeforces.com/gym/105293/problem/A","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n1\n0\n2\n0 1\n4\n0 0 1 0\n5\n0 1 0 0 1\n6\n0 0 0 0 0 0\n","output":"-1\n0 0\n0 1 0 1\n0 0 1 1 0\n0 0 1 1 0 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AMrWowAndLuckyArray"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::{TaskType, TestType};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let mut a = input.read_int_vec(n);

    if n == 1 {
        out.print_line(-1);
    } else if n == 2 {
        a[1] = 1 - a[1];
        out.print_line(a);
    } else {
        let mut b = vec![0; n];
        for i in (n + 1) / 2..n {
            b[i] = 1;
        }
        if a != b {
            out.print_line(b);
        } else {
            b[(n + 1) / 2] = 0;
            b[(n + 1) / 2 - 1] = 1;
            out.print_line(b);
        }
    }
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
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

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
