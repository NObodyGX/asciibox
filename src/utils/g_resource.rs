use gtk::gio;

pub fn load_gresource(path: &'static str) -> String {
    let content = match gio::resources_lookup_data(path, gio::ResourceLookupFlags::NONE) {
        Ok(data) => match String::from_utf8((&data).to_vec()) {
            Ok(ctx) => ctx,
            Err(e) => {
                log::error!("failed to string from_utf8 from gresource: {e}");
                String::new()
            }
        },
        Err(e) => {
            log::error!("failed to load {path} from gresource: {e}");
            String::new()
        }
    };
    content
}
