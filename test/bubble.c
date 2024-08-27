#include<stdio.h>

void print(int *array, int n) {
	for(int i = 0; i < n; i++) {
		printf("%d ", array[i]);
	}
	puts("");
}

void bubble_sort(int *array, int n) {
	for(int i = 0; i < n - 1; i++) {
		for(int j = 0; j < n - i - 1; j++) {
			if(array[j] > array[j + 1]) {
				int tmp = array[j];
				array[j] = array[j + 1];
				array[j + 1] = tmp;
			}	
		}
	}
}

int main() {
	int array[] = {5, 51, 32, 98, 76, 65, 32, 12};
	int num = sizeof(array) / sizeof(array[0]);
	print(array, num);	
	bubble_sort(array, num);
	print(array, num);
}
