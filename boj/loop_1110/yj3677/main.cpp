#include <iostream>
using namespace std;

int main()
{
	ios::sync_with_stdio(false); cin.tie(NULL); cout.tie(NULL);
	int a, b, f = 0;
	int e = 0;
	cin >> f;
	a = f;

	while (true)
	{
		b = (a / 10) + (a % 10);   
		a = ((a % 10) * 10) + (b % 10);  
		++e;
		if (a == f)
		{
			break;
		}
	}
	cout << e;
}