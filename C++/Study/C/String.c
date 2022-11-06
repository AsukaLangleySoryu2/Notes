#include <stdio.h>
#include<string.h>
void main()
{
    //字符串
    char *str1="hahaha";
    char *str2="hehehe";
    if(strlen(str1)>=strlen(str2)){
        printf("%s\t==%s",str1,str2);
    }
    //这里注意该函数返回一个无符号整型，所以相减的结果可能不同
    if(strlen(str1)-strlen(str2)>=0){
        printf("%s\t==%s", str1, str2);
    }
    //字符串的复制,程序员需要保证目标足够容纳被添加的值strcpy
    //字符串比较strcmp,不修改字符串的值
    //这些字符串都必须以NUL结尾

    //strncpy、strncat、strncmp
    //长度受限的函数，此时要指定长度，否则可能以非NULL结尾，必须确保长度的一定
    //长度不受限必然会覆盖数组及后面的值，而长度受限只会覆盖该字符串常量的值

    
    char target[40]="Hello,World!My name is C language.";
    char *origin="Just for test";
    strcpy(target,origin);
    // strcat(origin,target);
    printf("\n%s",target);

    //查找strchr strrchr某个字符
    char string[20]="Hello there,honey";
    char *ans;
    char *nasLast=strrchr(string,'h');
    ans=strchr(string,'h');
    printf("\n%s",ans);
    printf("\n%s",nasLast);

    //查找任意匹配字符字符 strpbrk
    char *ansPlus=strpbrk(string,"aeiou");
    printf("\n%s\t",ansPlus);
    //查找子串

    //高级字符串
    int len1,len2;
    char buffer[]="25,142,330,Smith,J,239-4123";
    len1=strspn(buffer,"0123456789");
    len2=strspn(buffer,",0123456789");
    printf("\n%d",len1);//返回匹配数目，直到出现不包含的字符为止
    printf("\n%d",len2);

    //查找标记




}
//查找任意子串的位置
char * my_strstr(const char *string,char const *str){
        register char*last;
        register char*current;
        last=NULL;//设置为上一次的位置

        
        if(*str!='\0'){
            //查找在string中第一次出现的位置，并且将该指针赋值给current值
            current=strstr(current,str);
            //当存在该值时
            while(current!=NULL){
                //更新当前指针
                last=current;
                //current指向下一个子串位置
                current=strstr(last+1,str);
            }
        }
        return last;
}

