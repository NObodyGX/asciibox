use adw::subclass::prelude::*;
use gtk::CompositeTemplate;
use gtk::gio;
use gtk::glib;
use gtk::prelude::*;
use gtk::prelude::{TextBufferExt, TextViewExt};
use log::error;
use sourceview;
use std::cell::{Cell, OnceCell};
use webkit::WebView;
use webkit::prelude::*;

const DEFAULT_CONTENT: &str = "
<html>
<body>
    <script src=\"https://cdn.jsdelivr.net/npm/mermaid/dist/mermaid.min.js\"></script>
    <script>mermaid.initialize({ startOnLoad: true });</script>
    <div class=\"mermaid\">
        @@ASCIIBOX-NOBODYGX-PLACEHOLD@@
    </div>
</body>
</html>
";

mod imp {

    use super::*;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/com/github/nobodygx/asciibox/ui/page_mermaid.ui")]
    pub struct MermaidPage {
        #[template_child]
        pub in_view: TemplateChild<sourceview::View>,
        #[template_child]
        pub obox: TemplateChild<gtk::Box>,

        pub html_content: OnceCell<String>,
        pub webview: OnceCell<WebView>,
        pub cur_zoom: Cell<f64>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for MermaidPage {
        const NAME: &'static str = "MermaidPage";
        type Type = super::MermaidPage;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();

            klass.install_action("mermaid.execute-transform", None, move |obj, _, _| {
                obj.execute_transform();
            });

            klass.install_action("mermaid.execute-clear", None, move |obj, _, _| {
                obj.execute_clear();
            });

            klass.install_action("mermaid.execute-copy-result", None, move |obj, _, _| {
                obj.execute_copy_result();
            });

            klass.install_action("mermaid.zoom-in", None, move |obj, _, _| {
                obj.zoom_in();
            });

            klass.install_action("mermaid.zoom-out", None, move |obj, _, _| {
                obj.zoom_out();
            });
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for MermaidPage {
        fn constructed(&self) {
            let obj = self.obj();
            self.parent_constructed();

            obj.setup_html();
            obj.setup_config();
            obj.setup_view();
        }
    }
    impl WidgetImpl for MermaidPage {}
    impl BinImpl for MermaidPage {}
}

glib::wrapper! {
    pub struct MermaidPage(ObjectSubclass<imp::MermaidPage>)
    @extends gtk::Widget, adw::Bin,
    @implements gtk::Accessible;
}

impl MermaidPage {
    pub fn new() -> Self {
        let page: MermaidPage = glib::Object::new();
        page
    }
    fn setup_html(&self) {
        let path = "/com/github/nobodygx/asciibox/html/index.html";
        let content = match gio::resources_lookup_data(path, gio::ResourceLookupFlags::NONE) {
            Ok(data) => match String::from_utf8((&data).to_vec()) {
                Ok(ctx) => ctx,
                Err(e) => {
                    error!("failed to string from_utf8 from gresource: {e}");
                    DEFAULT_CONTENT.to_string()
                }
            },
            Err(e) => {
                error!("failed to load {path} from gresource: {e}");
                DEFAULT_CONTENT.to_string()
            }
        };

        match self.imp().html_content.set(content) {
            Ok(_) => {}
            Err(e) => {
                error!("failed to set content: {e}");
            }
        }
    }

    fn setup_view(&self) {
        let imp = self.imp();
        let webview = WebView::new();
        webview.set_hexpand(true);
        webview.set_vexpand(true);
        let obox = imp.obox.get();
        obox.append(&webview);
        imp.webview.set(webview).unwrap();

        imp.cur_zoom.set(1.0);
    }

    fn execute_copy_result(&self) {}

    fn zoom_in(&self) {
        let imp = self.imp();
        let val = imp.cur_zoom.get();
        if val < 5.0 {
            let val = val + 0.2;
            imp.cur_zoom.set(val);

            let webview = imp.webview.get().unwrap();
            webview.set_zoom_level(val);
        }
    }

    fn zoom_out(&self) {
        let imp = self.imp();
        let val = imp.cur_zoom.get();
        if val > 0.2 {
            let val = val - 0.2;
            imp.cur_zoom.set(val);

            let webview = imp.webview.get().unwrap();
            webview.set_zoom_level(val);
        }
    }

    fn execute_transform(&self) {
        let imp = self.imp();
        let ibuffer = self.imp().in_view.get().buffer();
        let content = ibuffer.text(&ibuffer.bounds().0, &ibuffer.bounds().1, false);
        let content = content.as_str();

        if content.len() <= 1 {
            return;
        }

        let html_content = imp
            .html_content
            .get()
            .unwrap()
            .replace("@@ASCIIBOX-NOBODYGX-PLACEHOLD@@", content);

        imp.webview.get().unwrap().load_html(&html_content, None);
    }

    fn execute_clear(&self) {
        let ibuffer: gtk::TextBuffer = self.imp().in_view.get().buffer();
        ibuffer.set_text("");
    }

    fn setup_config(&self) {}
}

impl Default for MermaidPage {
    fn default() -> Self {
        Self::new()
    }
}
