//! An implementation of the todo.txt format.
//!
//! The typical workflow is to read in a line from a todo file and call [Task::new], perform some
//! action on the [Task], and then to write it back using [Task::to_string]. For example:
//!
//! ```
//! let example = "Document this crate".to_string();
//! let mut task = todo_txt::Task::new(&example);
//!
//! task.projects.push("rust".to_string());
//! task.is_complete = true;
//!
//! assert_eq!(task.to_string(), "x Document this crate +rust".to_string());
//! ```
use lazy_static::lazy_static;
use regex::Regex;
use std::fmt;

/// A task.
#[derive(Debug, Clone, PartialEq)]
pub struct Task {
    pub is_complete: bool,
    pub priority: Option<char>,
    pub description: String,
    pub projects: Vec<String>,
    pub contexts: Vec<String>,
    pub attributes: Vec<(String, String)>,
}

impl Task {
    /// Creates a new task from a string.
    ///
    /// # Examples
    ///
    /// ```
    /// let example = "(B) Write some code +rust @work due:tomorrow".to_string();
    /// let task = todo_txt::Task::new(&example);
    ///
    /// assert_eq!(task, todo_txt::Task {
    ///     is_complete: false,
    ///     priority: Some('B'),
    ///     description: "Write some code".to_string(),
    ///     projects: vec!["rust".to_string()],
    ///     contexts: vec!["work".to_string()],
    ///     attributes: vec![("due".to_string(), "tomorrow".to_string())],
    /// });
    /// ```
    ///
    /// ```
    /// let example = "x Buy eggs @shopping @home".to_string();
    /// let task = todo_txt::Task::new(&example);
    ///
    /// assert_eq!(task, todo_txt::Task {
    ///     is_complete: true,
    ///     priority: None,
    ///     description: "Buy eggs".to_string(),
    ///     projects: vec![],
    ///     contexts: vec!["shopping".to_string(), "home".to_string()],
    ///     attributes: vec![],
    /// });
    /// ```
    pub fn new(task: &String) -> Self {
        lazy_static! {
            static ref TASK_EXP: Regex =
                Regex::new("(?P<c>x\\s)?(?P<p>\\([A-Z]\\)\\s)?(?P<d>.*)").unwrap();
            static ref PROJECT_EXP: Regex = Regex::new("\\+\\w+").unwrap();
            static ref CONTEXT_EXP: Regex = Regex::new("@\\w+").unwrap();
            static ref KEY_VALUE_EXP: Regex = Regex::new("(?P<k>\\w+):(?P<v>\\w+)").unwrap();
        }

        // We can unwrap this because there might be an empty line, but there will never be
        // a `None` description.
        let captures = TASK_EXP.captures(&task).unwrap();

        let is_complete = captures.name("c").is_some();
        let priority = if let Some(priority) = captures.name("p") {
            priority.as_str().chars().nth(1)
        } else {
            None
        };

        // We can unwrap this because there might be an empty description, but there will never be
        // a `None` description.
        let description = captures.name("d").unwrap().as_str();

        let projects = PROJECT_EXP
            .find_iter(description)
            .map(|m| m.as_str().get(1..).unwrap().to_string()) // If `PROJECT_EXP` matched, there is always at least one char.
            .collect();
        let description = PROJECT_EXP.replace_all(description, "").to_string();

        let contexts = CONTEXT_EXP
            .find_iter(&description)
            .map(|m| m.as_str().get(1..).unwrap().to_string()) // If `CONTEXT_EXP` matched, there is always at least one char.
            .collect();
        let description = CONTEXT_EXP.replace_all(&description, "").to_string();

        let attributes = KEY_VALUE_EXP
            .captures_iter(&description)
            .map(|c| {
                (
                    c.name("k").unwrap().as_str().to_string(),
                    c.name("v").unwrap().as_str().to_string(),
                )
            })
            .collect();
        let description = KEY_VALUE_EXP.replace_all(&description, "").to_string();

        Self {
            is_complete,
            priority,
            description: description.trim().to_string(),
            projects,
            contexts,
            attributes,
        }
    }

    /// Returns the task as a string.
    ///
    /// # Examples
    ///
    /// ```
    /// let task = todo_txt::Task {
    ///     is_complete: true,
    ///     priority: Some('C'),
    ///     description: "Take out the trash".to_string(),
    ///     projects: vec![],
    ///     contexts: vec!["home".to_string()],
    ///     attributes: vec![("day".to_string(), "wednesdays".to_string())],
    /// };
    ///
    /// assert_eq!(task.to_string(), "x (C) Take out the trash @home day:wednesdays".to_string());
    /// ```
    pub fn to_string(&self) -> String {
        let mut string = String::new();

        if self.is_complete {
            string.push_str("x ");
        }

        if let Some(p) = self.priority {
            string.push_str(&format!("({}) ", p));
        }

        string.push_str(&format!("{} ", &self.description));

        self.projects.iter().for_each(|project| {
            string.push_str(&format!("+{} ", project));
        });

        self.contexts.iter().for_each(|context| {
            string.push_str(&format!("@{} ", context));
        });

        self.attributes.iter().for_each(|(key, value)| {
            string.push_str(&format!("{}:{} ", key, value));
        });

        string.trim_end().to_string()
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
