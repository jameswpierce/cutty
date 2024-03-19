use std::env;

use chrono::{DateTime, Utc};

use serde::Deserialize;
use serde_json::value::Number;

#[derive(Deserialize, Debug)]
pub struct LinkedFile {
    pub content_type: Option<String>,
    pub created_at: DateTime<Utc>,
    pub description: Option<String>,
    pub entity_type: String,
    pub group_mention_ids: Vec<String>,
    pub id: Number,
    pub member_mention_ids: Vec<String>,
    pub mention_ids: Vec<String>,
    pub name: String,
    pub size: Option<Number>,
    pub story_ids: Vec<Number>,
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
    pub id: Number,
    pub object_id: Number,
    pub subject_id: Number,
    pub updated_at: DateTime<Utc>,
    #[serde(rename = "type")]
    pub story_type: String,
    pub verb: String,
}

#[derive(Deserialize, Debug)]
pub struct StoryStats {
    pub num_related_documents: Number,
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
    pub id: Number,
    pub member_mention_ids: Vec<Number>,
    pub mention_ids: Vec<Number>,
    pub owner_ids: Vec<Number>,
    pub position: Number,
    pub story_id: Number,
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
    pub id: Number,
    pub name: String,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize, Debug)]
pub struct Branch {
    pub created_at: Option<DateTime<Utc>>,
    pub deleted: bool,
    pub entity_type: String,
    pub id: Option<Number>,
    pub merged_branch_ids: Vec<Number>,
    pub name: String,
    pub persistent: bool,
    pub pull_requests: Vec<PullRequest>,
    pub repository_id: Number,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize, Debug)]
pub struct PullRequest {
    pub branch_id: Number,
    pub branch_name: String,
    pub build_status: Option<String>,
    pub closed: bool,
    pub created_at: DateTime<Utc>,
    pub draft: bool,
    pub entity_type: String,
    pub has_overlapping_stories: bool,
    pub id: Number,
    pub merged: bool,
    pub num_added: Number,
    pub num_commits: Option<Number>,
    pub num_modified: Option<Number>,
    pub num_removed: Option<Number>,
    pub number: Number,
    pub overlapping_stories: Vec<Number>,
    pub repository_id: Number,
    pub review_status: String,
    pub target_branch_id: Number,
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
    pub id: Number,
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
    pub estimate_scale: Vec<Number>,
    pub url_slug: String,
}

#[derive(Deserialize, Debug)]
pub struct StorySearchResults {
    pub data: Vec<StorySearchResult>,
    pub next: Option<String>,
    pub total: Number,
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
    pub id: Number,
    pub member_mention_ids: Vec<String>,
    pub mention_ids: Vec<String>,
    pub parent_id: Option<Number>,
    pub position: Number,
    pub story_id: Number,
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
    pub id: Option<Number>,
    pub message: String,
    pub repository_id: Option<Number>,
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
    pub id: Number,
    pub member_mention_ids: Vec<String>,
    pub mention_ids: Vec<String>,
    pub name: String,
    pub size: Number,
    pub story_ids: Vec<Number>,
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
    pub comment_ids: Option<Vec<Number>>,
    pub comments: Vec<StoryComment>,
    pub commits: Vec<Commit>,
    pub completed: bool,
    pub completed_at: Option<DateTime<Utc>>,
    pub completed_at_override: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub custom_fields: Vec<StoryCustomField>,
    pub cycle_time: Option<Number>,
    pub deadline: Option<DateTime<Utc>>,
    pub description: String,
    pub entity_type: String,
    pub epic_id: Option<Number>,
    pub estimate: Option<Number>,
    pub external_id: Option<String>,
    pub external_links: Vec<String>,
    pub file_ids: Option<Vec<Number>>,
    pub files: Vec<UploadedFile>,
    pub follower_ids: Vec<String>,
    pub group_id: Option<String>,
    pub group_mention_ids: Vec<String>,
    pub id: Number,
    pub iteration_id: Option<Number>,
    pub label_ids: Vec<Number>,
    pub labels: Vec<LabelSlim>,
    pub lead_time: Option<Number>,
    pub linked_file_ids: Option<Vec<Number>>,
    pub linked_files: Vec<LinkedFile>,
    pub member_mention_ids: Vec<String>,
    pub mention_ids: Vec<String>,
    pub moved_at: Option<DateTime<Utc>>,
    pub name: String,
    pub num_tasks_completed: Option<Number>,
    pub owner_ids: Vec<String>,
    pub position: Number,
    pub previous_iteration_ids: Vec<Number>,
    pub project_id: Option<Number>,
    pub pull_requests: Vec<PullRequest>,
    pub requested_by_id: String,
    pub started: bool,
    pub started_at: Option<DateTime<Utc>>,
    pub stats: StoryStats,
    pub story_links: Vec<TypedStoryLink>,
    pub story_template_id: Option<String>,
    pub story_type: String,
    pub synced_item: Option<SyncedItem>,
    pub task_ids: Option<Vec<Number>>,
    pub tasks: Vec<Task>,
    pub unresolved_blocker_comments: Option<Vec<Number>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub workflow_id: Number,
    pub workflow_state_id: Number,
}

#[derive(Deserialize, Debug)]
pub struct Workflow {
    pub auto_assign_owner: bool,
    pub created_at: DateTime<Utc>,
    pub default_state_id: Number,
    pub description: String,
    pub entity_type: String,
    pub id: Number,
    pub name: String,
    pub project_ids: Vec<Number>,
    pub states: Vec<WorkflowState>,
    pub team_id: Number,
    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize, Debug)]
pub struct WorkflowState {
    pub color: Option<String>,
    pub created_at: DateTime<Utc>,
    pub description: String,
    pub entity_type: String,
    pub id: Number,
    pub name: String,
    pub num_stories: Number,
    pub num_story_templates: Number,
    pub position: Number,
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
    let result = client.get("https://api.app.shortcut.com/api/v3/workflows")
        .header("Content-Type", "application/json")
        .header("Shortcut-Token", token)
        .send()
        .expect("request failed");

    let json = result.json::<Vec<Workflow>>();
    return json.expect("Somethin else happended");
}
