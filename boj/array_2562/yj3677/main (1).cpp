#include <iostream>
using namespace std;

int main()
{
	ios::sync_with_stdio(false); cin.tie(NULL); cout.tie(NULL);
	int a[9];
	int b, max = 0;
	int num;
	for (int i = 0; i < 9; i++)
	{
		cin >> a[i];
		if (max < a[i])
		{
			max = a[i];
			num = i + 1;
		}
	}
	cout << max << endl;
	cout << num;
}