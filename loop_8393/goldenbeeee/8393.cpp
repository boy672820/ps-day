#include <iostream>
using namespace std;
int main()
{
    int N;
    int total = 0;
    
    cin >> N;
    
    for(int i = 1; i <= N; i++)
    {
        total += i;
    }
    
    cout << total;
    return 0;
}