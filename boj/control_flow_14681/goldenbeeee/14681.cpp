#include <iostream>
using namespace std;
int main(){
    int x, y;
    int N;

    cin >> x >> y;
    
    if(x > 0)
    {
        N = y > 0 ? 1 : 4;
        cout << N;
    }
    else{
        N = y > 0 ? 2 : 3;
        cout << N;
    }
    
    return 0;
}