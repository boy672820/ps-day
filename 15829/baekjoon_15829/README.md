# 50점?

문제를 풀면서 `M` 값이 맘에 걸렸다.

결국 50점을 받고 검색해본 결과 큰 수가 들어올 경우 31 제곱 수 때문에 오버플로우가 발생


| mod M

모듈러 연산을 통해 M 값을 분배하여 계산하면 제곱(pow)으로 인한 오버플로우 문제를 해결할 수 있다.

| (A+B) mod C = (A mod C + B mod C) mod C

| (A - B) mod C = (A mod C - B mod C) mod C

| (A * B) mod C = (A mod C * B mod C) mod C