// So for SOME REASON wpilib does NOT include the standard library headers
// before including wpi_string.h which requires the standard library headers.
// nice job, guys.
#include <stddef.h>

#include "ntcoreffi/ntcore_c.h"
