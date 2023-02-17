#include <iostream>
using namespace std;

int main()
{
	ios::sync_with_stdio(false); cin.tie(NULL); cout.tie(NULL);
	int a, b = 1;
	while (cin >> a >> b)
	{
		if (a == 0 && b == 0)
		{
			break;
		}
		cout << a + b << "\n";
	}
}