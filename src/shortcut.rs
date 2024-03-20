use std::env;

use chrono::{DateTime, Utc};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct EpicSearchResults {
    pub data: Vec<EpicSearchResult>,
    pub next: Option<String>,
    pub total: usize,
}

#[derive(Deserialize, Debug)]
pub struct EpicSearchResult {
    pub app_url: String,
    pub archived: bool,
    pub associated_groups: Option<Vec<EpicAssociatedGroup>>,
    pub comments: Vec<ThreadedComment>,
    pub completed: bool,
    pub completed_at: Option<DateTime<Utc>>,
    pub completed_at_override: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
    pub deadline: Option<DateTime<Utc>>,
    pub description: String,
    pub entity_type: String,
    pub epic_state_id: usize,
    pub external_id: Option<String>,
    pub follower_ids: Option<Vec<String>>,
    pub group_id: Option<String>,
    pub group_ids: Option<Vec<String>>,
    pub id: usize,
    pub label_ids: Option<Vec<usize>>,
    pub labels: Vec<LabelSlim>,
    pub member_mention_ids: Option<Vec<String>>,
    pub mention_ids: Option<Vec<String>>,
    pub milestone_id: Option<usize>,
    pub name: String,
    pub objective_ids: Option<Vec<usize>>,
    pub owner_ids: Option<Vec<String>>,
    pub planned_start_date: Option<DateTime<Utc>>,
    pub position: usize,
    pub productboard_id: Option<String>,
    pub productboard_name: Option<String>,
    pub productboard_plugin_id: Option<String>,
    pub productboard_url: Option<String>,
    pub project_ids: Option<Vec<usize>>,
    pub requested_by_id: String,
    pub started: bool,
    pub started_at: Option<DateTime<Utc>>,
    pub started_at_override: Option<DateTime<Utc>>,
    pub state: String,
    pub stats: EpicStats,
    pub stories_without_projects: usize,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize, Debug)]
pub struct EpicStats {
    pub average_cycle_time: usize,
    pub average_lead_time: usize,
    pub last_story_update: Option<DateTime<Utc>>,
    pub num_points: usize,
    pub num_points_backlog: usize,
    pub num_points_done: usize,
    pub num_points_started: usize,
    pub num_points_unstarted: usize,
    pub num_related_documents: usize,
    pub num_stories_backlog: usize,
    pub num_stories_done: usize,
    pub num_stories_started: usize,
    pub num_stories_total: usize,
    pub num_stories_unestimated: usize,
    pub num_stories_unstarted: usize,
}

#[derive(Deserialize, Debug)]
pub struct EpicAssociatedGroup {
    pub associated_stories_count: usize,
    pub group_id: String,
}

#[derive(Deserialize, Debug)]
pub struct ThreadedComment {
    pub app_url: String,
    pub author_id: String,
    pub comments: Vec<ThreadedComment>,
    pub created_at: DateTime<Utc>,
    pub deleted: bool,
    pub entity_type: String,
    pub external_id: Option<String>,
    pub group_mention_ids: Option<Vec<String>>,
    pub id: usize,
    pub member_mention_ids: Option<Vec<String>>,
    pub mention_ids: Option<Vec<String>>,
    pub text: String,
    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize, Debug)]
pub struct LinkedFile {
    pub content_type: Option<String>,
    pub created_at: DateTime<Utc>,
    pub description: Option<String>,
    pub entity_type: String,
    pub group_mention_ids: Vec<String>,
    pub id: usize,
    pub member_mention_ids: Vec<String>,
    pub mention_ids: Vec<String>,
    pub name: String,
    pub size: Option<usize>,
    pub story_ids: Vec<usize>,
    pub thumbnail_url: Option<String>,
    #[serde(rename = "type")]
    pub linked_file_type: String,
    pub updated_at: DateTime<Utc>,
    pub uploader_id: String,
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct TypedStoryLink {
    pub created_at: DateTime<Utc>,
    pub entity_type: String,
    pub id: usize,
    pub object_id: usize,
    pub subject_id: usize,
    pub updated_at: DateTime<Utc>,
    #[serde(rename = "type")]
    pub story_type: String,
    pub verb: String,
}

#[derive(Deserialize, Debug)]
pub struct StoryStats {
    pub num_related_documents: usize,
}

#[derive(Deserialize, Debug)]
pub struct SyncedItem {
    pub external_id: String,
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct Task {
    pub complete: bool,
    pub completed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub description: String,
    pub entity_type: String,
    pub external_id: Option<String>,
    pub group_mention_ids: Vec<String>,
    pub id: usize,
    pub member_mention_ids: Vec<usize>,
    pub mention_ids: Vec<usize>,
    pub owner_ids: Vec<usize>,
    pub position: usize,
    pub story_id: usize,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize, Debug)]
pub struct LabelSlim {
    pub app_url: String,
    pub archived: bool,
    pub color: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub description: Option<String>,
    pub entity_type: String,
    pub external_id: Option<String>,
    pub id: usize,
    pub name: String,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize, Debug)]
