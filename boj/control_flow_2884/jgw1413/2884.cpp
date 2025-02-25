#include <iostream>
#include <cstdio>
using namespace std;

int main()
{
	int hour, min;

	cin >> hour >> min;

	if (min - 45 < 0)
	{
		hour -= 1;
		min = 60 - (45 - min);
	}
	if (hour < 0)
	{
		hour = 23;
	}

	cout << hour << min;
}