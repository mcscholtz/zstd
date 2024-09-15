/*
 * SPDX-License-Identifier: Apache-2.0
 */

#include <zephyr/kernel.h>
#include <zephyr/device.h>
#include <zephyr/init.h>
#include <zephyr/logging/log.h>
#include <zephyr/logging/log_msg.h>
#include <zephyr/logging/log_instance.h>
#include "logger.h"

LOG_MODULE_REGISTER(logger, LOG_LEVEL_DBG);


/* Define a catchall instance for any logging that do not specify an instance */
NEW_LOG_INSTANCE(logger, unknown, LOG_LEVEL_INF);

extern void zlog_init_dbg();
extern void zlog_init_inf();
extern void zlog_init_wrn();
extern void zlog_init_err();

int zlog_start()
{
    return 0;
}

SYS_INIT(zlog_start, POST_KERNEL, CONFIG_KERNEL_INIT_PRIORITY_DEFAULT);

void zlog_inst_dbg(struct logger_instance * inst, const char *restrict msg)
{
	LOG_LEVEL_SET(LOG_LEVEL_DBG);
	LOG_INST_DBG(inst->log, "%s", msg);
}

void zlog_inst_inf(struct logger_instance * inst, const char *restrict msg)
{
	LOG_LEVEL_SET(LOG_LEVEL_INF);
	LOG_INST_INF(inst->log, "%s", msg);
}

void zlog_inst_wrn(struct logger_instance * inst, const char *restrict msg)
{
	LOG_LEVEL_SET(LOG_LEVEL_WRN);
	LOG_INST_WRN(inst->log, "%s", msg);
}

void zlog_inst_err(struct logger_instance * inst, const char *restrict msg)
{
	LOG_LEVEL_SET(LOG_LEVEL_ERR);
	LOG_INST_ERR(inst->log, "%s", msg);
}

void zlog_inst_log(struct logger_instance * inst, const char *restrict msg)
{
	LOG_LEVEL_SET(LOG_LEVEL_NONE);
	LOG_INST_ERR(inst->log, "%s", msg);
}