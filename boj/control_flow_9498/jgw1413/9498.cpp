#include <iostream>
#include <cstdio>
using namespace std;

int main()
{
	int score;

	cin >> score;

	if (100 >= score && score >= 90)
	{
		cout << "A";
	}
	else if (89 >= score && score >= 80)
	{
		cout << "B";
	}
	else if (79 >= score && score >= 70)
	{
		cout << "C";
	}
	else if (69 >= score && score >= 60)
	{
		cout << "D";
	}
	else
	{
		cout << "F";
	}
}