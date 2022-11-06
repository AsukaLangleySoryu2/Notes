#include <stdio.h>
#define TRUE   1;
#define FALSE  0;


size_t
strien(char *string){//一个字符串指针
    int length=0;   
    while(*string++!='\0')//该返回当前字符的值并且指针后移,空字符
        length+=1;
    return length;    
}
int 
find_char(char **strings,char value){//定义一个指向字符串指针的指针
    char *string;
    //当字符串指针遍历并且该字符串不可为NULL
    //这里是遍历多个字符串?
    while((string=*strings++)!=NULL){
            while(*string!='\0'){
                if(*string++=value){//这一步改变了该指针的值，所以一次性
                    return TRUE;
                }
            }
            return FALSE;
    }
}
int main()
{
    //内存与名字之间的关联是硬件给我们提供的
    int a=100;
    int b=-1;
    float c=3.14;
    //简单来说这是初始化指针的值
    int *d=&a;
    float *e=&c;
    //这是更改指针所指向位置的值
    //不能直接对指针赋值，因为地址是未定义的
    *d=99;
    d=&b;
    printf("%d\n",a);
    printf("%d\n",*d);
    // printf("*d:%s\n",*d);
    // printf("d:%s",d);
    // printf("%d\n",a);
    *&a=25;
    //原来是没有空格问题
    printf("%d,%d",a,*d);

    int g=12;
    int *h=&g;//这里指针e指向的地址
    int **f=&h;//这里表示指向指针的指针,所以值为指针的地址
    
    printf("%d,%d,%d",f,*f,**f);
    //对于每一个自增运算符，带上指针就会产生副本
    int x=100;
    int *x1=&x;
    printf("\n%d",x1);
    //简单来说，自增有两步，返回副本同时改变自己，也就是副作用

    //指针的算数运算，会自动根据类型的字节*1
    //指针运算,关系运算,同一个数组情况

}
