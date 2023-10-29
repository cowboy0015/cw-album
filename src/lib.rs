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
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cw721::Cw721Query;

    const CREATOR: &str = "creator";

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
}