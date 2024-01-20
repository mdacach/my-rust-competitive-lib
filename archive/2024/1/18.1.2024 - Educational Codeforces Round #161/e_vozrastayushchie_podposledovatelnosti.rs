//{"name":"E. Возрастающие подпоследовательности","group":"Codeforces - Educational Codeforces Round 161 (Rated for Div. 2)","url":"https://codeforces.com/contest/1922/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n2\n5\n13\n37\n","output":"1\n0\n3\n0 1 0\n5\n2 2 3 4 2\n7\n-1 -1 0 0 2 3 -1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EVozrastayushchiePodposledovatelnosti"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::numbers::num_traits::bit_ops::BitOps;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let x = input.read_u64();

    for i in 1.. {
        if (1 << i) > x {
            let mut ans = Vec::new();
            for j in 1..i {
                ans.push(2 * j);
            }
            for j in (1..i).rev() {
                if x.is_set(j - 1) {
                    ans.push(2 * j - 1);
                }
            }
            out.print_line(ans.len());
            out.print_line(ans);
            return;
        }
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::MultiNumber;
    match test_type {
        TestType::Single => solve(&mut input, &mut output, 1, &pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i + 1, &pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &pre_calc);
                i += 1;
            }
        }
    }
    output.flush();
    input.skip_whitespace();
    input.peek().is_none()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    //    tester::stress_test();
}
//END MAIN
