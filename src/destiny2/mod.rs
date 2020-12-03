use anyhow::Error;
use async_trait::async_trait;
use crate::BungieClient;

use self::models::*;
use std::fmt::Write;

pub mod models;

fn push_components(
    out: &mut String,
    mut components: Vec<DestinyComponentType>,
    default_component: DestinyComponentType
) {
    if components.is_empty() {
        components.push(default_component);
    }
    for item in &components {
        write!(out, "{},", *item as i64).unwrap();
    }
    out.pop();
}

#[async_trait]
pub trait Destiny2 {
    async fn get_destiny_manifest(&self) -> Result<Response<DestinyManifest>, Error>;
    async fn search_destiny_player(&self, membership_type: MembershipType, name: &str) -> Result<Response<DestinyManifest>, Error>;
    async fn equip_item(&self, destiny_item_action_request: DestinyItemActionRequest) -> Result<Response<DestinyManifest>, Error>;
    async fn get_character(&self, membership_type: MembershipType, destiny_membership_id: &str, character_id: i64, components: Vec<DestinyComponentType>) -> Result<Response<DestinyCharacterResponse>, Error>;
    async fn get_item(&self, membership_type: MembershipType, destiny_membership_id: &str, item_instance_id: i64, components: Vec<DestinyComponentType>) -> Result<Response<DestinyItemResponse>, Error>;
    async fn get_profile(&self, membership_type: MembershipType, destiny_membership_id: &str, components: Vec<DestinyComponentType>) -> Result<Response<DestinyProfileResponse>, Error>;
}

#[async_trait]
impl Destiny2 for BungieClient {
    async fn get_destiny_manifest(&self) -> Result<Response<DestinyManifest>, Error> {
        self.send_request("/Destiny2/Manifest", None).await
    }

    async fn search_destiny_player(&self, membership_type: MembershipType, name: &str) -> Result<Response<DestinyManifest>, Error> {
        let path = &format!("/Destiny2/SearchDestinyPlayer/{}/{}", membership_type as i64, name);
        self.send_request(path, None).await
    }

    async fn equip_item(&self, destiny_item_action_request: DestinyItemActionRequest) -> Result<Response<DestinyManifest>, Error> {
        self.send_request("/Destiny2/Actions/Items/EquipItem", Some(serde_json::to_string(&destiny_item_action_request)?)).await
    }

    async fn get_character(&self, membership_type: MembershipType, destiny_membership_id: &str, character_id: i64, components: Vec<DestinyComponentType>) -> Result<Response<DestinyCharacterResponse>, Error> {
        let mut path = format!("/Destiny2/{}/Profile/{}/Character/{}?components=", membership_type as i64, destiny_membership_id, character_id);
        push_components(&mut path, components, DestinyComponentType::Profiles);
        self.send_request(&path, None).await
    }

    async fn get_item(&self, membership_type: MembershipType, destiny_membership_id: &str, item_instance_id: i64, components: Vec<DestinyComponentType>) -> Result<Response<DestinyItemResponse>, Error> {
        let mut path = format!("/Destiny2/{}/Profile/{}/Item/{}?components=", membership_type as i64, destiny_membership_id, item_instance_id);
        push_components(&mut path, components, DestinyComponentType::None);
        self.send_request(&path, None).await
    }

    async fn get_profile(&self, membership_type: MembershipType, destiny_membership_id: &str, components: Vec<DestinyComponentType>) -> Result<Response<DestinyProfileResponse>, Error> {
        let mut path = format!("/Destiny2/{}/Profile/{}?components=", membership_type as i64, destiny_membership_id);
        push_components(&mut path, components, DestinyComponentType::Profiles);
        self.send_request(&path, None).await
    }
}