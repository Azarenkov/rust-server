pub enum Functions {
    GetUserData,
    GetAllCourses,
    GetDeadlines,
    GetGrades,
    GetGradesOverview
}
impl Functions {
    pub fn new(&self) -> &'static str {
        match self {
            Functions::GetUserData => "core_webservice_get_site_info",
            Functions::GetAllCourses => "core_enrol_get_users_courses",
            Functions::GetDeadlines => "core_calendar_get_action_events_by_timesort",
            Functions::GetGrades => "gradereport_user_get_grade_items",
            Functions::GetGradesOverview => "gradereport_overview_get_course_grades"
        }
    }
}