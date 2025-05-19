use crate::core::AMap;
use adw::prelude::*;
use adw::subclass::prelude::*;
use gettextrs::gettext;
use gtk::CompositeTemplate;
use gtk::glib;
use gtk::glib::property::PropertySet;
use gtk::prelude::{TextBufferExt, TextViewExt};
use sourceview;
use std::cell::RefCell;
use std::fs::OpenOptions;
use std::io::Write;
use svgbob::to_svg_string_pretty;

use super::image_preview_dialog::ImagePreviewDialog;

mod imp {

    use super::*;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/com/github/nobodygx/asciibox/ui/page_flowchart.ui")]
    pub struct FlowchartPage {
        #[template_child]
        pub in_view: TemplateChild<sourceview::View>,
        #[template_child]
        pub out_view: TemplateChild<sourceview::View>,

        pub svg_content: RefCell<String>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for FlowchartPage {
        const NAME: &'static str = "FlowchartPage";
        type Type = super::FlowchartPage;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();

            klass.install_action("flowchart.execute-transform", None, move |obj, _, _| {
                obj.execute_transform();
            });
            klass.install_action("flowchart.execute-clear", None, move |obj, _, _| {
                obj.execute_clear();
            });

            klass.install_action("flowchart.execute-clear-result", None, move |obj, _, _| {
                obj.execute_clear_result();
            });

            klass.install_action(
                "flowchart.execute-preview-svgbob",
                None,
                move |obj, _, _| {
                    obj.execute_preview_svgbob();
                },
            );

            klass.install_action_async("flowchart.execute-save", None, |obj, _, _| async move {
                obj.execute_save().await
            });
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for FlowchartPage {
        fn constructed(&self) {
            self.parent_constructed();

            let obj = self.obj();
            obj.setup_text_view();
        }
    }
    impl WidgetImpl for FlowchartPage {}
    impl BinImpl for FlowchartPage {}

    #[gtk::template_callbacks]
    impl FlowchartPage {
        #[template_callback]
        fn svgbob_svg_copy(&self) {
            self.obj().do_copy_svg_file();
        }
    }
}

glib::wrapper! {
    pub struct FlowchartPage(ObjectSubclass<imp::FlowchartPage>)
        @extends gtk::Widget, adw::Bin,
        @implements gtk::Accessible;
}

impl Default for FlowchartPage {
    fn default() -> Self {
        Self::new()
    }
}

impl FlowchartPage {
    pub fn new() -> Self {
        let page: FlowchartPage = glib::Object::new();
        page
    }

    pub fn init_page(&self) {
        log::debug!("init page");
    }
}

impl FlowchartPage {
    // 配置默认的 placeholdtext
    fn setup_text_view(&self) {}

    fn execute_transform(&self) {
        let ibuffer: gtk::TextBuffer = self.imp().in_view.get().buffer();
        let content = ibuffer.text(&ibuffer.bounds().0, &ibuffer.bounds().1, false);

        // 当输入为 0 的时候不覆盖，这样可以编辑 svgbob 窗口并转换
        if content.len() != 0 {
            let mut mmap: AMap = AMap::new(true);
            let otext: String = mmap.load_content(content.as_str());

            let obuffer = self.imp().out_view.get().buffer();
            obuffer.set_text(otext.as_str());
        }
    }

    fn execute_clear(&self) {
        let ibuffer: gtk::TextBuffer = self.imp().in_view.get().buffer();
        ibuffer.set_text("");
    }

    fn execute_clear_result(&self) {
        let obuffer = self.imp().out_view.get().buffer();
        obuffer.set_text("");
    }

    fn execute_preview_svgbob(&self) {
        let buffer = self.imp().out_view.get().buffer();
        let content = buffer.text(&buffer.bounds().0, &buffer.bounds().1, false);
        let svg_content = to_svg_string_pretty(content.as_str());
        self.imp().svg_content.set(svg_content.clone());

        let dialog = ImagePreviewDialog::new();
        dialog.set_svg(svg_content);

        let Some(parent_window) = self.root().and_downcast::<gtk::Window>() else {
            return;
        };
        dialog.present(Some(&parent_window));
    }

    fn do_copy_svg_file(&self) {
        let clipboard = self.clipboard();
        clipboard.set_text(self.imp().svg_content.borrow().as_str());
    }

    async fn execute_save(&self) {
        let dialog = gtk::FileDialog::builder()
            .title(&gettext("Open File"))
            .accept_label("Save")
            .modal(true)
            .build();

        let window = self.root().and_downcast::<gtk::Window>().unwrap();
        let ofile = dialog.save_future(Some(&window)).await;
        if ofile.is_err() {
            log::error!("dialog error in : {ofile:#?}");
            return;
        }
        let ofile = ofile.unwrap();
        let filename = ofile.path();
        if filename.is_none() {
            log::error!("get ofile error");
            return;
        }
        let mut filename = filename.unwrap();
        if !filename.ends_with("svg") {
            filename.set_extension("svg");
        }
        match OpenOptions::new().write(true).create(true).open(&filename) {
            Ok(mut f2) => f2
                .write_all(self.imp().svg_content.borrow().as_bytes())
                .expect("write error"),
            Err(e) => {
                log::error!("create file error in {filename:#?}: {e}")
            }
        }
    }
}
