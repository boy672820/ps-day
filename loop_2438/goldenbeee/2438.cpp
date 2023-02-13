#include <iostream>
using namespace std;
int main(){
    // 동기화 된 두 스트림을 끊어준다.
    ios_base::sync_with_stdio(false);
    // 입출력이 서로 번갈아가면서 사용되지 않았으므로 묶음 해제는 안해줘도 된다.
    //cin.tie(NULL); 

    int N;
    cin >> N;
    for(int i = 1; i <= N; i++)
    {
        for(int j = 1; j <= i; j++ )
        {
             cout << "*";
        }
        cout <<"\n";
    }
    return 0;
}