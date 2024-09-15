#pragma once

#include <zephyr/logging/log.h>
#include <zephyr/logging/log_msg.h>
#include <zephyr/logging/log_instance.h>

struct logger_instance {
	LOG_INSTANCE_PTR_DECLARE(log);
	uint32_t cnt;
};

#define NEW_LOG_INSTANCE(_module, _name, _level)    \
	LOG_INSTANCE_REGISTER(_module, _name, _level);  \
	struct logger_instance _name = {		        \
		LOG_INSTANCE_PTR_INIT(log, _module, _name)  \
	}
