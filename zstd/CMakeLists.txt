# Copyright (c) 2024 Marthinus Scholtz
# SPDX-License-Identifier: Apache-2.0

zephyr_library()
zephyr_include_directories(.)
zephyr_library_sources(
	src/utils.c
	src/sync/mutex.c
	src/thread/thread.c
	zephyr/src/time.c
)

