#include <stdio.h>
#include <stdlib.h>
#include "kubos-hal-iobc/supervisor.h"

int main(int argc, char ** argv)
{
    supervisor_version_t version = { 0 };
    if (supervisor_get_version(&version))
    {
        if ((version.fields.major_version == 53)
            && (version.fields.minor_version == 53)
            && (version.fields.patch_version == 48))
        {
            printf("Supervisor Communication Successful!\n");
            return 0;
        }
    }

    printf("Supervisor Communication Failed!\n");

    return 1;
}
