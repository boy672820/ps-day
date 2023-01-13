#include <iostream>
using namespace std;
int main(){
    int a;
    string b;
    
    cin >> a;
    cin >> b;
    
    cout << a * (b[2] - '0')<<endl;
    cout << a * (b[1] - '0')<<endl;
    cout << a * (b[0] - '0')<<endl;
    cout << a * stoi(b) << endl;
    return 0;
   
}