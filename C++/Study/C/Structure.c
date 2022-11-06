#include <stdio.h>
void main()
{
    //变量day与a[10]、b的类型不一样
    //因为没有标识类型，所以结构不一样
    struct 
    {
        int a;
        char b;
        float c;
    } day;
    struct {
        int a;
        int b;
    } a[10],b;
    struct Test{
        int haha;
        int hehe;
        int gege;
    };
    typedef struct
    {
        int a;
        int b;
        int c;
    } Simple;
    //方法一：标签声明一种结构
    typedef struct{
        int a;
        char b;
        float c;
        struct Test aq;
        Simple test;
    } SIMPLE;
    SIMPLE test1={1,'s',3.14,{},{}};
    int *testInt;
    testInt=(int*)&test1;//这里将结构指针转化为指向int类型的指针
    SIMPLE test2;
    test2=test1;
    //方法二：typedof声明类型
    test1.test.a=1;
    // int *testAndTest=test1;//错误的
    struct Test* test3=&(test1.aq);
    (*test3).haha;
    test3->gege;

    //结构的自引用
    typedef struct Node{
        int data;
        struct Node *left;//因为知道指针的大小
        struct Node *right;
    }Tree;    
    //结构的初始化
    struct INT_EX{
        int a;
        short b[10];
        Simple c;
    }xxxxx={
        10,
        {1,2,3,6,7}, 
        {}
    };
    //结构名称，但是结构名不直接代表指针
    struct INT_EX* testPlsu;
    testPlsu=&xxxxx;
    struct INT_EX* zi=&xxxxx;
    int *pi=&zi->a;//->应用于指向结构的指针

    printf("\n%d",*pi);
    //结构的存储分配,结构严格分配即使存在浪费
    //作为函数参数的结构,指针效率更高，然而结构因为要复制副本，所以效率更低
    //问题:结构传递的不是指针的副本吗?
    struct CHAR{
        unsigned ch: 7;
        unsigned font: 6;
        unsigned size:18;
    };
    struct CHAR  ch1;
    //联合
    union
    {
        float f;
        int i;
    } fi;
    fi.f=3.1415926;
    printf("\n%d",fi.f);





}