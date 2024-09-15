# Copyright (c) 2024 Marthinus Scholtz
# SPDX-License-Identifier: Apache-2.0

zephyr_library()
zephyr_include_directories(include)
zephyr_library_sources(
	src/utils.c
	src/sync/mutex.c
	src/thread/thread.c
	src/logger/logger.c
	zephyr/src/time.c
)

