use anyhow::Result;

use codegen_experiment::wire::*;

fn main() -> Result<()> {
    let _ = WireTrade {
        t: 5i64,
        px: 1i64,
        qty: 2i64,
        aggr: false,
    };

    Ok(())
}
