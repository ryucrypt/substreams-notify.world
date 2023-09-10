use crate::abi;
use crate::pb;

use substreams::errors::Error;
use substreams::log;
use substreams_antelope::pb::Block;
use abi::notify_world_abi::actions::Logmine;
use abi::notify_world_abi::ACCOUNT;
use pb::antelope::notify::world::v1::{LogMineEvent, LogMineEvents};

#[substreams::handlers::map]
fn map_logmines(block: Block) -> Result<LogMineEvents, Error> {
    Ok(LogMineEvents {
        items: block.actions::<Logmine>(&[ACCOUNT.expect("specify contract account")])
            .map(|(action, trx)| LogMineEvent {
                // trace information
                trx_id: trx.transaction_id.clone(),
                action_ordinal: trx.action_ordinal,
                timestamp: trx.block_time.clone(),

                // payload
                miner: action.miner,
                bounty: action.bounty,
                landowner: action.landowner,
                land_id: action.land_id,
                landowner_share: action.landowner_share
            })
            .collect(),
    })
}