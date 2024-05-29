use std::fs::OpenOptions;
use std::io::Write;
use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::gdk;
use gtk::gio;
use gtk::glib;
use gtk::prelude::{TextBufferExt, TextViewExt};
use gtk::CompositeTemplate;
use svgbob::to_svg;

use crate::core::svgbob::GSMap;

mod imp {

    use std::cell::RefCell;

    use super::*;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/com/github/nobodygx/asciibox/ui/page_svgbob.ui")]
    pub struct SvgbobPage {
        #[template_child]
        pub in_view: TemplateChild<gtk::TextView>,
        #[template_child]
        pub out_view: TemplateChild<gtk::TextView>,
        #[template_child]
        pub out_image: TemplateChild<gtk::Image>,
        #[template_child]
        pub run_button: TemplateChild<gtk::Button>,

        pub icon_str_backup: RefCell<String>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for SvgbobPage {
        const NAME: &'static str = "SvgbobPage";
        type Type = super::SvgbobPage;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
            load_css();

            klass.install_action("svgbob.do_transform", None, move |obj, _, _| {
                obj.do_transform();
            });

            klass.install_action_async(
                "svgbob.save_svg",
                None,
                |win, _action_name, _action_target| async move {
                    if let Err(error) = win.save_svg_file().await {
                        println!("Error Save svg file: {error}");
                    };
                },
            );
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for SvgbobPage {
        fn constructed(&self) {
            self.parent_constructed();

            let obj = self.obj();
            obj.setup_text_view();
        }
    }
    impl WidgetImpl for SvgbobPage {}
    impl WindowImpl for SvgbobPage {}
    impl AdwWindowImpl for SvgbobPage {}
    impl BoxImpl for SvgbobPage {}

    #[gtk::template_callbacks]
    impl SvgbobPage {
        #[template_callback]
        fn svgbob_svg_copy(&self) {
            println!("copy svg");
            self.obj().copy_svg_output();
        }
    }
}

glib::wrapper! {
    pub struct SvgbobPage(ObjectSubclass<imp::SvgbobPage>)
        @extends gtk::Widget, gtk::Window, adw::Window, gtk::Box,
        @implements gtk::Accessible, gtk::Buildable,gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Default for SvgbobPage {
    fn default() -> Self {
        Self::new()
    }
}

impl SvgbobPage {
    pub fn new() -> Self {
        let page: SvgbobPage = glib::Object::new();
        page
    }

    pub fn init_page(&self) {
        println!("init page");
    }
}

impl SvgbobPage {
    // 配置默认的 placeholdtext
    fn setup_text_view(&self) {}

    fn do_transform(&self) {
        let iview: gtk::TextView = self.imp().in_view.get();
        let ibuffer: gtk::TextBuffer = iview.buffer();
        let oimage: gtk::Image = self.imp().out_image.get();

        let (istart, iend) = ibuffer.bounds();
        let content = ibuffer.text(&istart, &iend, false);

        let mut mmap: GSMap = GSMap::new();
        let otext: String = mmap.load_content(content.as_str());

        let out_view: gtk::TextView = self.imp().out_view.get();
        let obuffer = out_view.buffer();
        obuffer.set_text(otext.as_str());

        let oimage_str = to_svg(otext.as_str());

        // svg_backup = oimage_str.clone();
        let texture: gdk::Texture =
            gdk::Texture::from_bytes(&glib::Bytes::from(oimage_str.as_bytes()))
                .expect("load svgbob out svg error");
        oimage.set_from_paintable(Some(&texture));

        self.imp().icon_str_backup.replace(oimage_str);
    }

    fn copy_svg_output(&self) {
        let clipboard = self.clipboard();
        clipboard.set_text(self.imp().icon_str_backup.borrow().as_str());
    }

    pub async fn save_svg_file(&self) -> Result<(), Box<dyn std::error::Error>> {
        let dialog = gtk::FileDialog::builder()
            .title("Open File")
            .accept_label("Save")
            .modal(true)
            .build();

        let window = self.root().and_downcast::<gtk::Window>().unwrap();
        let file: gio::File = dialog.save_future(Some(&window)).await?;
        let filename = file.path().expect("Couldn't get file path");
        let mut file2: std::fs::File =
            OpenOptions::new().write(true).create(true).open(filename)?;
        file2.write_all(self.imp().icon_str_backup.borrow().as_bytes())?;
        Ok(())
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
