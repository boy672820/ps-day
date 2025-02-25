use std::cmp;
use std::io;

const MAX: usize = 50_001;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let n = buf.trim().parse::<usize>().unwrap();

    let mut dp: [usize; MAX] = [0; MAX];
    dp[1] = 1; // 초기 값

    for i in 2..n + 1 {
        // 1^2을 n을 충족할 때까지 더할 경우의 값 => 이전 값에 +1
        dp[i] = dp[i - 1] + 1;

        // 하지만, 최소 제곱수의 개수를 구하기 위해서 무차별 대입(브루트 포스)한다.
        // 현재 값보다 작은 수 중에서 제곱 수를 찾음
        for j in 1..i {
            let j_sqr = j * j;

            // 현재 값(i)보다 제곱수가 클 경우 반복 중단
            if j_sqr > i {
                break;
            }

            // 점화식: "dp[i - j_sqr] + 1"
            // 현재 제곱수(j_sqr = j*j)와 현재값에서 제곱수를 뺀 값을 더한다.
            dp[i] = cmp::min(dp[i], dp[i - j_sqr] + 1);
        }
    }

    println!("{}", dp[n])
}
