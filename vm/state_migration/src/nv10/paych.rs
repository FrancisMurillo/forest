// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0, MIT

use crate::nv10::util::migrate_amt_raw;
use crate::{ActorMigration, ActorMigrationInput};
use crate::{MigrationError, MigrationOutput, MigrationResult};
use actor::paych::{LaneState, LANE_STATES_AMT_BITWIDTH};
use actor_interface::actorv2::paych::State as V2PayChannelState;
use actor_interface::actorv2::paych::State as V3PayChannelState;
use actor_interface::actorv3;
use async_std::sync::Arc;
use cid::Cid;
use ipld_blockstore::BlockStore;
use cid::Code;

struct PayChannelMigrator(Cid);

impl<BS: BlockStore + Send + Sync> ActorMigration<BS> for PayChannelMigrator {
    fn migrate_state(
        &self,
        store: Arc<BS>,
        input: ActorMigrationInput,
    ) -> MigrationResult<MigrationOutput> {
        let v2_in_state: Option<V2PayChannelState> = store
            .get(&input.head)
            .map_err(|e| MigrationError::BlockStoreRead(e.to_string()))?;

        let v2_in_state = v2_in_state.unwrap();

        let lane_states_out = migrate_amt_raw::<_, LaneState>(
            &*store,
            v2_in_state.lane_states,
            LANE_STATES_AMT_BITWIDTH as i32,
        )
        .map_err(|e| MigrationError::MigrateHAMT(e.to_string()))?;

        let out_state = V3PayChannelState {
            from: v2_in_state.from,
            to: v2_in_state.to,
            to_send: v2_in_state.to_send,
            settling_at: v2_in_state.settling_at,
            min_settle_height: v2_in_state.min_settle_height,
            lane_states: lane_states_out
        };

        let new_head = store.put(&out_state, Code::Blake2b256);

        Ok(MigrationOutput {
            new_code_cid: *actorv3::PAYCH_ACTOR_CODE_ID,
            new_head: new_head.unwrap(),
        })
    }
}
