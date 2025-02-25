#include <iostream>
using namespace std;
int main(){
    int A,B,C;
    int score;
    cin >> A >> B >> C ;
    
    if(A == B && A == C)
    {
       score = 10000 + A * 1000; 
    }
    else if( A == B || A == C )
    {
        score = 1000 + A * 100;
    }
    else if(B == C)
    { 
        score = 1000 + B * 100;
    }
    else
    {
        int max = A > B ? (A > C ? A : C) : (B > C ? B : C);
        score = max * 100;
    }
    
    cout << score;
    
    return 0;
}