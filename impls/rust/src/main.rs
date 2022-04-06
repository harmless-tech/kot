mod logging;

use log::{debug, error, info, trace, warn};

#[cfg(debug_assertions)]
const DEBUG_BUILD: bool = true;
#[cfg(not(debug_assertions))]
const DEBUG_BUILD: bool = false;

/* OLD: This options will probably be changed.
 * kot-bin --compress compile FILES -> Builds all FILES.
 * kot-bin --compress build BUILD_FILE -> Builds using a build file.
 * kot-bin read FILE -> Reads in a kob file and outputs it as text.
 * kot-bin decompress ARCHIVE -> Decompresses a brotli archive.
 *
 * Args:
 * --compress -> Inverts the argument in the build file or enables it. (Compression will compile all files with brotli and then zip them)
 *
 * CURRENT:
 * kot build FILES [--output DIR -> Sets output directory] -> Builds each file and outputs them as FILE_NAME.kob.
 * kot pack FILES OUT_FILE -> Builds all files and outputs them as a single file with OUT_FILE as the name and place. (FILES name will be used to seperate them.)
 * kot read ...
 * Should there be a --compress option? Should hashing also be applied somehow? How to handle logging to a file?
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
