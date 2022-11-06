#include<stdio.h>
#include<stdarg.h>
#define NAME_LENGTH 30
#define ADDR_LENGTH  100
#define PHONE_LENGTH  11

#define MAX_ADDRESSES 1000


int *find_int(int key,int array[],int array_len);
int main(){
}

//函数的缺省认定
//C函数采用传值调用,传的为副本
//就无意义的行为来看，至少两小时
//但是他的利用率却很高
int *find_int(int key, int array[], int array_len)
{
    int i;
    for (int i = 0; i < array_len; i++)
    {
        if (array[i] == key)
        {
            return &array[i];
        }
    }
    return NULL;
}
//递归实现
void 
binary_to_ascii(unsigned int value){
    unsigned int quotient;
    quotient = value/10;
    if(quotient!=0){
        binary_to_ascii(quotient);
    }
    putchar(value %10 +'0');
}
float
average(int n_values,...){
    va_list va_arg;
    int count;
    float sum=0;
    va_start(va_arg,n_values);
    for(count=0;count<n_values;count++){
        sum+=va_arg(va_arg,int);
        va_end(va_arg);
        return sum/n_values;
    }
}