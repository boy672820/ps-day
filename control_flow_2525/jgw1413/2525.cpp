#include <iostream>
#include <cstdio>
using namespace std;

int main()
{
	int hour, min, needMin;

	cin >> hour >> min >> needMin;

	min += needMin;
	
	if (min >= 60)
	{
		hour += (min / 60);
	}

	if (hour >= 24)
	{
		hour -= 24;
	}

	cout << hour << " " << (min % 60);
}