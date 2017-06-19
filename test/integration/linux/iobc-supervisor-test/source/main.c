#include <stdio.h>
#include <fcntl.h>
#include <stdlib.h>

#include "kubos-hal-iobc/supervisor.h"

int main(int argc, char **argv) 
{
    supervisor_version_t version = {0};
    if (supervisor_get_version(&version))
    {
        printf("Supervisor Communication Successful!\n");
    }
    else
    {
        printf("Supervisor Communication Failed!\n");
    }

	return 0;
}
