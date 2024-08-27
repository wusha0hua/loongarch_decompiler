#include<stdio.h>
#include<stdlib.h>

int main() {
	int a =rand();
	if(a < 10) {
		puts("a < 10");
		goto label1;
	}
	if(a < 20) {
		puts("a < 20");
		goto label2;
	}
	if(a < 30) {
		puts("a < 30");
		goto label3;
	}
	while(a < 100) {
label1:
		printf("%d\n", a);
		a += 1;
label2:
		printf("%d\n", a);
		a += 2;
label3:
		printf("%d\n", a);
		a += 3;
	}
}
