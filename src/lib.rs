/// Feed Event API types and docs taken from [github docs](https://developer.github.com/v3/activity/events/types).
///
/// Utilized [json_typegen](http://vestera.as/json_typegen/) in creation.
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod actions;

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
    /// receive the `rerequested` payload. Similarly,
    /// only the GitHub App someone requests to perform
    /// an action specified by the app will receive the `requested_action` payload.
    ///
    /// GitHub Apps that have the `checks:read` permission and subscribe to the `check_run` webhook
    /// event receive the `created` and `completed` action payloads for all check runs in the app's
    /// repository. Repositories and organizations that subscribe to the `check_run` webhook event
    /// only receive `created` and `completed` event actions.
    CheckRunEvent {
        /// The action performed.
        /// Can be `Created,` `Rerequested,` `Completed,` or `RequestedAction`.
        action: actions::Check,
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
    /// action payloads without subscribing to the `check_suite` webhook event.
    /// The `requested` action
    /// triggers when new code is pushed to the app's repository. A `rerequested` action occurs when
    /// someone requests to re-run the entire check suite from the pull request UI. See "[About
    /// status checks](https://help.github.com/articles/about-status-checks#checks)" for more
    /// details about the GitHub UI. When you receive the `requested` or
    /// `rerequested` action events, you'll need to
    /// [create a new check run](https://developer.github.com/v3/checks/runs/#create-a-check-run).
    /// Only the GitHub App that
    /// is being asked to run a check will receive the `requested` and `rerequested` payloads.
    ///
    /// GitHub Apps that have the `checks:read` permission and
    /// subscribe to the `check_suite` webhook
    /// event receive the completed action payload for all check suites in the app's repository.
    /// Repositories and organizations that subscribe to the `check_suite`
    /// webhook event only receive
    /// the `completed` event action.
    CheckSuiteEvent {
        /// The action performed.
        /// Can be `Created,` `Rerequested,` `Completed,` or `RequestedAction.`
        action: actions::Check,
        /// The [check_suite](https://developer.github.com/v3/checks/suites/).
        check_suite: CheckSuite,
    },

    /// Triggered when a
    /// [commit comment](https://developer.github.com/v3/repos/comments/#list-commit-comments-for-a-repository) is created.
    CommitCommentEvent {
        action: actions::Created,
        /// The [comment](https://developer.github.com/v3/repos/comments/#list-commit-comments-for-a-repository) itself.
        // FIXME
        comment: Comment,
        repository: Repository,
        sender: Sender,
    },

    /// Represents a created repository, branch, or tag.
    /// Note: webhooks will not receive this event for created repositories.
    /// Additionally, webhooks will not receive this event for tags
    /// if more than three tags are pushed at once.
    CreateEvent {
        /// The git ref (or `null` if only a repository was created).
        #[serde(rename = "ref")]
        ref_field: String,
        /// The object that was created. Can be one of "repository", "branch", or "tag"
        ref_type: String,
        /// The name of the repository's default branch (usually `master`).
        master_branch: String,
        /// The repository's current description.
        description: ::serde_json::Value,
        pusher_type: String,
        repository: Repository,
        sender: Sender,
    },

    /// Represents a [deleted branch or tag](https://developer.github.com/v3/git/refs/#delete-a-reference).
    /// Note: webhooks will not receive this event for tags
    /// if more than three tags are deleted at once.
    DeleteEvent {
        /// The full git ref.
        #[serde(rename = "ref")]
        ref_field: String,
        /// The object that was deleted. Can be "branch" or "tag".
        ref_type: String,
        pusher_type: String,
    },

    /// Represents a [deployment](https://developer.github.com/v3/repos/deployments/#list-deployments).
    ///
    /// Events of this type are not visible in timelines. These events are only used to trigger hooks.
    DeploymentEvent {
        /// The [deployment](https://developer.github.com/v3/repos/deployments/#list-deployments).
        deployment: Deployment,
        /// The [repository](https://developer.github.com/v3/repos/) for this deployment.
        repository: Repository,
        sender: Sender,
    },

    /// Represents a [deployment status](https://developer.github.com/v3/repos/deployments/#list-deployment-statuses).
    ///
    /// Events of this type are not visible in timelines.
    /// These events are only used to trigger hooks.
    DeploymentStatusEvent {
        /// The [deployment status](https://developer.github.com/v3/repos/deployments/#list-deployment-statuses).
        deployment_status: DeploymentStatus,
        /// The [deployment](https://developer.github.com/v3/repos/deployments/#list-deployments)
        /// that this status is associated with.
        deployment: Deployment,
        /// The [repository](https://developer.github.com/v3/repos/) for this deployment.
        repository: Repository,
        sender: Sender,
    },

    /// Triggered when a user [forks a
    /// repository](https://developer.github.com/v3/repos/forks/#create-a-fork).
    ForkEvent {
        /// The created [repository](https://developer.github.com/v3/repos/).
        forkee: Forkee,
        repository: Repository,
        sender: Sender,
    },

    /// Triggered when someone revokes their authorization of a GitHub App. A GitHub App receives
    /// this webhook by default and cannot unsubscribe from this event.
    /// This event is not available in the [Events API](https://developer.github.com/v3/activity/events/).
    ///
    /// Anyone can revoke their authorization of a GitHub App from their
    /// [GitHub account settings page](https://github.com/settings/apps/authorizations).
    /// Revoking the authorization of a GitHub App does not uninstall the GitHub App.
    /// You should program your GitHub App so that when it receives this webhook,
    /// it stops calling the API on behalf of the person who revoked the token.
    /// If your GitHub App continues to use a revoked access token,
    /// it will receive the `401 Bad Credentials` error.
    /// For details about user-to-server requests, which require GitHub App authorization,
    /// see ["Identifying and authorizing users for GitHub Apps.](https://developer.github.com/apps/building-github-apps/identifying-and-authorizing-users-for-github-apps/)"
    GitHubAppAuthorizationEvent {
        action: actions::Revoked,
        sender: Sender,
    },

    /// Triggered when a Wiki page is created or updated.
    GollumEvent {
        pages: Vec<Page>,
        repository: Repository,
        sender: Sender,
    },

    /// Triggered when a GitHub App has been installed or uninstalled.
    InstallationEvent {
        /// The action that was performed. Can be either `Created` or `Deleted`.
        action: actions::CreatedDeleted,
        /// The installation itself.
        installation: Installation,
        repositories: Vec<PartialRepository>,
        sender: Sender,
    },

    InstallationRepositoriesEvent {
        /// The action that was performed. Can be either `Added` or `Removed`.
        action: actions::AddedRemoved,
        /// The installation itself.
        installation: Installation,
        /// The choice of repositories the installation is on. Can be either "selected" or "all".
        repository_selection: String,
        /// An array of repository objects, which were added to the installation.
        repositories_added: Vec<PartialRepository>,
        /// An array of repository objects, which were removed from the installation.
        repositories_removed: Vec<RepositoriesRemoved>,
        sender: Sender,
    },

    IssueCommentEvent {
        /// The action that was performed on the comment.
        /// Can be one of `Created`, `Edited`, or `Deleted`.
        action: actions::CrEdDel,
        /// The changes to the comment if the action was "edited".
        /// `changes[body][from]: String` The changes to the comment if the action was "edited".
        // FIXME it's unclear what the structure of changes is.
        changes: Option<::serde_json::Value>,
        /// The [issue](https://developer.github.com/v3/issues/) the comment belongs to.
        issue: Issue,
        /// The [comment](https://developer.github.com/v3/issues/comments/) itself.
        comment: Comment,
        repository: Repository,
        sender: Sender,
    },

    IssueEvent(IssueEvent),

    LabelEvent {
        /// The action that was performed on the comment.
        /// Can be one of `Created`, `Edited`, or `Deleted`.
        action: actions::CrEdDel,
        /// The label that was added.
        label: Label,
        /// The changes to the label if the action was "edited".
        /// `changes[name][from]: String` The previous version of the name if the action was "edited".
        /// `changes[color][from]: String` The previous version of the color if the action was "edited".
        changes: Option<serde_json::Value>,
        repository: Repository,
        sender: Sender,
    },

    /// Triggered when a user accepts an invitation or is removed as a collaborator to a repository,
    /// or has their permissions changed.
    MemberEvent {
        /// The action that was performed. Can be one of `added`, `deleted`, or `edited`.
        action: String,
        /// The user that was added.
        member: Member,
        /// The changes to the collaborator permissions if the action was `edited`.
        changes: MemberEventChanges,
        repository: Repository,
        sender: Sender,
    },

    /// Triggered when a user is added or removed from a team.
    ///
    /// Events of this type are not visible in timelines.
    /// These events are only used to trigger hooks.
    MembershipEvent {
        /// The action that was performed. Can be "added" or "removed".
        action: String,
        /// The scope of the membership. Currently, can only be "team".
        scope: String,
        /// The [user](https://developer.github.com/v3/users/) that was added or removed.
        member: Member,
        sender: Sender,
        /// The [team](https://developer.github.com/v3/teams/) for the membership.
        team: Team,
        organization: Organization,
    },

    /// Triggered when a milestone is created, closed, opened, edited, or deleted.
    ///
    /// Events of this type are not visible in timelines.
    /// These events are only used to trigger hooks.
    MilestoneEvent {
        /// The action that was performed.
        /// Can be one of `created`, `closed`, `opened`, `edited`, or `deleted`.
        action: String,
        /// The milestone itself.
        milestone: Milestone,
        /// The changes to the milestone if the action was edited.
        /// changes[description][from]: String` The previous version of the description if the action was `edited`.
        /// `changes[due_on][from]: String` The previous version of the due date if the action was `edited`.
        /// `changes[title][from]: String` The previous version of the title if the action was `edited`.
        changes: Option<::serde_json::Value>,
        repository: Repository,
        sender: Sender,
    },

    /// Triggered when a user is added, removed, or invited to an Organization.
    /// Events of this type are not visible in timelines.
    /// These events are only used to trigger organization hooks.
    OrganizationEvent {
        /// The action that was performed.
        /// Can be one of: `member_added`, `member_removed`, or `member_invited`.
        action: String,
        /// The invitation for the user or email if the action is member_invited.
        // FIXME What is the structure of an invitation.
        invitation: Option<::serde_json::Value>,
        /// The membership between the user and the organization.
        /// Not present when the action is `member_invited`.
        membership: Membership,
        /// The organization in question.
        organization: Organization,
        sender: Sender,
    },

    /// Triggered when an organization blocks or unblocks a user.
    OrgBlockEvent {
        /// The action performed. Can be `blocked` or `unblocked`.
        action: String,
        /// Information about the user that was blocked or unblocked.
        blocked_user: User,
        /// Information about the organization that blocked or unblocked the user.
        organization: Organization,
        /// Information about the user who sent the blocking/unblocking request on behalf of the organization.
        sender: Sender,
    },

    /// Represents an attempted build of a GitHub Pages site, whether successful or not.
    ///
    /// Triggered on push to a GitHub Pages enabled branch
    /// (`gh-pages` for project pages, `master` for user and organization pages).
    PageBuildEvent {
        id: i64,
        /// The page [build](https://developer.github.com/v3/repos/pages/#list-pages-builds) itself.
        build: Build,
        repository: Repository,
        sender: Sender,
    },

    /// Triggered when a [project card](https://developer.github.com/v3/projects/cards) is created, updated, moved, converted to an issue, or deleted.
    ProjectCardEvent {
        /// The action performed on the project card.
        /// Can be "created", "edited", "converted", "moved", or "deleted".
        action: String,
        /// The changes to the project card if the action was "edited" or "converted".
        /// `changes[note][from]: String` The previous version of the note if the action was "edited" or "converted".
        // FIXME should be enum
        changes: Option<serde_json::Value>,
        /// The id of the card that this card now follows if the action was "moved".
        /// Will be `null` if it is the first card in a column.
        after_id: Option<isize>,
        /// The [project card](https://developer.github.com/v3/projects/cards) itself.
        project_card: ProjectCard,
        repository: Repository,
        sender: Sender,
    },

    /// Triggered when a [project column](https://developer.github.com/v3/projects/columns) is created, updated, moved, or deleted.
    ProjectColumnEvent {
        /// The action that was performed on the project column.
        /// Can be one of "created", "edited", "moved" or "deleted".
        action: String,
        /// The changes to the project column if the action was "edited".
        /// `changes[name][from]: String` The previous version of the name if the action was "edited".
        changes: serde_json::Value,
        /// The id of the column that this column now follows if the action was "moved". Will be null if it is the first column in a project.
        after_id: Option<isize>,
        /// The [project column](https://developer.github.com/v3/projects/columns) itself.
        project_column: ProjectColumn,
        repository: Repository,
        sender: Sender,
    },

    /// Triggered when a [project](https://developer.github.com/v3/projects/) is created, updated, closed, reopened, or deleted.
    ProjectEvent {
        /// The action that was performed on the project. Can be one of "created", "edited", "closed", "reopened", or "deleted".
        action: String,
        /// The changes to the project if the action was "edited".
        /// `changes[name][from]: String` The previous version of the name if the action was "edited".
        /// `changes[body][from]: String` The previous version of the body if the action was "edited".
        changes: serde_json::Value,
        /// The [project](https://developer.github.com/v3/projects/) itself.
        project: Project,
        repository: Repository,
        sender: Sender,
    },

    /// Triggered when a private repository is open sourced.
    /// Without a doubt: the best GitHub event.
    PublicEvent {
        repository: Repository,
        sender: Sender,
    },

    /// Triggered when a pull request is assigned, unassigned, labeled, unlabeled,
    /// opened, edited, closed, reopened, or synchronized.
    /// Also triggered when a pull request review is requested,
    /// or when a review request is removed.
    PullRequestEvent {
        /// The action that was performed.
        /// Can be one of "assigned", "unassigned", "review_requested",
        /// "review_request_removed", "labeled", "unlabeled",
        /// "opened", "edited", "closed", or "reopened".
        ///
        /// If the action is "closed" and the `merged` key is `false`,
        /// the pull request was closed with unmerged commits.
        /// If the action is "closed" and the `merged` key is `true`,
        /// the pull request was merged.
        ///
        /// While webhooks are also triggered when a pull request is synchronized,
        /// Events API timelines don't include pull request events with the "synchronize" action.
        action: String,
        /// The pull request number.
        number: i64,
        /// The changes to the comment if the action was "edited".
        /// `changes[title][from]: String` The previous version of the title if the action was "edited".
        /// `changes[body][from]: String` The previous version of the body if the action was "edited".
        changes: serde_json::Value,
        /// The [pull request](https://developer.github.com/v3/pulls) itself.
        pull_request: PullRequest,
        repository: Repository,
        sender: Sender,
    },

    /// Triggered when a pull request review is submitted into a non-pending state, the body is
    /// edited, or the review is dismissed.
    PullRequestReviewEvent {
        /// The action that was performed on the comment. Can be one of "created", "edited", or "deleted".
        action: String,
        /// The changes to the comment if the action was "edited".
        /// `changes[body][from]: String` The previous version of the body if the action was "edited".
        changes: serde_json::Value,
        review: Review,
        /// The [pull request](https://developer.github.com/v3/pulls/) the comment belongs to.
        pull_request: PullRequest,
        /// The [comment](https://developer.github.com/v3/pulls/comments) itself.
        repository: Repository,
        sender: Sender,
    },

    /// Triggered when a [comment on a pull request's unified diff](https://developer.github.com/v3/pulls/comments) is created, edited, or deleted (in the Files Changed tab).
    PullRequestReviewCommentEvent {
        /// The action that was performed on the comment. Can be one of "created", "edited", or "deleted".
        action: String,
        /// The [comment](https://developer.github.com/v3/pulls/comments) itself.
        comment: Comment,
        /// The changes to the comment if the action was "edited".
        /// `changes[body][from]: String` The previous version of the body if the action was "edited".
        changes: serde_json::Value,
        ///	The [pull request](https://developer.github.com/v3/pulls/) the comment belongs to.
        pull_request: PullRequest,
        repository: Repository,
        sender: Sender,
    },

    /// Triggered on a push to a repository branch.
    /// Branch pushes and repository tag pushes also trigger webhook [`push` events](https://developer.github.com/webhooks/#events).
    ///		Note: The webhook payload example following the table differs significantly from
    ///		theents API payload described in the table. Among other differences, the webhook
    ///		payload includes both sender and pusher objects. Sender and pusher are the same user
    ///		who initiated the push event, but the sender object contains more detail.
    PushEvent {
        // FIXME the note
        /// The full Git ref that was pushed. Example: `refs/heads/master`.
        #[serde(rename = "ref")]
        ref_field: String,
        /// The SHA of the most recent commit on `ref` after the push.
        head: Option<String>,
        /// The SHA of the most recent commit on `ref` before the push.
        before: String,
        after: String,
        /// The number of commits in the push.
        size: isize,
        created: bool,
        deleted: bool,
        forced: bool,
        base_ref: ::serde_json::Value,
        compare: String,
        /// An array of commit objects describing the pushed commits.
        /// (The array includes a maximum of 20 commits.
        /// If necessary, you can use the Commits API to fetch additional commits.
        /// This limit is applied to timeline events only and isn't applied to webhook deliveries.)
        commits: Vec<Commit>,
        head_commit: ::serde_json::Value,
        repository: Repository,
        pusher: Pusher,
        sender: Sender,
    },

    /// Triggered when a
    /// [release](https://developer.github.com/v3/repos/releases/#get-a-single-release) is published.
    ReleaseEvent {
        /// The action that was performed. Currently, can only be "published".
        action: String,
        /// The [release](https://developer.github.com/v3/repos/releases/#get-a-single-release) itself.
        release: Release,
        repository: Repository,
        sender: Sender,
    },

    /// Triggered when a repository is created, archived, unarchived, made public, or made private.
    /// [Organization hooks](https://developer.github.com/v3/orgs/hooks/) are also triggered when a repository is deleted.
    ///
    /// Events of this type are not visible in timelines. These events are only used to trigger hooks.
    RepositoryEvent {
        /// The action that was performed. This can be one of `created`, `deleted` (organization hooks only), `archived`, `unarchived`, `publicized`, or `privatized`.
        action: String,
        /// The [repository](https://developer.github.com/v3/repos/) itself.
        repository: Repository,
        sender: Sender,
    },

    /// Triggered when a successful or unsuccessful repository import finishes
    /// for a GitHub organization or a personal repository.
    /// To receive this event for a personal repository,
    /// you must create an empty repository prior to the import.
    /// This event can be triggered using either the [GitHub Importer](https://help.github.com/articles/importing-a-repository-with-github-importer/)
    /// or the [Source imports API](https://developer.github.com/v3/migrations/source_imports/).
    RepositoryImportEvent {
        /// The final state of the import. This can be either `success` or `failure`.
        status: String,
        /// The [repository](https://developer.github.com/v3/repos/) you are importing.
        repository: Repository,
        /// The information about the organization where the imported repository will live.
        organization: Organization,
        /// The GitHub user who is importing the repository.
        sender: Sender,
    },

    /// Triggered when a [security alert](https://help.github.com/articles/about-security-alerts-for-vulnerable-dependencies/) is created, dismissed, or resolved.
    RepositoryVulnerabilityAlertEvent {
        /// The action that was performed. This can be one of `create`, `dismiss`, or `resolve`.
        action: String,

        /// The security alert of the vulnerable dependency.
        alert: Alert,
    },

    /// Triggered when a new security advisory is published, updated, or withdrawn.
    /// A security advisory provides information about security-related vulnerabilities in software on GitHub.
    /// Security Advisory webhooks are available to GitHub Apps only.
    /// The security advisory dataset also powers the GitHub security alerts,
    /// see "[About security alerts for vulnerable dependencies](https://help.github.com/articles/about-security-alerts-for-vulnerable-dependencies/)."
    SecurityAdvisoryEvent {
        /// The action that was performed. The action can be one of `published`, `updated`, or `performed` for all new events.
        action: String,
        /// The details of the security advisory, including summary, description, and severity.
        security_advisory: SecurityAdvisory,
    },

    /// Triggered when the status of a Git commit changes.
    /// Events of this type are not visible in timelines. These events are only used to trigger hooks.
    StatusEvent {
        id: i64,
        /// The Commit SHA.
        sha: String,
        name: String,
        /// The optional link added to the status.
        // FIXME will Option parse {}?
        target_url: Option<String>,
        context: String,
        /// The optional human-readable description added to the status.
        // FIXME will Option parse {}?
        description: Option<String>,
        /// The new state. Can be `pending`, `success`, `failure`, or `error`.
        state: String,
        commit: Commit,
        /// An array of branch objects containing the status' SHA.
        /// Each branch contains the given SHA, but the SHA may or may not be the head of the branch.
        /// The array includes a maximum of 10 branches.
        branches: Vec<Bran>,
        created_at: String,
        updated_at: String,
        repository: Repository,
        sender: Sender,
    },

    /// Triggered when an organization's team is created or deleted.
    ///
    /// Events of this type are not visible in timelines. These events are only used to trigger organization hooks.
    TeamEvent {
        /// The action that was performed.
        /// Can be one of `Created`, `Deleted`, `Edited`, `AddedToRepository`, or `RemovedFromRepository`.
        action: actions::TeamEvent,
        /// The team itself.
        team: Team,
        /// The changes to the team if the action was "edited".
        /// `changes[description][from]: String` The previous version of the description if the action was `edited`.
        /// `changes[name][from]: String` The previous version of the name if the action was `edited`.
        /// The previous version of the team's privacy if the action was `edited`.
        ///
        /// `changes[repository][permissions][from][admin]: bool`
        /// The previous version of the team member's `admin` permission on a repository, if the action was `edited`.
        ///
        /// `changes[repository][permissions][from][pull]: bool`
        /// The previous version of the team member's `pull` permission on a repository, if the action was `edited`.
        ///
        /// `changes[repository][permissions][from][push]: bool`
        /// The previous version of the team member's `push` permission on a repository, if the action was `edited`.
        changes: serde_json::Value,
        /// The repository that was added or removed from to the team's purview if the action was `added_to_repository`, `removed_from_repository`, or `edited`. For `edited` actions, `repository` also contains the team's new permission levels for the repository.
        repository: TeamEventRepository,
        organization: Organization,
        sender: Sender,
    },

    /// Triggered when a [repository is added to a
    /// team](https://developer.github.com/v3/teams/#add-or-update-team-repository).
    ///
    /// Events of this type are not visible in timelines. These events are only used to trigger hooks.
    TeamAddEvent {
        /// The [team](https://developer.github.com/v3/teams/) that was modified. Note: older events may not include this in the payload.
        team: Team,
        /// The [repository](https://developer.github.com/v3/repos/) that was added to this team.
        repository: Repository,
        organization: Organization,
        sender: Sender,
    },

    /// The WatchEvent is related to [starring a repository](https://developer.github.com/v3/activity/starring/#star-a-repository),
    /// not [watching](https://developer.github.com/v3/activity/watching/).
    /// See [this API blog post](https://developer.github.com/changes/2012-09-05-watcher-api/) for an explanation.
    ///
    /// The event’s actor is the [user](https://developer.github.com/v3/users/) who starred a repository,
    /// and the event’s repository is the [repository](https://developer.github.com/v3/repos/) that was starred.
    WatchEvent {
        /// The action that was performed. Currently, can only be `started`.
        action: String,
        repository: Repository,
        sender: Sender,
    },
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct IssueEvent {
    /// The action that was performed. Can be one of `opened`, `edited`, `deleted`, `transferred`, `closed`,
    /// `reopened`, `assigned`, `unassigned`, `labeled`, `unlabeled`, `milestoned`, or `demilestoned`.
    action: String,
    /// The [issue](https://developer.github.com/v3/issues) itself.
    issue: Issue,
    /// The changes to the issue if the action was "edited".
    /// `changes[title][from]: String` The previous version of the title if the action was "edited".
    /// `changes[body][from]:String` The previous version of the body if the action was "edited".
    changes: Option<::serde_json::Value>,
    repository: Repository,
    sender: Sender,
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
    /// The result of the completed `check` run.
    /// Can be one of `success,` `failure,` `neutral,` `cancelled,`
    /// timed_out, or `action_required.`
    /// This value will be `null` until the check run has `completed.`
    // FIXME should be enum
    conclusion: Option<String>,
    started_at: String,
    completed_at: String,
    output: Output,
    /// The name of the check run.
    name: String,
    check_suite: CheckSuite,
    app: App,
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
    /// The summary status for all check runs that are part of the check suite.
    /// Can be `requested`, `in_progress`, or `completed`.
    status: String,
    /// The summary conclusion for all check runs that are part of the check suite. Can be one
    /// `success`, `failure`, `neutral`, `cancelled`, `timed_out`, or `action_required`.
    /// This value will be `null` until the check run has `completed`.
    conclusion: String,
    /// URL that points to the check suite API resource.
    url: String,
    before: String,
    after: String,
    /// An array of pull requests that match this check suite. A pull request matches a check suite if
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
struct Repository {
    id: i64,
    node_id: String,
    name: String,
    full_name: String,
    owner: Owner,
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
    account: Account,
    repository_selection: String,
    access_tokens_url: String,
    repositories_url: String,
    html_url: String,
    app_id: i64,
    target_id: i64,
    target_type: String,
    permissions: Permissions,
    events: Vec<String>,
    created_at: i64,
    updated_at: i64,
    single_file_name: String,
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
    /// The git author's name.
    name: String,
    /// The git author's email address.
    email: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Committer {
    name: String,
    email: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct User {
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
struct Comment {
    url: String,
    html_url: String,
    id: i64,
    node_id: String,
    user: User,
    position: ::serde_json::Value,
    line: ::serde_json::Value,
    path: ::serde_json::Value,
    commit_id: String,
    created_at: String,
    updated_at: String,
    author_association: String,
    body: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Deployment {
    url: String,
    id: i64,
    node_id: String,
    sha: String,
    #[serde(rename = "ref")]
    ref_field: String,
    task: String,
    payload: Payload,
    environment: String,
    description: ::serde_json::Value,
    creator: Creator,
    created_at: String,
    updated_at: String,
    statuses_url: String,
    repository_url: String,
}

/// FIXME Empty?
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Payload {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Creator {
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
struct DeploymentStatus {
    url: String,
    id: i64,
    node_id: String,
    /// The new state. Can be `pending`, `success`, `failure`, or `error`.
    state: String,
    creator: Creator,
    /// The optional human-readable description added to the status.
    description: String,
    /// The optional link added to the status.
    target_url: String,
    created_at: String,
    updated_at: String,
    deployment_url: String,
    repository_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Forkee {
    id: i64,
    node_id: String,
    name: String,
    full_name: String,
    owner: Owner,
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
    public: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Page {
    /// The name of the page.
    page_name: String,
    /// The current page title.
    title: String,
    summary: ::serde_json::Value,
    /// The action that was performed on the page. Can be "created" or "edited".
    action: String,
    /// The latest commit SHA of the page.
    sha: String,
    /// Points to the HTML wiki page.
    html_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Account {
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
struct Permissions {
    metadata: String,
    contents: String,
    issues: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct PartialRepository {
    id: i64,
    name: String,
    full_name: String,
    private: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct RepositoriesRemoved {
    id: i64,
    name: String,
    full_name: String,
    private: bool,
}

/// Triggered when an [issue comment](https://developer.github.com/v3/issues/comments/) is created, edited, or deleted.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Issue {
    url: String,
    repository_url: String,
    labels_url: String,
    comments_url: String,
    events_url: String,
    html_url: String,
    id: i64,
    node_id: String,
    number: i64,
    title: String,
    user: User,
    /// The optional labels that were added or removed from the issue.
    labels: Vec<Label>,
    state: String,
    locked: bool,
    /// The optional user who was assigned or unassigned from the issue.
    assignee: ::serde_json::Value,
    assignees: Vec<::serde_json::Value>,
    milestone: ::serde_json::Value,
    comments: i64,
    created_at: String,
    updated_at: String,
    closed_at: ::serde_json::Value,
    author_association: String,
    body: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Label {
    id: i64,
    node_id: String,
    url: String,
    name: String,
    color: String,
    default: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Member {
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
struct MemberEventChanges {
    /// The previous permissions of the collaborator if the action was `edited`
    permission: Permission,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Permission {
    from: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Team {
    name: String,
    id: i64,
    node_id: String,
    slug: String,
    description: String,
    privacy: String,
    url: String,
    members_url: String,
    repositories_url: String,
    permission: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Milestone {
    url: String,
    html_url: String,
    labels_url: String,
    id: i64,
    node_id: String,
    number: i64,
    title: String,
    description: String,
    creator: Creator,
    open_issues: i64,
    closed_issues: i64,
    state: String,
    created_at: String,
    updated_at: String,
    due_on: String,
    closed_at: ::serde_json::Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Membership {
    url: String,
    state: String,
    role: String,
    organization_url: String,
    user: User,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Build {
    url: String,
    status: String,
    error: Error,
    pusher: Pusher,
    commit: String,
    duration: i64,
    created_at: String,
    updated_at: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Error {
    message: ::serde_json::Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Pusher {
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
struct ProjectCard {
    url: String,
    project_url: String,
    column_url: String,
    column_id: i64,
    id: i64,
    node_id: String,
    note: String,
    creator: Creator,
    created_at: String,
    updated_at: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct ProjectColumn {
    url: String,
    project_url: String,
    cards_url: String,
    id: i64,
    node_id: String,
    name: String,
    created_at: String,
    updated_at: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Project {
    owner_url: String,
    url: String,
    html_url: String,
    columns_url: String,
    id: i64,
    node_id: String,
    name: String,
    body: String,
    number: i64,
    state: String,
    creator: Creator,
    created_at: String,
    updated_at: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct PullRequest {
    url: String,
    id: i64,
    node_id: String,
    html_url: String,
    diff_url: String,
    patch_url: String,
    issue_url: String,
    number: i64,
    state: String,
    locked: bool,
    title: String,
    user: User,
    body: String,
    created_at: String,
    updated_at: String,
    closed_at: String,
    merged_at: ::serde_json::Value,
    merge_commit_sha: String,
    assignee: ::serde_json::Value,
    assignees: Vec<::serde_json::Value>,
    requested_reviewers: Vec<::serde_json::Value>,
    requested_teams: Vec<::serde_json::Value>,
    labels: Vec<::serde_json::Value>,
    milestone: ::serde_json::Value,
    commits_url: String,
    review_comments_url: String,
    review_comment_url: String,
    comments_url: String,
    statuses_url: String,
    head: Head,
    base: Base,
    _links: Links,
    author_association: String,
    merged: bool,
    mergeable: bool,
    rebaseable: bool,
    mergeable_state: String,
    merged_by: ::serde_json::Value,
    comments: i64,
    review_comments: i64,
    maintainer_can_modify: bool,
    commits: i64,
    additions: i64,
    deletions: i64,
    changed_files: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Head {
    label: String,
    #[serde(rename = "ref")]
    ref_field: String,
    sha: String,
    user: User,
    repo: Repository,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Base {
    label: String,
    #[serde(rename = "ref")]
    ref_field: String,
    sha: String,
    user: User,
    repo: Repository,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Links {
    #[serde(rename = "self")]
    self_field: Link,
    html: Link,
    issue: Link,
    comments: Link,
    review_comments: Link,
    review_comment: Link,
    commits: Link,
    statuses: Link,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Link {
    href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Review {
    id: i64,
    node_id: String,
    user: User,
    body: ::serde_json::Value,
    commit_id: String,
    submitted_at: String,
    state: String,
    html_url: String,
    pull_request_url: String,
    author_association: String,
    _links: ReviewLinks,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct ReviewLinks {
    html: Link,
    pull_request: PullRequest,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Commit {
    /// The SHA of the commit.
    sha: String,
    /// The commit message.
    message: String,
    /// The git author of the commit.
    author: Author,
    /// URL that points to the commit API resource.
    url: String,
    /// Whether this commit is distinct from any that have been pushed before.
    distinct: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Release {
    url: String,
    assets_url: String,
    upload_url: String,
    html_url: String,
    id: i64,
    node_id: String,
    tag_name: String,
    target_commitish: String,
    name: ::serde_json::Value,
    draft: bool,
    author: ReleaseAuthor,
    prerelease: bool,
    created_at: String,
    published_at: String,
    assets: Vec<::serde_json::Value>,
    tarball_url: String,
    zipball_url: String,
    body: ::serde_json::Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct ReleaseAuthor {
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
struct Alert {
    id: i64,
    affected_range: String,
    affected_package_name: String,
    external_reference: String,
    external_identifier: String,
    fixed_in: String,
    dismisser: User,
    dismiss_reason: String,
    dismissed_at: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct SecurityAdvisory {
    ghsa_id: String,
    summary: String,
    description: String,
    severity: String,
    identifiers: Vec<Identifier>,
    references: Vec<Reference>,
    published_at: String,
    updated_at: String,
    withdrawn_at: ::serde_json::Value,
    vulnerabilities: Vec<Vulnerability>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Identifier {
    value: String,
    #[serde(rename = "type")]
    type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Reference {
    url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Vulnerability {
    package: Package,
    severity: String,
    vulnerable_version_range: String,
    first_patched_version: FirstPatchedVersion,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Package {
    ecosystem: String,
    name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct FirstPatchedVersion {
    identifier: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct StatusEventCommitNode {
    sha: String,
    node_id: String,
    commit: CommitTree,
    url: String,
    html_url: String,
    comments_url: String,
    author: AuthorDate,
    committer: CommitterDate,
    parents: Vec<::serde_json::Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct CommitTree {
    author: AuthorDate,
    committer: CommitterDate,
    message: String,
    tree: Tree,
    url: String,
    comment_count: i64,
    verification: Verification,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct AuthorDate {
    name: String,
    email: String,
    date: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct CommitterDate {
    name: String,
    email: String,
    date: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Tree {
    sha: String,
    url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Verification {
    verified: bool,
    reason: String,
    signature: String,
    payload: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Bran {
    name: String,
    commit: Commit,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct TeamEventRepository {
    id: i64,
    node_id: String,
    name: String,
    full_name: String,
    owner: Owner,
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
    permissions: TeamEventPermissions,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct TeamEventPermissions {
    pull: bool,
    push: bool,
    admin: bool,
}
