#include <iostream>
#include <string>
 
using namespace std;
 
int main() {
 
	string s = "         ,r'\"7\n";   // \", \n 이 제어문자다.
	       s += "r`-_   ,'  ,/\n";    // \n 이 제어문자다.
	       s += " \\. \". L_r'\n";    // \\, \", \n 이 제어문자다.
	       s += "   `~\\/\n";         // \\, \n 이 제어문자다.
	       s += "      |\n";           // \n 이 제어문자다.
	       s += "      |";
	cout << s;
	return 0;
}