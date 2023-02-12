#include <iostream>
using namespace std;
int main(){

    // c와 c++의 표준 입출력 동기화 해제
    ios_base::sync_with_stdio(false); 
    // 입출력 묶음 해제
    cin.tie(NULL);  

    int T, A, B;
    cin >> T;
    for(int i = 1 ; i <= T ; i++)
    {
        cin >> A >> B;
        cout << "Case #" << i <<": "<< A<< " + " << B << " = "<< A + B<< "\n";
    }
    
    return 0;
}