use octorust::{
    types::{Order, RepoSearchResultItem, SearchReposSort},
    Client,
};

pub struct SearchRepositoryQuery {
    min_stars: Option<u32>,
    topics: Vec<String>,
}

impl SearchRepositoryQuery {
    pub fn new(min_stars: Option<u32>, topics: Vec<String>) -> Self {
        Self {
            min_stars,
            topics,
        }
    }

    pub fn get_raw_query(&self) -> String {
        let topics = format!("{} in:topics", self.topics.join(" "));

        let stars = match self.min_stars {
            Some(min_stars) => format!("stars:>={min_stars}"),
            None => String::new(),
        };

        String::from(format!("{topics} {stars}").trim())
    }
}

// typescript nextjs nestjs in:topics stars:>=100
pub async fn trending_repositories(
    client: &Client,
    query: &SearchRepositoryQuery,
) -> Vec<RepoSearchResultItem> {
    let result = client
        .search()
        .repos(
            &query.get_raw_query(),
            SearchReposSort::Updated,
            Order::Desc,
            10,
            1,
        )
        .await;

    vec![]
}
