crate::endpoint::list_endpoint!(ListPlayers("/players") => crate::model::player::Player);
crate::endpoint::get_endpoint!(GetPlayer("/players") => crate::model::player::Player);
