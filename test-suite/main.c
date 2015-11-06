
#include <stdio.h>
#include <stdlib.h>

#include <embUnit.h>
#include <embUnit/TextUIRunner.h>
#include <embUnit/TextOutputter.h>
#include <lpm.h>

#include "tests.h"

int main(void)
{
    TextUIRunner_setOutputter(TextOutputter_outputter());
    TextUIRunner_start();
    TextUIRunner_runTest(aprs_suite());
    TextUIRunner_runTest(ax25_suite());
    TextUIRunner_runTest(kiss_suite());
    TextUIRunner_runTest(nmea_suite());
    TextUIRunner_runTest(klog_suite());
    TextUIRunner_end();

    lpm_set(LPM_POWERDOWN);
    return 0;
}
