use octorust::{
    types::{Contributor, MinimalRepository, Order, ReposListOrgSort, ReposListOrgType},
    Client,
};

pub async fn find_org_repositories(client: &Client, org_id: &str) -> Vec<MinimalRepository> {
    let result = client
        .repos()
        .list_all_for_org(
            org_id,
            ReposListOrgType::All,
            ReposListOrgSort::Updated,
            Order::Desc,
        )
        .await;

    match result {
        Ok(reponse) => reponse.body,
        Err(_) => vec![],
    }
}

pub async fn find_org_repo_contributors(
    client: &Client,
    org_id: &str,
    repo_name: String,
) -> Vec<Contributor> {
    let result = client
        .repos()
        .list_all_contributors(org_id, &repo_name, "true")
        .await;

    match result {
        Ok(reponse) => reponse.body,
        Err(_) => vec![],
    }
}
