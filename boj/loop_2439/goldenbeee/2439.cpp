#include <iostream>
using namespace std;
int main(){

	ios_base::sync_with_stdio(false);
    
    int N;
    cin >> N;
    
    for(int i = 0; i < N; i++)
    {
        for(int j = 0; j < N; j++)
        {
            if(j < N-i-1)
            {
                cout << " ";
            }
            else{
                cout << "*";
            }
        }
        cout << "\n";
    }
    return 0;
    
}