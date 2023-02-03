#include <iostream>
using namespace std;
int main(){
    int A, B, C;
    int hour , min;
    
    cin >> A >> B;
    cin >> C;
    
    // 조리시간을 합해서 모두 분으로 바꿈
    min = A * 60 + B;
    min += C;
    
    // 24시일 경우 hour가 0부터 시작하도록 나머지 연산
    hour = (min / 60) % 24;
    min = min % 60; // 분
    
    
    cout << hour << " " << min;
    return 0;
}