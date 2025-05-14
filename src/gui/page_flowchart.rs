use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::CompositeTemplate;
use gtk::gdk;
use gtk::gio;
use gtk::glib;
use gtk::prelude::{TextBufferExt, TextViewExt};
use std::fs::OpenOptions;
use std::io::Write;
use svgbob::to_svg_string_pretty;

use crate::core::AMap;

mod imp {

    use std::cell::{OnceCell, RefCell};

    use super::*;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/com/github/nobodygx/asciibox/ui/page_flowchart.ui")]
    pub struct FlowchartPage {
        #[template_child]
        pub in_view: TemplateChild<gtk::TextView>,
        #[template_child]
        pub out_view: TemplateChild<gtk::TextView>,

        pub icon_str_backup: RefCell<String>,
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
                    obj.execute_clear_result();
                },
            );
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
            obj.setup_config();
        }
    }
    impl WidgetImpl for FlowchartPage {}
    impl BinImpl for FlowchartPage {}

    #[gtk::template_callbacks]
    impl FlowchartPage {
        #[template_callback]
        fn svgbob_svg_copy(&self) {
            println!("copy svg");
            self.obj().do_copy_svg_file();
        }
    }
}

glib::wrapper! {
    pub struct FlowchartPage(ObjectSubclass<imp::FlowchartPage>)
        @extends gtk::Widget, adw::Bin, @implements gtk::Accessible;
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
        println!("init page");
    }
}

impl FlowchartPage {
    // 配置默认的 placeholdtext
    fn setup_text_view(&self) {}

    fn setup_config(&self) {
        // let config = Config::new();
        // self.imp()
        //     .config
        //     .set(config)
        //     .expect("could not init config");
    }

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

        self.do_transform_to_svg();
    }

    fn execute_clear(&self) {
        let ibuffer: gtk::TextBuffer = self.imp().in_view.get().buffer();
        ibuffer.set_text("");
    }

    fn execute_clear_result(&self) {
        let obuffer = self.imp().out_view.get().buffer();
        obuffer.set_text("");
    }

    fn execute_preview_svgbob(&self) {}

    fn do_copy_svg_file(&self) {
        let clipboard = self.clipboard();
        clipboard.set_text(self.imp().icon_str_backup.borrow().as_str());
    }

    pub async fn do_save_svg_file(&self) -> Result<(), Box<dyn std::error::Error>> {
        let dialog = gtk::FileDialog::builder()
            .title("Open File")
            .accept_label("Save")
            .modal(true)
            .build();

        let window = self.root().and_downcast::<gtk::Window>().unwrap();
        let file: gio::File = dialog.save_future(Some(&window)).await?;
        let mut filename = file.path().expect("Couldn't get file path");
        if !filename.ends_with("svg") {
            filename.set_extension("svg");
        }
        let mut file2: std::fs::File =
            OpenOptions::new().write(true).create(true).open(filename)?;
        file2.write_all(self.imp().icon_str_backup.borrow().as_bytes())?;
        Ok(())
    }

    fn do_transform_copy(&self) {
        let clipboard = self.clipboard();
        let buffer = self.imp().out_view.get().buffer();
        let content = buffer.text(&buffer.bounds().0, &buffer.bounds().1, false);
        clipboard.set_text(content.as_str());
    }

    fn do_transform_to_svg(&self) {
        // let buffer = self.imp().out_view.get().buffer();
        // let content = buffer.text(&buffer.bounds().0, &buffer.bounds().1, false);
        // let svg_content = to_svg_string_pretty(content.as_str());

        // let texture: gdk::Texture =
        //     gdk::Texture::from_bytes(&glib::Bytes::from(svg_content.as_bytes()))
        //         .expect("load svgbob out svg error");
        // self.imp().out_image.get().set_paintable(Some(&texture));

        // self.imp().icon_str_backup.replace(svg_content);
    }
}
