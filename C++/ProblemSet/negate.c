#include<stdio.h>
#include<math.h>
int negate(int i){
    if(i<0){
        i=~(i-1);
    }
    else if(i>0){
        i=~i+1;
    }
    return i;
}