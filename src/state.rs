use crate::projects::repository::ProjectRepository;

pub struct SharedState {
    pub project_repository: ProjectRepository,
    pub github_client: octorust::Client,
}
