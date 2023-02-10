#include <iostream>
using namespace std;

int main()
{
	ios::sync_with_stdio(false); cin.tie(NULL); cout.tie(NULL);
	int a, b, c, sum, max;
	cin >> a >> b >> c;
	if (a == b && b == c && a == c)
	{
		sum = 10000 + a * 1000;
		cout << sum;
	}
	else if (a == b || b == c || a == c)
	{
		if ( (a == b) || (a ==c))
		{
			sum = 1000 + a * 100;
		}
		else if (b == c)
		{
			sum = 1000 + b * 100;
		}
		cout << sum;
	}
	else if (a != b && b != c && a != c)
	{
		max = a;
		if (max < b)
		{
			max = b;
		}
		if (max < c)
		{
			max = c;
		}
		sum = max * 100;
		cout << sum;
	}
}