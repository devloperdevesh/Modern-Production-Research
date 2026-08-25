#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Route {
    ControlPlane,
    DataPlane,
}

pub const fn select_route(data_plane: bool) -> Route {
    if data_plane {
        Route::DataPlane
    } else {
        Route::ControlPlane
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selects_data_plane() {
        assert_eq!(select_route(true), Route::DataPlane);
    }
}
