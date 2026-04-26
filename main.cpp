#include <iostream>
using namespace std;

#define SQUARE(x) ((x) * (x))

int main() {
    int i = 3;
    int result = SQUARE(i++);

    cout << "Result: " << result << endl;
    cout << "Final value of i: " << i << endl;

    return 0;
}
