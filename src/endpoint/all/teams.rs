crate::endpoint::list_endpoint!(ListTeams("/teams") => crate::model::team::Team);
crate::endpoint::get_endpoint!(GetTeam("/teams") => crate::model::team::Team);
