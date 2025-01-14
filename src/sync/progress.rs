use console::{style, Emoji};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

pub struct SyncProgress {
    max_tool_size: usize,
    multi_progress: MultiProgress,
}

const SUCCESS: Emoji<'_, '_> = Emoji("✅  ", "OK ");
const FAILURE: Emoji<'_, '_> = Emoji("⛔  ", "NO ");
const PROCESS: Emoji<'_, '_> = Emoji("📥  ", ".. ");

impl SyncProgress {
    /// Creates new `SyncProgress` from a list of tools.
    /// !!! The given `Vec` must be non-empty !!!
    pub fn new(tools: Vec<String>) -> SyncProgress {
        // unwrap is safe here because 'new' is called with a non-empty vector
        let max_tool_size = tools.iter().map(|tool| tool.len()).max().unwrap();

        let multi_progress = MultiProgress::new();

        SyncProgress {
            max_tool_size,
            multi_progress,
        }
    }

    fn fmt_prefix(&self, emoji: Emoji, tool_name: &str) -> String {
        let aligned_tool = format!("{:width$}", tool_name, width = self.max_tool_size);

        format!("{}{}", emoji, aligned_tool)
    }

    pub fn create_message_bar(&self, tool_name: &str) -> ProgressBar {
        let message_style = ProgressStyle::with_template("{prefix:.bold.dim} {msg}").unwrap();

        self.multi_progress.add(
            ProgressBar::new(100)
                .with_style(message_style)
                .with_prefix(self.fmt_prefix(PROCESS, tool_name)),
        )
    }

    pub fn create_progress_bar(&self, size: u64) -> ProgressBar {
        let bar_style =
            ProgressStyle::with_template("{bytes}/{total_bytes} {wide_bar:.cyan/blue}").unwrap();

        self.multi_progress
            .add(ProgressBar::new(size).with_style(bar_style))
    }

    pub fn finish_progress(pb: ProgressBar) {
        pb.finish_and_clear()
    }

    pub fn success(&self, pb: ProgressBar, tool_name: &str) {
        pb.set_prefix(self.fmt_prefix(SUCCESS, tool_name));

        let success_msg = format!("{}", style("Completed!").bold().green());
        pb.set_message(success_msg);
        pb.finish();
    }

    pub fn failure(&self, pb: ProgressBar, tool_name: &str, err_msg: String) {
        pb.set_prefix(self.fmt_prefix(FAILURE, tool_name));

        let failure_msg = format!("{}", style(err_msg).red());
        pb.set_message(failure_msg);
        pb.finish();
    }
}
