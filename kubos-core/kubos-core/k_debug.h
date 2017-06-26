/*
 * Copyright (C) 2014 Freie Universität Berlin
 *
 * This file is subject to the terms and conditions of the GNU Lesser
 * General Public License v2.1. See the file LICENSE in the top level
 * directory for more details.
 */

 /**
  * @defgroup KUBOS_CORE_DEBUG Kubos Core Debugging
  * @addtogroup KUBOS_CORE_DEBUG
  * @{
  */

#ifndef DEBUG_H
#define DEBUG_H

#include <stdio.h>
#include <csp/csp_debug.h>

#ifdef __cplusplus
extern "C" {
#endif

/**
 * @def ENABLE_DEBUG
 * @brief   This macro can be defined as 0 or other on a file-based level.
 *          @ref DEBUG() will generate output only if ENABLE_DEBUG is non-zero.
 */


/**
 *
 * @{
 */
#if ENABLE_DEBUG

#define DEBUG_PRINT(...) printf(__VA_ARGS__);

/**
 * @def DEBUG_FUNC
 *
 * @brief   Contains the function name if given compiler supports it.
 *          Otherwise it is an empty string.
 */
# if defined(__cplusplus) && defined(__GNUC__)
#  define DEBUG_FUNC __PRETTY_FUNCTION__
# elif __STDC_VERSION__ >= 199901L
#  define DEBUG_FUNC __func__
# elif __GNUC__ >= 2
#  define DEBUG_FUNC __FUNCTION__
# else
#  define DEBUG_FUNC ""
# endif

/**
 * @def DEBUG
 *
 * @brief Print debug information to stdout
 *
 * @note Another name for ::DEBUG_PRINT
 */
#define DEBUG(...) DEBUG_PRINT(__VA_ARGS__)
#else
#define DEBUG(...)
#endif
/** @} */


#ifdef __cplusplus
}
#endif

#endif /* DEBUG_H */
/** @} */
