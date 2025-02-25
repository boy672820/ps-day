#include <iostream>
using namespace std;
int main(){
    int N;
    cin >> N;
    
    for(int i = 1; i <= 9; i++)
    {
        //줄바꿈 출력할땐 endl 잊지말자~
        cout << N << " * "<< i << " = " << N * i << endl;
    }
    
    return 0;
}