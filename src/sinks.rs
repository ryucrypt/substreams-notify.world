use crate::utils;
use crate::pb;

use substreams::errors::Error;
use substreams::log;
use pb::antelope::notify::world::v1::LogMineEvents;
use pb::sf::substreams::sink::files::v1::Lines;

#[substreams::handlers::map]
fn csv_out(params: String, events: LogMineEvents) -> Result<Lines, Error> {
    let mut lines = vec![];

    // query-params
    let filter_landowner = utils::create_filters(params.as_str(), "landowner");
    let filter_land_id = utils::create_filters(params.as_str(), "land_id");

    for event in events.items.into_iter() {
        // filter by params
        if !filter_landowner.is_empty() && !filter_landowner.contains(&event.landowner) { continue; }
        if !filter_land_id.is_empty() && !filter_land_id.contains(&event.land_id.to_string()) { continue; }

        lines.push(
            format!("{},{},{},{},{},{},{},{}",
                event.trx_id,
                event.action_ordinal,
                event.timestamp.unwrap().to_string(),
                event.miner,
                event.bounty,
                event.landowner,
                event.land_id,
                event.landowner_share,
            )
        );
        // log::debug!("logmine: {:?}", lines.last());
    }
    Ok(Lines { lines })
}