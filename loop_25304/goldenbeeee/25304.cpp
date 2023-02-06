#include <iostream>
using namespace std;
int main(){
    int sum;
    int compareSum = 0;
    int N;
    int count, price;

    cin >> sum;
    cin >> N;
    
    for(int i = 0; i < N; i++)
    {
        cin >> count >> price;
        compareSum += count * price;
    }    
    
    if(sum == compareSum)
        cout << "Yes";
    else cout << "No";
    
    return 0;
}