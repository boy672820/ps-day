#include <iostream>

using namespace std;

int main()
{
	int H, M;
	cin >> H >> M;
	if (M < 45)
	{
		M = 60 - 45 + M;
		H -= 1;
		if (H < 0)
		{
			H = 24 - 1;
		}
	}
	else
	{
		M = M - 45;
	}
	cout << H << " " << M;
}