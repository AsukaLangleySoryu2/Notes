#include <stdio.h>

int increment(int i);
int negate(int i);
int main()
{
    int i = 15;
    printf("increment:%d\n" + increment(i));
    printf("negate:%d\n" + negate(i));
    printf("Hello,Just for Test");
}