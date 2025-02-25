#include <iostream>
#include <cstdio>
using namespace std;

int main()
{
	int x, y;

	cin >> x >> y;

	if (x > 0 && y > 0)
	{
		cout << 1;	// 1 »çºÐ¸é
	}
	else if (x < 0 && y > 0)
	{
		cout << 2;	// 2
	}
	else if (x < 0 && y < 0)
	{
		cout << 3;	// 3
	}
	else if (x > 0 && y < 0)
	{
		cout << 4;	// 4
	}
}