pub struct Branch {
    pub created_at: Option<DateTime<Utc>>,
    pub deleted: bool,
    pub entity_type: String,
    pub id: Option<usize>,
    pub merged_branch_ids: Vec<usize>,
    pub name: String,
    pub persistent: bool,
    pub pull_requests: Vec<PullRequest>,
    pub repository_id: usize,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize, Debug)]
pub struct PullRequest {
    pub branch_id: usize,
    pub branch_name: String,
    pub build_status: Option<String>,
    pub closed: bool,
    pub created_at: DateTime<Utc>,
    pub draft: bool,
    pub entity_type: String,
    pub has_overlapping_stories: bool,
    pub id: usize,
    pub merged: bool,
    pub num_added: usize,
    pub num_commits: Option<usize>,
    pub num_modified: Option<usize>,
    pub num_removed: Option<usize>,
    pub number: usize,
    pub overlapping_stories: Vec<usize>,
    pub repository_id: usize,
    pub review_status: String,
    pub target_branch_id: usize,
    pub target_branch_name: String,
    pub title: String,
    pub updated_at: DateTime<Utc>,
    pub url: String,
    pub vcs_labels: Option<Vec<PullRequestLabel>>,
}

#[derive(Deserialize, Debug)]
pub struct PullRequestLabel {
    pub color: String, // Color
    pub description: Option<String>,
    pub entity_type: String,
    pub id: usize,
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct MemberInfo {
    pub id: String,
    pub mention_name: String,
    pub name: String,
    pub workspace2: BasicWorkspaceInfo,
}

#[derive(Deserialize, Debug)]
pub struct BasicWorkspaceInfo {
    pub estimate_scale: Vec<usize>,
    pub url_slug: String,
}

#[derive(Deserialize, Debug)]
pub struct StorySearchResults {
    pub data: Vec<StorySearchResult>,
    pub next: Option<String>,
    pub total: usize,
}

#[derive(Deserialize, Debug)]
pub struct Identity {
    pub entity_type: String,
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub identity_type: Option<String>,
}
#[derive(Deserialize, Debug)]
pub struct StoryComment {
    pub app_url: String,
    pub author_id: Option<String>,
    pub blocker: Option<bool>, // optional booleans humph
    pub created_at: DateTime<Utc>,
    pub deleted: bool,
    pub entity_type: String,
    pub external_id: Option<String>,
    pub group_mention_ids: Vec<String>,
    pub id: usize,
    pub member_mention_ids: Vec<String>,
    pub mention_ids: Vec<String>,
    pub parent_id: Option<usize>,
    pub position: usize,
    pub story_id: usize,
    pub text: Option<String>,
    pub unblocks_parent: Option<bool>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize, Debug)]
