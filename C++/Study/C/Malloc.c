#include "stdio.h"
#include "malloc.h"
#include "string.h"
#include "stdlib.h"
int main(){
    
    char *p=NULL;
    p=(char*)malloc(sizeof(char));
    if(!p){
        exit(1);
    }
    
}
typedef struct NODE
{
    struct NODE *next;
    int value;
} Node;