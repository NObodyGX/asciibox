use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::gdk;
use gtk::gio;
use gtk::glib;
use gtk::prelude::{TextBufferExt, TextViewExt};
use gtk::CompositeTemplate;
use std::fs::OpenOptions;
use std::io::Write;
use svgbob::to_svg;

use crate::core::flowchart::AMap;

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
        #[template_child]
        pub out_image: TemplateChild<gtk::Image>,

        pub icon_str_backup: RefCell<String>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for FlowchartPage {
        const NAME: &'static str = "FlowchartPage";
        type Type = super::FlowchartPage;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
            load_css();

            klass.install_action("flowchart.do_transform", None, move |obj, _, _| {
                obj.do_transform();
            });
            klass.install_action("flowchart.do_clear", None, move |obj, _, _| {
                obj.do_clear();
            });

            klass.install_action("flowchart.do_svg_copy", None, move |obj, _, _| {
                obj.do_copy_svg_file();
            });

            klass.install_action_async(
                "flowchart.do_svg_save",
                None,
                |win, _action_name, _action_target| async move {
                    if let Err(error) = win.do_save_svg_file().await {
                        println!("Error Save svg file: {error}");
                    };
                },
            );

            klass.install_action("flowchart.do_transform_copy", None, move |obj, _, _| {
                obj.do_transform_copy();
            });

            klass.install_action("flowchart.do_transform_to_svg", None, move |obj, _, _| {
                obj.do_transform_to_svg();
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
    impl WindowImpl for FlowchartPage {}
    impl AdwWindowImpl for FlowchartPage {}
    impl BoxImpl for FlowchartPage {}

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
        @extends gtk::Widget, gtk::Window, adw::Window, gtk::Box,
        @implements gtk::Accessible, gtk::Buildable,gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
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

    fn do_transform(&self) {
        let ibuffer: gtk::TextBuffer = self.imp().in_view.get().buffer();
        let content = ibuffer.text(&ibuffer.bounds().0, &ibuffer.bounds().1, false);

        // 当输入为 0 的时候不覆盖，这样可以编辑 svgbob 窗口并转换
        if content.len() != 0 {
            let expand_mode = false;
            let mut mmap: AMap = AMap::new(expand_mode);
            let otext: String = mmap.load_content(content.as_str());

            let obuffer = self.imp().out_view.get().buffer();
            obuffer.set_text(otext.as_str());
        }

        self.do_transform_to_svg();
    }

    fn do_clear(&self) {
        let ibuffer: gtk::TextBuffer = self.imp().in_view.get().buffer();
        ibuffer.set_text("");
    }

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
        let buffer = self.imp().out_view.get().buffer();
        let content = buffer.text(&buffer.bounds().0, &buffer.bounds().1, false);
        let svg_content = to_svg(content.as_str());

        let texture: gdk::Texture =
            gdk::Texture::from_bytes(&glib::Bytes::from(svg_content.as_bytes()))
                .expect("load svgbob out svg error");
        self.imp().out_image.get().set_paintable(Some(&texture));

        self.imp().icon_str_backup.replace(svg_content);
    }
}

fn load_css() {
    // Load the CSS file and add it to the provider
    // let provider = CssProvider::new();
    // provider.load_from_resource("/com/gitee/gmg137/NeteaseCloudMusicGtk4/themes/discover.css");

    // // Add the provider to the default screen
    // style_context_add_provider_for_display(
    //     &gdk::Display::default().expect("Could not connect to a display."),
    //     &provider,
    //     gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    // );
}
