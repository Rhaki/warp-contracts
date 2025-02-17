pub mod condition;
pub mod variable;

use controller::job::{ExternalInput, JobStatus};
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{CosmosMsg, QueryRequest};
#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    ExecuteSimulateQuery(ExecuteSimulateQueryMsg),
    ExecuteValidateJobCreation(ExecuteValidateJobCreationMsg),
    ExecuteHydrateVars(ExecuteHydrateVarsMsg),
    ExecuteResolveCondition(ExecuteResolveConditionMsg),
    ExecuteApplyVarFn(ExecuteApplyVarFnMsg),
    ExecuteHydrateMsgs(ExecuteHydrateMsgsMsg),
}

#[derive(QueryResponses)]
#[cw_serde]
pub enum QueryMsg {
    #[returns(SimulateResponse)]
    SimulateQuery(SimulateQueryMsg),
    #[returns(String)]
    QueryValidateJobCreation(QueryValidateJobCreationMsg),
    #[returns(String)]
    QueryHydrateVars(QueryHydrateVarsMsg),
    #[returns(bool)]
    QueryResolveCondition(QueryResolveConditionMsg),
    #[returns(String)]
    QueryApplyVarFn(QueryApplyVarFnMsg),
    #[returns(Vec<CosmosMsg>)]
    QueryHydrateMsgs(QueryHydrateMsgsMsg),
}

#[cw_serde]
pub struct MigrateMsg {}

#[cw_serde]
pub struct ExecuteSimulateQueryMsg {
    pub query: QueryRequest<String>,
}

#[cw_serde]
pub struct ExecuteHydrateMsgsMsg {
    pub msgs: String,
    pub vars: String,
}

#[cw_serde]
pub struct ExecuteHydrateVarsMsg {
    pub vars: String,
    pub external_inputs: Option<Vec<ExternalInput>>,
}

#[cw_serde]
pub struct ExecuteResolveConditionMsg {
    pub condition: String,
    pub vars: String,
}

#[cw_serde]
pub struct ExecuteApplyVarFnMsg {
    pub vars: String,
    pub status: JobStatus,
}

#[cw_serde]
pub struct ExecuteValidateJobCreationMsg {
    pub condition: Option<String>,
    pub terminate_condition: Option<String>,
    pub vars: String,
    pub msgs: String,
}

#[cw_serde]
pub struct QueryValidateJobCreationMsg {
    pub condition: Option<String>,
    pub terminate_condition: Option<String>,
    pub vars: String,
    pub msgs: String,
}

#[cw_serde]
pub struct QueryHydrateMsgsMsg {
    pub msgs: String,
    pub vars: String,
}

#[cw_serde]
pub struct QueryHydrateVarsMsg {
    pub vars: String,
    pub external_inputs: Option<Vec<ExternalInput>>,
}

#[cw_serde]
pub struct QueryResolveConditionMsg {
    pub condition: String,
    pub vars: String,
}

#[cw_serde]
pub struct QueryApplyVarFnMsg {
    pub vars: String,
    pub status: JobStatus,
}

#[cw_serde]
pub struct SimulateQueryMsg {
    pub query: QueryRequest<String>,
}

#[cw_serde]
pub struct SimulateResponse {
    pub response: String,
}
