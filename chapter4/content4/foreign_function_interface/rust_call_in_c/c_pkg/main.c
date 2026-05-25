#include <stdio.h>
#include <stdint.h>
#include <inttypes.h>

extern int add(int a, int b);

int main(void) {
  int sum = add(1, 2);
  printf("%d\n", sum);
}
