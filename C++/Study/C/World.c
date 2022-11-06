#include <stdio.h>
int main()
{
    int x;
    int y = 0;
    x = y + 3;
    // C不具备布尔类型，而是用整形代替
    // 0值表示false,非零值为真
    if (x > 3)
    {
        printf("Greater\n");
    }
    else
    {
        printf("Not Greater\n");
    }
    // switch的表达式必须为整型值
    switch (1)
    {
    }
    //操作符和表达式，算术移位填充由最高位决定
    int value = 20;
    //操作\符的优先级
    int bit_number = 2;
    value = value | 1 << bit_number;
    //这是将指定的位清零
    value = value & ~(1 << bit_number);
    int a;
    //注意赋值中隐式转换造成的截断
    a = x = y + 3;
    //单目操作符 + - & * ++ -- ! ~ sizeof
    //判断表达式的值不需要对表达式赋值
    sizeof(a = value + 1); //为什么没有赋值
    //关系操作符，满足关系则值为1，否则值为0
    //逻辑操作符 && || 短路求值
    //位操作符 & | 两边都需要求值
    //条件操作符
    //逗号操作符,对每个表达式求值，最后的值为最后一个
    //隐式类型转换
    //操作符的优先级 () 引用访问 单目 间接访问 算数 、位运算 逻辑
    










}