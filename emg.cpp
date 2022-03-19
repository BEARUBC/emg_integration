#include <stdio.h>

int main(void){
    for(int i = 0; i < 8; i++){
        if(i % 2 == 0)
            printf("%d", 0);
        else 
            printf("%d", 1);
    }

    return 0;
}