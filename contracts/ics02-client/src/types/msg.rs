//! # Messages
//!
//! This module defines the messages that this contract receives.

use cosmwasm_schema::{cw_serde, QueryResponses};

/// The message to instantiate the contract.
#[cw_serde]
pub struct InstantiateMsg {
    /// The address of the cw-ibc-lite-router contract.
    pub router_address: String,
}

/// The execute messages supported by the contract.
#[cw_serde]
pub enum ExecuteMsg {
    /// Create a new client.
    CreateClient {
        /// Code id of the light client contract code.
        code_id: String,
        /// Instantiate message for the light client contract.
        instantiate_msg: cw_ibc_lite_types::clients::InstantiateMsg,
    },
    /// Execute a message on a client.
    ExecuteClient {
        /// The client id of the client to execute the message on.
        client_id: String,
        /// The message to execute on the client.
        message: cw_ibc_lite_types::clients::ExecuteMsg,
    },
}

/// The query messages supported by the contract.
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// Execute a query on a client.
    #[returns(query_responses::QueryClient)]
    QueryClient {
        /// The client id of the client to execute the query on.
        client_id: String,
        /// The query to execute on the client.
        query: cw_ibc_lite_types::clients::QueryMsg,
    },
}

/// Contains the query responses supported by the contract.
pub mod query_responses {
    /// The response to [`super::QueryMsg::QueryClient`].
    #[super::cw_serde]
    pub enum QueryClient {
        /// The response to [`cw_ibc_lite_types::clients::QueryMsg::Status`].
        Status(cw_ibc_lite_types::clients::query_responses::Status),
        /// The response to [`cw_ibc_lite_types::clients::QueryMsg::ExportMetadata`].
        ExportMetadata(cw_ibc_lite_types::clients::query_responses::ExportMetadata),
        /// The response to [`cw_ibc_lite_types::clients::QueryMsg::TimestampAtHeight`].
        TimestampAtHeight(cw_ibc_lite_types::clients::query_responses::TimestampAtHeight),
        /// The response to [`cw_ibc_lite_types::clients::QueryMsg::VerifyClientMessage`].
        VerifyClientMessage(cw_ibc_lite_types::clients::query_responses::VerifyClientMessage),
        /// The response to [`cw_ibc_lite_types::clients::QueryMsg::CheckForMisbehaviour`].
        CheckForMisbehaviour(cw_ibc_lite_types::clients::query_responses::CheckForMisbehaviour),
    }
}
