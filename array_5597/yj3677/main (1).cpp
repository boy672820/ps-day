#include <iostream>
using namespace std;

int main()
{
	ios::sync_with_stdio(false); cin.tie(NULL); cout.tie(NULL);
	int a;
	int b[31] = { 0 };

	for (int i = 1; i < 29; i++)
	{
		cin >> a;
		b[a] = 1;
	}
	for (int i = 1; i <= 30; i++)
	{
		if (b[i] == 0)
		{
			cout << i << "\n";
		}
	}
}