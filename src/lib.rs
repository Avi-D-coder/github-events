/// Feed Event API types and docs taken from [github docs](https://developer.github.com/v3/activity/events/types).
///
/// Utilized [json_typegen](http://vestera.as/json_typegen/) in creation.
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
enum Event {
    /// Triggered when a check run is `created`, `rerequested`, `completed`, or has a
    /// `requested_action`. The checks permission allows you to use the checks API. If you plan to
    /// create or modify check runs, your GitHub App will need to have the `checks:write` permission.
    /// If you only plan to consume check runs, your GitHub App only needs the `checks:read`
    /// permission.
    ///
    /// GitHub Apps with the `checks:write` permission will receive the `rerequested` action without
    /// subscribing to the check_run webhook event. The `rerequested` action occurs when someone
    /// requests to re-run your app's check from the pull request UI. See "About status checks" for
    /// more details about the GitHub UI. When you receive a `rerequested` action, you'll need to
    /// create a new check run. Only the GitHub App that someone requests to re-run the check will
    /// receive the `rerequested` payload. Similarly, only the GitHub App someone requests to perform
    /// an action specified by the app will receive the `requested_action` payload.
    ///
    /// GitHub Apps that have the `checks:read` permission and subscribe to the `check_run` webhook
    /// event receive the `created` and `completed` action payloads for all check runs in the app's
    /// repository. Repositories and organizations that subscribe to the `check_run` webhook event
    /// only receive `created` and `completed` event actions.
    CheckRunEvent {
        /// The action performed. Can be `created,` `rerequested,` `completed,` or `requested_action.`
        action: String,
        /// The [`check_run`](https://developer.github.com/v3/checks/runs/).
        check_run: CheckRun,
        ///
        repository: Repository,
        organization: Organization,
        sender: Sender,
        installation: Installation,
    },

    /// Triggered when a check suite is `completed`, `requested`, or `rerequested`. The checks permission
    /// allows you to use the Checks API. If you plan to create or modify check runs, your GitHub
    /// App will need to have the checks:write permission. If you only plan to consume check runs,
    /// your GitHub App only needs the checks:read permission.
    ///
    /// GitHub Apps with the checks:write permission will receive the requested and `rerequested`
    /// action payloads without subscribing to the `check_suite` webhook event. The `requested` action
    /// triggers when new code is pushed to the app's repository. A `rerequested` action occurs when
    /// someone requests to re-run the entire check suite from the pull request UI. See "[About
    /// status checks](https://help.github.com/articles/about-status-checks#checks)" for more
    /// details about the GitHub UI. When you receive the `requested` or
    /// `rerequested` action events, you'll need to
    /// [create a new check run](https://developer.github.com/v3/checks/runs/#create-a-check-run).
    /// Only the GitHub App that
    /// is being asked to run a check will receive the `requested` and `rerequested` payloads.
    ///
    /// GitHub Apps that have the `checks:read` permission and subscribe to the `check_suite` webhook
    /// event receive the completed action payload for all check suites in the app's repository.
    /// Repositories and organizations that subscribe to the `check_suite` webhook event only receive
    /// the `completed` event action.
    CheckSuiteEvent {
        /// The action performed. Can be `created,` `rerequested,` `completed,` or `requested_action.`
        action: String,
        /// The [check_suite](https://developer.github.com/v3/checks/suites/).
        check_suite: CheckSuite,
    },
}

