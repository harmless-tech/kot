mod logging;

use log::{debug, error, info, trace, warn};

#[cfg(debug_assertions)]
const DEBUG_BUILD: bool = true;
#[cfg(not(debug_assertions))]
const DEBUG_BUILD: bool = false;

/*
 * kot-bin --compress compile FILES -> Builds all FILES.
 * kot-bin --compress build BUILD_FILE -> Builds using a build file.
 * kot-bin read FILE -> Reads in a kob file and outputs it as text.
 * kot-bin decompress ARCHIVE -> Decompresses a brotli archive.
 *
 * Args:
 * --compress -> Inverts the argument in the build file or enables it. (Compression will compile all files with brotli and then zip them)
 */
fn main() {
    // Logging
    let _logging = logging::setup_log();
    info!("Logging Level Check");
    info!("TRUE");
    warn!("TRUE");
    error!("TRUE");
    debug!("TRUE");
    trace!("TRUE");
    info!("Logging Level Check - Done!");
}
