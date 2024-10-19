//{"name":"Replace With First","group":"CodeChef - START155A","url":"https://www.codechef.com/START155A/problems/STRFIRSTCHAR","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n4 3\naabc\naab\n8 8\naccepted\naccapted\n1 1\nz\nz\n","output":"1\n2\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ReplaceWithFirst"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let s = input.read_str();
    let t = input.read_str();

    if s[0] != t[0] {
        out.print_line(-1);
        return;
    }
    if s == t {
        out.print_line(0);
        return;
    }
    let mut common_start = 0;
    for i in 0..s.len().min(t.len()) {
        if s[i] == t[i] {
            common_start += 1;
        } else {
            break;
        }
    }
    let mut common_end = 0;
    for i in 0..s.len().min(t.len()) {
        if s[s.len() - 1 - i] == t[t.len() - 1 - i] {
            common_end += 1;
        } else {
            break;
        }
    }
    if common_start + common_end >= n.min(m) {
        out.print_line(1);
    } else {
        out.print_line(2);
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
        TaskType::Classic => input.is_empty(),
        TaskType::Interactive => true,
    }
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN