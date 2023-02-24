#include <iostream>
using namespace std;

int main()
{
	ios::sync_with_stdio(false); cin.tie(NULL); cout.tie(NULL);
	int n, x = 1, v = 0, count = 0;
	cin >> n;
	int* a = new int[n];

	for (int i = 0; i < n; i++)
	{
		cin >> a[i];
	}
	cin >> v;
	for (int i = 0; i < n; i++)
	{
		if (a[i] == v)
		{
			++count;
		}
	}
	cout << count;
}