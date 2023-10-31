use cosmwasm_schema::cw_serde;
use cosmwasm_std::Empty;
pub use cw721_base::{ContractError, InstantiateMsg, MinterResponse};


#[cw_serde]
#[derive(Default)]
pub struct Metadata {
    /// The name of the artist
    pub artist: String,
    /// The name of the album
    pub album: String,
    /// The url of the album artwork
    pub artwork_url: String,
    /// The album year
    pub year: i32,
    /// The name of the track
    pub track_name: String,
    /// The link to the external audio track reference
    pub audio_track_url: String,
}

pub type Extension = Option<Metadata>;

pub type Cw721MetadataContract<'a> = cw721_base::Cw721Contract<'a, Extension, Empty, Empty, Empty>;
pub type ExecuteMsg = cw721_base::ExecuteMsg<Extension, Empty>;
pub type QueryMsg = cw721_base::QueryMsg<Empty>;

#[cfg(not(feature = "library"))]
pub mod entry {
    use super::*;

    use cosmwasm_std::entry_point;
    use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
    
    #[entry_point]
    pub fn instantiate(
        mut deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: InstantiateMsg,
    ) -> StdResult<Response> {
        Cw721MetadataContract::default().instantiate(deps.branch(), env, info, msg)
    }

    #[entry_point]
    pub fn execute(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg,
    ) -> Result<Response, ContractError> {
        Cw721MetadataContract::default().execute(deps, env, info, msg)
    }

    #[entry_point]
    pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
        Cw721MetadataContract::default().query(deps, env, msg)
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::{CosmosMsg, WasmMsg, Response, to_binary, Uint128};
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cw721::{Cw721Query, Cw721ReceiveMsg};
    use cw_ownable::OwnershipError;

    const CREATOR: &str = "creator";
    #[cw_serde]
    struct AuctionConfig {
        start_time: u64,
        duration: u64,
        coin_denom: String,
        min_bid: Option<Uint128>,
    }

    #[test]
    fn cw721_album_metadata()
    {
        let mut deps = mock_dependencies();
        let contract = Cw721MetadataContract::default();

        let info = mock_info(CREATOR, &[]);
        let init_msg = InstantiateMsg {
            name: "cw721-album".to_string(),
            symbol: "cw_foo".to_string(),
            minter: CREATOR.to_string(),
        };
        contract
            .instantiate(deps.as_mut(), mock_env(), info.clone(), init_msg)
            .unwrap();

        let token_id = "pirate";
        let token_uri = Some("https://pirate.com/evil".into());
        let extension = Some(Metadata {
            artist: "Albert".into(),
            album: "https://pirate.album/best-sellers".to_string(),
            ..Metadata::default()
        });
        let exec_msg = ExecuteMsg::Mint {
            token_id: token_id.to_string(),
            owner: "albert".to_string(),
            token_uri: token_uri.clone(),
            extension: extension.clone(),
        };
        contract
            .execute(deps.as_mut(), mock_env(), info, exec_msg)
            .unwrap();

        let res = contract.nft_info(deps.as_ref(), token_id.into()).unwrap();
        assert_eq!(res.token_uri, token_uri);
        assert_eq!(res.extension, extension);
    }
    #[test]
    fn test_receive_nft_msg() {
        let mut deps = mock_dependencies();
        let contract = Cw721MetadataContract::default();
        let init_msg = InstantiateMsg {
            name: "cw721-album".to_string(),
            symbol: "cw_foo".to_string(),
            minter: CREATOR.to_string(),
        };
        let info = mock_info(CREATOR, &[]);
        contract
            .instantiate(deps.as_mut(), mock_env(), info.clone(), init_msg)
            .unwrap();

        // Mint a token
        let token_id = "pirate".to_string();
        let token_uri = Some("https://pirate.com/evil".into());
        let extension = Some(Metadata {
            artist: "Albert".into(),
            album: "https://pirate.album/best-sellers".to_string(),
            ..Metadata::default()
        });
        let exec_msg = ExecuteMsg::Mint {
            token_id: token_id.clone(),
            owner: "albert".to_string(),
            token_uri: token_uri.clone(),
            extension: extension.clone(),
        };

        let minter = mock_info(CREATOR, &[]);
        contract
            .execute(deps.as_mut(), mock_env(), minter, exec_msg)
            .unwrap();


        let msg = to_binary(
                &AuctionConfig {
                    start_time: 0,
                    duration: 1000000,
                    coin_denom: "usd".to_string(),
                    min_bid: Some(Uint128::from(10000u128)),
                }
            ).unwrap();
        let target = String::from("another_contract");
        let send_msg = ExecuteMsg::SendNft {
            contract: target.clone(),
            token_id: token_id.clone(),
            msg: msg.clone(),
        };

        let random = mock_info("random", &[]);
        let err = contract
            .execute(deps.as_mut(), mock_env(), random, send_msg.clone())
            .unwrap_err();
        assert_eq!(err, ContractError::Ownership(OwnershipError::NotOwner));

        // but owner can
        let random = mock_info("albert", &[]);
        let res = contract
            .execute(deps.as_mut(), mock_env(), random, send_msg)
            .unwrap();

        let payload = Cw721ReceiveMsg {
            sender: String::from("albert"),
            token_id: token_id.clone(),
            msg,
        };
        let expected = payload.into_cosmos_msg(target.clone()).unwrap();
        // ensure expected serializes as we think it should
        match &expected {
            CosmosMsg::Wasm(WasmMsg::Execute { contract_addr, .. }) => {
                assert_eq!(contract_addr, &target)
            }
            m => panic!("Unexpected message type: {m:?}"),
        }
        // and make sure this is the request sent by the contract
        assert_eq!(
            res,
            Response::new()
                .add_message(expected)
                .add_attribute("action", "send_nft")
                .add_attribute("sender", "albert")
                .add_attribute("recipient", "another_contract")
                .add_attribute("token_id", token_id)
        );
    }
}