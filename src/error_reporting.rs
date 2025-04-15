use std::convert::Infallible;
pub use urlencoding;

#[non_exhaustive]
#[derive(Clone, Debug)]
pub struct GitHubBugReporter {
    pub repo_owner: String,
    pub repo_name: String,
}

impl GitHubBugReporter {
    pub fn new(repo_owner: String, repo_name: String) -> Self {
        Self {
            repo_owner,
            repo_name,
        }
    }
}

impl ReportBugUrlMaker for GitHubBugReporter {
    fn get_report_url(&self, payload: Option<String>, bug_report: String) -> String {
        format!(
            "https://github.com/{}/{}/issues/new?title=Unhandled panic: {}&body={}",
            self.repo_owner,
            self.repo_name,
            urlencoding::encode(&payload.unwrap_or_default()),
            urlencoding::encode(&format!("### Panic report\n{bug_report}"))
        )
    }
}

/// Generates a URL for bug reports - to be used as [`AppInfo::report_bug_url`](crate::AppInfo::report_bug_url).
///
/// If you are using GitHub, you can use the [`GitHubBugReporter`] function.
pub trait ReportBugUrlMaker: Clone + Send + Sync + 'static {
    fn get_report_url(&self, payload: Option<String>, bug_report: String) -> String;
}

impl<T: Fn(Option<String>, String) -> String + Clone + Send + Sync + 'static> ReportBugUrlMaker
    for T
{
    fn get_report_url(&self, payload: Option<String>, bug_report: String) -> String {
        self(payload, bug_report)
    }
}

impl ReportBugUrlMaker for Infallible {
    fn get_report_url(&self, _payload: Option<String>, _bug_report: String) -> String {
        eprintln!("Called `get_report_url` of `Infallible` - `Infallible` should never exist, but here it is: {self:?}");
        String::new()
    }
}

/// Type alias for [`AppInfo`](crate::AppInfo) to help solve generics inference issue in case of no bug reporting.
///
/// Just set `report_bug_url` field to [`None`] while using this alias.
///
/// Note: It is recommended to disable the `error-reporting` feature instead of using this alias.
pub type AppInfoNoBugReport = super::AppInfo<Infallible>;
