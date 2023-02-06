#include <iostream>
using namespace std;
int main()
{
	int N;
	cin >> N;
	for (int i = 1; i <= N; i++)
	{
		for (int k = 0; k < N - i; k++)
		{
			cout << " ";
		}
		for (int j = 0; j < i; j++)
		{
			cout << "*";
		}
		cout << endl;
	}
	return 0;
}