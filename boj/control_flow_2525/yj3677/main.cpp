#include <iostream>

using namespace std;

int main()
{
	int A, B, C;
	cin >> A >> B;
	cin >> C;

	B = B + C;

	if (B >= 60)
	{
		while (B >= 60)
		{
			B -= 60;
			A += 1;
			if (A >= 24)
			{
				A = 0;
			}
		}
	}
	cout << A << " " << B;
}
