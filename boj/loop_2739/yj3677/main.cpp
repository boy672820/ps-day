#include <iostream>

using namespace std;


int main()
{
	int N, M;
	cin >> N;

	for (int i = 1; i < 10; i++)
	{
		M = N * i;
		cout << N << " * " << i << " " << "=" << " " << M << endl;
	}

	return 0;
}