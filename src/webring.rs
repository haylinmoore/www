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
    // Make the request
    let res = ureq::get("https://umaring.mkr.cx/hampton").call();

    if let Err(_) = res {
        return None;
    }

    // Parse JSON response
    let response_body = res.unwrap().into_json();

    if let Err(_) = response_body {
        return None;
    }

    let res: MemberGetResponse = response_body.unwrap();

    // Check if member ID is the same as prev or next, return None if it is
    if res.member.id == res.next.id || res.member.id == res.prev.id {
        return None;
    }

    Some(res)
}
