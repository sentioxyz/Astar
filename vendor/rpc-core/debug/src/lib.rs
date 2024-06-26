// Copyright 2019-2022 PureStake Inc.
// This file is part of Moonbeam.

// Moonbeam is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Moonbeam is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Moonbeam.  If not, see <http://www.gnu.org/licenses/>.

use std::collections::BTreeMap;
use ethereum_types::{H160, H256};
use jsonrpsee::{core::RpcResult, proc_macros::rpc};
use moonbeam_client_evm_tracing::types::single;
use moonbeam_rpc_core_types::RequestBlockId;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Eq, PartialEq, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TraceParams {
    pub disable_storage: Option<bool>,
    pub disable_memory: Option<bool>,
    pub disable_stack: Option<bool>,
    /// Javascript tracer (we just check if it's Blockscout tracer string)
    pub tracer: Option<String>,
    pub timeout: Option<String>,
    pub tracer_config: Option<Value>
}

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageEntry {
    pub key: H256,
    pub value: H256,
}

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageRangeResult {
    pub storage: BTreeMap<H256, StorageEntry>,
    pub next_key: Option<H256>
}

#[rpc(server)]
#[jsonrpsee::core::async_trait]
pub trait Debug {
    #[method(name = "debug_traceTransaction")]
    async fn trace_transaction(
        &self,
        transaction_hash: H256,
        params: Option<TraceParams>,
    ) -> RpcResult<single::TransactionTrace>;
    #[method(name = "debug_traceBlockByNumber", aliases = ["debug_traceBlockByHash"])]
    async fn trace_block(
        &self,
        id: RequestBlockId,
        params: Option<TraceParams>,
    ) -> RpcResult<Vec<single::TransactionTrace>>;
    // #[method(name = "debug_storageRangeAt")]
    // async fn storage_range_at(
    //     &self,
    //     block_hash: H256,
    //     tx_index: u64,
    //     address: H160,
    //     start_key: H256,
    //     limit: u64,
    // ) -> RpcResult<StorageRangeResult>;
}
