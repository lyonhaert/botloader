use crate::{internal::user::User, util::NotBigU64};
use serde::Serialize;
use ts_rs::TS;

#[derive(Clone, Debug, Serialize, TS)]
#[ts(export)]
#[ts(export_to = "bindings/discord/Member.ts")]
#[serde(rename_all = "camelCase")]
pub struct Member {
    pub deaf: bool,
    pub joined_at: NotBigU64,
    pub mute: bool,
    pub nick: Option<String>,
    pub pending: bool,
    pub premium_since: Option<NotBigU64>,
    pub roles: Vec<String>,
    pub user: User,
}

impl From<twilight_model::guild::Member> for Member {
    fn from(v: twilight_model::guild::Member) -> Self {
        Self {
            deaf: v.deaf,
            joined_at: NotBigU64(v.joined_at.as_micros() as u64 / 1000),
            mute: v.mute,
            nick: v.nick,
            pending: v.pending,
            premium_since: v
                .premium_since
                .map(|v| NotBigU64(v.as_micros() as u64 / 1000)),
            roles: v.roles.iter().map(ToString::to_string).collect(),
            user: v.user.into(),
        }
    }
}

impl Member {
    pub fn from_cache(user: User, member: twilight_cache_inmemory::model::CachedMember) -> Self {
        Self {
            user,
            deaf: member.deaf().unwrap_or_default(),
            joined_at: NotBigU64(member.joined_at().as_micros() as u64 / 1000),
            mute: member.mute().unwrap_or_default(),
            nick: member.nick().map(ToString::to_string),
            premium_since: member
                .premium_since()
                .map(|v| NotBigU64(v.as_micros() as u64 / 1000)),
            roles: member.roles().iter().map(ToString::to_string).collect(),
            pending: false,
        }
    }

    pub fn from_partial(partial: twilight_model::guild::PartialMember) -> Self {
        Self {
            joined_at: NotBigU64(partial.joined_at.as_micros() as u64 / 1000),
            nick: partial.nick,
            premium_since: partial
                .premium_since
                .map(|ts| NotBigU64(ts.as_micros() as u64 / 1000)),
            roles: partial.roles.iter().map(ToString::to_string).collect(),
            user: partial.user.unwrap().into(),
            deaf: partial.deaf,
            mute: partial.mute,
            // From what i can see in the docs, pending is only omitted in events where it's always false, as they
            // can't be pending to trigger that event
            // so this should be sufficient for now.
            pending: false,
        }
    }
}

impl From<twilight_model::gateway::payload::incoming::MemberUpdate> for Member {
    fn from(v: twilight_model::gateway::payload::incoming::MemberUpdate) -> Self {
        Self {
            deaf: v.deaf.unwrap_or_default(),
            joined_at: NotBigU64(v.joined_at.as_micros() as u64 / 1000),
            mute: v.mute.unwrap_or_default(),
            nick: v.nick,
            pending: v.pending,
            premium_since: v
                .premium_since
                .map(|v| NotBigU64(v.as_micros() as u64 / 1000)),
            roles: v.roles.iter().map(ToString::to_string).collect(),
            user: v.user.into(),
        }
    }
}

#[derive(Clone, Debug, Serialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
#[ts(export_to = "bindings/discord/PartialMember.ts")]
pub struct PartialMember {
    pub deaf: bool,
    pub joined_at: NotBigU64,
    pub mute: bool,
    pub nick: Option<String>,
    pub premium_since: Option<NotBigU64>,
    pub roles: Vec<String>,
}

impl From<twilight_model::guild::PartialMember> for PartialMember {
    fn from(v: twilight_model::guild::PartialMember) -> Self {
        Self {
            deaf: v.deaf,
            joined_at: NotBigU64(v.joined_at.as_micros() as u64 / 1000),
            mute: v.mute,
            nick: v.nick,
            premium_since: v
                .premium_since
                .map(|ts| NotBigU64(ts.as_micros() as u64 / 1000)),
            roles: v.roles.iter().map(ToString::to_string).collect(),
        }
    }
}
