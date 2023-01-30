use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {
    pub admin: Option<String>,
}


#[cw_serde]
pub enum ExecuteMsg {
    CreatePoll {
        poll_id: String,
        question: String,
        options: Vec<String>,
    },
    Vote {
        poll_id: String,
        vote: String,
    },
    DeletePoll {
        poll_id: String,
        options: Vec<String>,
    },
    RevokeVote {
        poll_id: String,
        option: String,
    },
}


#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}
