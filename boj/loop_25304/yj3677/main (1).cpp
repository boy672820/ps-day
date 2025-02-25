#include <iostream>
using namespace std;

int main()
{
	int totalX, count, a, b, sum = 0;
	cin >> totalX;
	cin >> count;

	for (int i = 0; i < count; i++)
	{
		cin >> a >> b;
		sum = (a * b) + sum;
	}
	if (totalX == sum)
	{
		cout << "Yes";
	}
	else
		cout << "No";
}