#include <iostream>
using namespace std;
int main(){
    int count;
    int A, B;
    cin >> count;
    
    for(int i =0; i < count; i++)
    {
        // cin은 endl을 쓸수없나봄
        cin >> A >> B;
        cout << A + B << endl;
    }
    
    return 0;
}