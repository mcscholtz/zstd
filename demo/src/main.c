#include <stdio.h>
#include "rustlib.h"

int main(void)
{
    printf("calling into rust....%p\n ", rust_test);
    int sum = rust_test(1, 2);

    printf("rust_add(1, 2) = %d\n", sum);

	return 0;
}
