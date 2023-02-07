#include <iostream>
using namespace std;

int main()
{
	int n = 0;
	int b = 0;
	cin >> n;
	for (int i = 1; i <= n; i++)
	{
		b = i + b;
	}
	cout << b;
	return 0;
}