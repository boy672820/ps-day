#include <iostream>
using namespace std;
int main()
{
    int N;
    int total = 0;
    
    cin >> N;
    
    total = N*(N + 1) / 2;
    
    cout << total;
    return 0;
}