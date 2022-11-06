#include <stdio.h>

#define MAX = 100;
#define MIN = 0;
/* int test(int e)
{
    int e = 3;
    fun(e);
};
int fun(int e)
{
    printf("e:%d\n", e);
} */
int main()
{
    //基本数据类型
    char* ch="b";
    char* chr="a";
    //有无符的缺省由编译器决定
    const int haha;
    // printf(sizeof(ch));
    enum Jar_Type{CUP=8,PINT=16,TWO=32,THREE=64};
    enum Jar_Type milk_jug,gas_can;
    int text=CUP;
    int test1=PINT;
    //指针常量
    int *a;//*a的结果为int
    char *message="Hello, world!";//注意字符串存储在字符数组中

    typedef char *ptr_to_char;
    ptr_to_char test2;
    printf(test2);
    //指针常量，两者都指向整数，前者整数不可变，后者地址不可变
    int const *pi;
    int * const pi;
    const int *const expr;
    // test(8);
    
    //链接属性:extern、static属性
    //静态变量--静态内存中。
    //实参在堆栈中传递参数用于递归
    //修改局部static不会改变作用域
    //自动变量的初始化在运行时确定，静态变量可以提前确定,但是前者可以通过表达式动态赋值,不过初始化时还是垃圾
    



}
