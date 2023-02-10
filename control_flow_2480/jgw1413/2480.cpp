#include <iostream>
#include <cstdio>
using namespace std;


int Three(int x)
{
	return 10000 + (x * 1000);
}

int Two(int x)
{
	return 1000 + (x * 100);
}

int One(int x)
{
	return 100 * x;
}

int main()
{
	int X[3];
	int sameCount = 0;

	cin >> X[0] >> X[1] >> X[2];

	for (int i = 0; i < 2; i++)
	{
		for (int j = i; j < 3; j++)
		{
			if (X[i] > X[j])
			{
				int temp = X[i];
				X[i] = X[j];
				X[j] = temp;
			}
		}
	}

	for (int i = 0; i < 2; i++) 
	{
		for (int j = i + 1; j < 3; j++) 
		{
			if (X[i] == X[j]) 
			{
				sameCount += 1;
			}
		}
	}

	if (sameCount == 3)
	{
		cout << Three(X[0]);
	}
	else if (sameCount == 1)
	{
		for (int i = 0; i < 2; i++) 
		{
			for (int j = i + 1; j < 3; j++)
			{
				if (X[i] == X[j]) 
				{
					cout << Two(X[i]);
				}
			}
		}
	}
	else if (sameCount == 0)
	{
		cout << 100 * X[2];
	}
}