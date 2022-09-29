use marketplace_core::dto::Project;

pub fn find_by_title(projects: &[Project], title: &str) -> Option<Project> {
	projects.iter().find(|project| project.title == title).cloned()
}
