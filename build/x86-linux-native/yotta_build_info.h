#ifndef __YOTTA_BUILD_INFO_H__
#define __YOTTA_BUILD_INFO_H__
// yotta build info, #include YOTTA_BUILD_INFO_HEADER to access
#define YOTTA_BUILD_YEAR 2016 // UTC year
#define YOTTA_BUILD_MONTH 3 // UTC month 1-12
#define YOTTA_BUILD_DAY 31 // UTC day 1-31
#define YOTTA_BUILD_HOUR 15 // UTC hour 0-24
#define YOTTA_BUILD_MINUTE 53 // UTC minute 0-59
#define YOTTA_BUILD_SECOND 28 // UTC second 0-61
#define YOTTA_BUILD_UUID 5a954fa3-1c6e-48ee-9911-c2af1ebe0ff7 // unique random UUID for each build
#define YOTTA_BUILD_VCS_ID ffd0a959debd797f45f949cfbbee24d31b92c849 // git or mercurial hash
#define YOTTA_BUILD_VCS_CLEAN 0 // evaluates true if the version control system was clean, otherwise false
#define YOTTA_BUILD_VCS_DESCRIPTION ffd0a95 // git describe or mercurial equivalent
#endif // ndef __YOTTA_BUILD_INFO_H__
