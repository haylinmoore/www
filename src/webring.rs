use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Member {
    pub id: String,
    pub name: String,
    pub url: String,
}

#[derive(Deserialize, Clone)]
pub struct MemberGetResponse {
    pub prev: Option<Member>,
    pub member: Member,
    pub next: Option<Member>,
}

pub async fn get_webring_link() -> Option<MemberGetResponse> {
    let res = reqwest::get("https://umaring.hamy.cc/hampton")
        .await;

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

    if res.member.id == res.next.as_ref().unwrap().id {
        return None;
    }
    if res.member.id == res.prev.as_ref().unwrap().id {
        return None;
    }
    
    Some(res)
}