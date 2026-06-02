use leptos_router::{
    AsPath, ParamSegment, PartialPathMatch, PathSegment, PossibleRouteMatch, StaticSegment,
};

use super::routes::Routes;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct StaticSegmentList<T: AsPath + std::fmt::Debug> {
    segments: Vec<StaticSegment<T>>,
}

impl From<&'static str> for StaticSegmentList<&'static str> {
    fn from(path: &'static str) -> Self {
        let segments = path
            .trim_start_matches("/")
            .split("/")
            .map(StaticSegment)
            .collect();

        Self { segments }
    }
}

impl<T> PossibleRouteMatch for StaticSegmentList<T>
where
    T: AsPath + std::fmt::Debug,
{
    fn optional(&self) -> bool {
        false
    }

    fn test<'a>(&self, path: &'a str) -> Option<PartialPathMatch<'a>> {
        let mut matched_len = 0;
        let mut remaining = path;

        for segment in &self.segments {
            let current_match = segment.test(remaining);
            if let Some(current_match) = current_match {
                remaining = current_match.remaining();
                matched_len += current_match.matched().len();
            } else {
                return None;
            }
        }

        let partial_match = PartialPathMatch::new(remaining, Vec::new(), &path[0..matched_len]);

        Some(partial_match)
    }

    fn generate_path(&self, path: &mut Vec<PathSegment>) {
        // Add each segment's contribution to the path
        for segment in &self.segments {
            segment.generate_path(path);
        }
    }
}

pub trait LeptosRoutes<T> {
    fn list_segments() -> T;
    fn new_segments() -> T;
    fn edit_segments() -> (StaticSegmentList<&'static str>, ParamSegment);
    fn read_segments() -> (StaticSegmentList<&'static str>, ParamSegment);
}

impl<T: Routes> LeptosRoutes<StaticSegmentList<&'static str>> for T {
    fn list_segments() -> StaticSegmentList<&'static str> {
        T::list_path().into()
    }

    fn new_segments() -> StaticSegmentList<&'static str> {
        T::new_path().into()
    }

    fn edit_segments() -> (StaticSegmentList<&'static str>, ParamSegment) {
        (T::edit_path().into(), ParamSegment(T::edit_param()))
    }

    fn read_segments() -> (StaticSegmentList<&'static str>, ParamSegment) {
        (T::read_path().into(), ParamSegment(T::read_param()))
    }
}

