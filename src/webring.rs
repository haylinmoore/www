use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Member {
    pub id: String,
    pub name: String,
    pub url: String,
}

#[derive(Deserialize, Clone)]
pub struct MemberGetResponse {
    pub prev: Member,
    pub member: Member,
    pub next: Member,
}

pub async fn get_webring_link() -> Option<MemberGetResponse> {
    let res = reqwest::get("https://umaring.mkr.cx/hampton").await;

    if res.is_err() {
        return None;
    }

    let res = res.unwrap();

    // Parse JSON to MemberGetResponse
    let res = res.json::<MemberGetResponse>().await;

    if res.is_err() {
        return None;
    }

    let res = res.unwrap();

    if res.member.id == res.next.id {
        return None;
    }
    if res.member.id == res.prev.id {
        return None;
    }

    Some(res)
}