/// FIXME add docs [`check_run`](https://developer.github.com/v3/checks/runs/)
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct CheckRun {
    /// The id of the check suite that this check run is part of.
    id: i64,
    head_sha: String,
    external_id: String,
    url: String,
    html_url: String,
    /// The current status of the check run. Can be `queued,` `in_progress,` or `completed.`
    // FIXME should be enum
    status: String,
    /// The result of the completed `check` run. Can be one of `success,` `failure,` `neutral,` `cancelled,`
    /// timed_out, or `action_required.` This value will be `null` until the check run has `completed.`
    // FIXME should be enum
    conclusion: Option<String>,
    started_at: String,
    completed_at: String,
    output: Output,
    /// The name of the check run.
    name: String,
    check_suite: CheckSuite,
    app: App2,
    pull_requests: Vec<::serde_json::Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Output {
    title: String,
    summary: String,
    text: String,
    annotations_count: i64,
    annotations_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct CheckSuite {
    id: i64,
    /// The head branch name the changes are on.
    head_branch: String,
    /// The SHA of the most recent commit for this check suite.
    head_sha: String,
    /// The summary status for all check runs that are part of the check suite. Can be `requested`, `in_progress`, or `completed`.
    status: String,
    /// The summary conclusion for all check runs that are part of the check suite. Can be one
    /// souccess`, `failure`, `neutral`, `cancelled`, `timed_out`, or `action_required`. This value will be
    /// `null` until the check run has `completed`.
    conclusion: String,
    /// URL that points to the check suite API resource.
    url: String,
    before: String,
    after: String,
    /// An array of pull requests that match this check suite. A pull request matches a check suiif
    /// they have the same `head_sha` and head_branch. When the check suite's `head_branch` is unknown
    /// (`null`) the `pull_requests` array will be empty.
    pull_requests: Vec<::serde_json::Value>,
    app: App,
    created_at: String,
    updated_at: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct App {
    id: i64,
    node_id: String,
    owner: Owner,
    name: String,
    description: ::serde_json::Value,
    external_url: String,
    html_url: String,
    created_at: String,
    updated_at: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Owner {
    login: String,
    id: i64,
    node_id: String,
    avatar_url: String,
    gravatar_id: String,
    url: String,
    html_url: String,
    followers_url: String,
    following_url: String,
    gists_url: String,
    starred_url: String,
    subscriptions_url: String,
    organizations_url: String,
    repos_url: String,
    events_url: String,
    received_events_url: String,
    #[serde(rename = "type")]
    type_field: String,
    site_admin: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct App2 {
    id: i64,
    node_id: String,
    owner: Owner2,
    name: String,
    description: ::serde_json::Value,
    external_url: String,
    html_url: String,
    created_at: String,
    updated_at: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Owner2 {
    login: String,
    id: i64,
    node_id: String,
    avatar_url: String,
    gravatar_id: String,
    url: String,
    html_url: String,
    followers_url: String,
    following_url: String,
    gists_url: String,
    starred_url: String,
    subscriptions_url: String,
    organizations_url: String,
    repos_url: String,
    events_url: String,
    received_events_url: String,
    #[serde(rename = "type")]
    type_field: String,
    site_admin: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Repository {
    id: i64,
    node_id: String,
    name: String,
    full_name: String,
    owner: Owner3,
    private: bool,
    html_url: String,
    description: ::serde_json::Value,
    fork: bool,
    url: String,
    forks_url: String,
    keys_url: String,
    collaborators_url: String,
    teams_url: String,
    hooks_url: String,
    issue_events_url: String,
    events_url: String,
    assignees_url: String,
    branches_url: String,
    tags_url: String,
    blobs_url: String,
    git_tags_url: String,
    git_refs_url: String,
    trees_url: String,
    statuses_url: String,
    languages_url: String,
    stargazers_url: String,
    contributors_url: String,
    subscribers_url: String,
    subscription_url: String,
    commits_url: String,
    git_commits_url: String,
    comments_url: String,
    issue_comment_url: String,
    contents_url: String,
    compare_url: String,
    merges_url: String,
    archive_url: String,
    downloads_url: String,
    issues_url: String,
    pulls_url: String,
    milestones_url: String,
    notifications_url: String,
    labels_url: String,
    releases_url: String,
    deployments_url: String,
    created_at: String,
    updated_at: String,
    pushed_at: String,
    git_url: String,
    ssh_url: String,
    clone_url: String,
    svn_url: String,
    homepage: ::serde_json::Value,
    size: i64,
    stargazers_count: i64,
    watchers_count: i64,
    language: ::serde_json::Value,
    has_issues: bool,
    has_projects: bool,
    has_downloads: bool,
    has_wiki: bool,
    has_pages: bool,
    forks_count: i64,
    mirror_url: ::serde_json::Value,
    archived: bool,
    open_issues_count: i64,
    license: ::serde_json::Value,
    forks: i64,
    open_issues: i64,
    watchers: i64,
    default_branch: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Owner3 {
    login: String,
    id: i64,
    node_id: String,
    avatar_url: String,
    gravatar_id: String,
    url: String,
    html_url: String,
    followers_url: String,
    following_url: String,
    gists_url: String,
    starred_url: String,
    subscriptions_url: String,
    organizations_url: String,
    repos_url: String,
    events_url: String,
    received_events_url: String,
    #[serde(rename = "type")]
    type_field: String,
    site_admin: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Organization {
    login: String,
    id: i64,
    node_id: String,
    url: String,
    repos_url: String,
    events_url: String,
    hooks_url: String,
    issues_url: String,
    members_url: String,
    public_members_url: String,
    avatar_url: String,
    description: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Sender {
    login: String,
    id: i64,
    node_id: String,
    avatar_url: String,
    gravatar_id: String,
    url: String,
    html_url: String,
    followers_url: String,
    following_url: String,
    gists_url: String,
    starred_url: String,
    subscriptions_url: String,
    organizations_url: String,
    repos_url: String,
    events_url: String,
    received_events_url: String,
    #[serde(rename = "type")]
    type_field: String,
    site_admin: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Installation {
    id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct GeneratedType {
    action: String,
    check_suite: CheckSuite,
    repository: Repository,
    organization: Organization,
    sender: Sender,
    installation: Installation,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct HeadCommit {
    id: String,
    tree_id: String,
    message: String,
    timestamp: String,
    author: Author,
    committer: Committer,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Author {
    name: String,
    email: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Committer {
    name: String,
    email: String,
}
