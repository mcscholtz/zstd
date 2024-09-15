#include <stdio.h>
#include "../../zstd/include/logger.h"
#include "rustlib.h"

LOG_MODULE_REGISTER(demo, 4);

NEW_LOG_INSTANCE(demo, rust, 4);
NEW_LOG_INSTANCE(demo, c, 4);

int main(void)
{
    LOG_INST_INF(c.log, "entering Rust code.....");

    int sum = rust_test(1, 2);

    LOG_INST_INF(c.log, "rust_add(1, 2) = %d", sum);

	return 0;
}
