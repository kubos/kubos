// Copyright (C) 2015 ARM Limited. All rights reserved.

#ifndef __CMSIS_CORE_CORE_GENERIC_H__
#define __CMSIS_CORE_CORE_GENERIC_H__

#define __CMSIS_GENERIC
#if defined(TARGET_LIKE_CORTEX_M3)
  #include "cmsis-core/core_cm3.h"
#elif defined(TARGET_LIKE_CORTEX_M4)
  #include "cmsis-core/core_cm4.h"
#elif defined(TARGET_LIKE_CORTEX_M0)
  #include "cmsis-core/core_cm0.h"
#elif defined(TARGET_LIKE_CORTEX_M0P)
  #include "cmsis-core/core_cm0p.h"
#else
  #error "Unknown platform for core_generic.h"
#endif

#endif // #ifndef __CMSIS_CORE_CORE_GENERIC_H__