pub struct Commit {
    pub author_email: String,
    pub author_id: Option<String>,
    pub author_identity: Identity,
    pub created_at: DateTime<Utc>,
    pub entity_type: String,
    pub hash: String,
    pub id: Option<usize>,
    pub message: String,
    pub repository_id: Option<usize>,
    pub timestamp: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct StoryCustomField {
    pub field_id: String,
    pub value: String,
    pub value_id: String,
}

#[derive(Deserialize, Debug)]
pub struct UploadedFile {
    pub content_type: String,
    pub created_at: DateTime<Utc>,
    pub description: String,
    pub entity_type: String,
    pub external_id: Option<String>,
    pub filename: String,
    pub group_mention_ids: Vec<String>,
    pub id: usize,
    pub member_mention_ids: Vec<String>,
    pub mention_ids: Vec<String>,
    pub name: String,
    pub size: usize,
    pub story_ids: Vec<usize>,
    pub thumbnail_url: Option<String>,
    pub updated_at: Option<DateTime<Utc>>,
    pub uploader_id: String,
    pub url: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct StorySearchResult {
    pub app_url: String,
    pub archived: bool,
    pub blocked: bool,
    pub blocker: Option<bool>,
    pub branches: Vec<Branch>,
    pub comment_ids: Option<Vec<usize>>,
    pub comments: Vec<StoryComment>,
    pub commits: Vec<Commit>,
    pub completed: bool,
    pub completed_at: Option<DateTime<Utc>>,
    pub completed_at_override: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub custom_fields: Vec<StoryCustomField>,
    pub cycle_time: Option<usize>,
    pub deadline: Option<DateTime<Utc>>,
    pub description: String,
    pub entity_type: String,
    pub epic_id: Option<usize>,
    pub estimate: Option<usize>,
    pub external_id: Option<String>,
    pub external_links: Vec<String>,
    pub file_ids: Option<Vec<usize>>,
    pub files: Vec<UploadedFile>,
    pub follower_ids: Vec<String>,
    pub group_id: Option<String>,
    pub group_mention_ids: Vec<String>,
    pub id: usize,
    pub iteration_id: Option<usize>,
    pub label_ids: Vec<usize>,
    pub labels: Vec<LabelSlim>,
    pub lead_time: Option<usize>,
    pub linked_file_ids: Option<Vec<usize>>,
    pub linked_files: Vec<LinkedFile>,
    pub member_mention_ids: Vec<String>,
    pub mention_ids: Vec<String>,
    pub moved_at: Option<DateTime<Utc>>,
    pub name: String,
    pub num_tasks_completed: Option<usize>,
    pub owner_ids: Vec<String>,
    pub position: usize,
    pub previous_iteration_ids: Vec<usize>,
    pub project_id: Option<usize>,
    pub pull_requests: Vec<PullRequest>,
    pub requested_by_id: String,
    pub started: bool,
    pub started_at: Option<DateTime<Utc>>,
    pub stats: StoryStats,
    pub story_links: Vec<TypedStoryLink>,
    pub story_template_id: Option<String>,
    pub story_type: String,
    pub synced_item: Option<SyncedItem>,
    pub task_ids: Option<Vec<usize>>,
    pub tasks: Vec<Task>,
    pub unresolved_blocker_comments: Option<Vec<usize>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub workflow_id: usize,
    pub workflow_state_id: usize,
}

#[derive(Deserialize, Debug)]
pub struct Workflow {
    pub auto_assign_owner: bool,
    pub created_at: DateTime<Utc>,
    pub default_state_id: usize,
    pub description: String,
    pub entity_type: String,
    pub id: usize,
    pub name: String,
    pub project_ids: Vec<usize>,
    pub states: Vec<WorkflowState>,
    pub team_id: usize,
    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize, Debug)]
pub struct WorkflowState {
    pub color: Option<String>,
    pub created_at: DateTime<Utc>,
    pub description: String,
    pub entity_type: String,
    pub id: usize,
    pub name: String,
    pub num_stories: usize,
    pub num_story_templates: usize,
    pub position: usize,
    #[serde(rename = "type")]
    pub workflow_state_type: String,
    pub updated_at: DateTime<Utc>,
    pub verb: Option<String>,
}

pub fn get_member() -> MemberInfo {
    let token = env::var("SHORTCUT_TOKEN").expect("$SHORTCUT_TOKEN is not set");
    let client = reqwest::blocking::Client::new();
    let result = client
        .get("https://api.app.shortcut.com/api/v3/member")
        .header("Content-Type", "application/json")
        .header("Shortcut-Token", token)
        .send()
        .expect("request failed");

    let json = result.json::<MemberInfo>();
    return json.expect("Somethin else happended");
}

pub fn search_epics() -> EpicSearchResults {
    let token = env::var("SHORTCUT_TOKEN").expect("$SHORTCUT_TOKEN is not set");
    let client = reqwest::blocking::Client::new();
    let result = client.get("https://api.app.shortcut.com/api/v3/search/epics")
        .header("Content-Type", "application/json")
        .header("Shortcut-Token", token)
        .body("{\"detail\": \"full\", \"page_size\": 25, \"query\": \"owner:jamespierce !state:completed !is:archived\"}")
        .send()
        .expect("request failed");

    let json = result.json::<EpicSearchResults>();
    return json.expect("Somethin else happended");
}

pub fn search_stories() -> StorySearchResults {
    let token = env::var("SHORTCUT_TOKEN").expect("$SHORTCUT_TOKEN is not set");
    let client = reqwest::blocking::Client::new();
    let result = client.get("https://api.app.shortcut.com/api/v3/search/stories")
        .header("Content-Type", "application/json")
        .header("Shortcut-Token", token)
        .body("{\"detail\": \"full\", \"page_size\": 25, \"query\": \"owner:jamespierce !state:completed !is:archived\"}")
        .send()
        .expect("request failed");

    let json = result.json::<StorySearchResults>();
    return json.expect("Somethin else happended");
}

pub fn list_workflows() -> Vec<Workflow> {
    let token = env::var("SHORTCUT_TOKEN").expect("$SHORTCUT_TOKEN is not set");
    let client = reqwest::blocking::Client::new();
    let result = client
        .get("https://api.app.shortcut.com/api/v3/workflows")
        .header("Content-Type", "application/json")
        .header("Shortcut-Token", token)
        .send()
        .expect("request failed");

    let json = result.json::<Vec<Workflow>>();
    return json.expect("Somethin else happended");
}
