use crate::core::{TableFormator, TableMode};
use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::CompositeTemplate;
use gtk::gdk;
use gtk::glib;
use gtk::prelude::{TextBufferExt, TextViewExt};
use sourceview;

mod imp {

    use super::*;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/com/github/nobodygx/asciibox/ui/page_table.ui")]
    pub struct TablePage {
        #[template_child]
        pub in_view: TemplateChild<sourceview::View>,
        #[template_child]
        pub out_view: TemplateChild<sourceview::View>,
        #[template_child]
        pub table_mode: TemplateChild<gtk::DropDown>,

        pub provider: gtk::CssProvider,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for TablePage {
        const NAME: &'static str = "TablePage";
        type Type = super::TablePage;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();

            klass.install_action("table.execute-transform", None, move |obj, _, _| {
                obj.execute_transform();
            });

            klass.install_action("table.execute-clear", None, move |obj, _, _| {
                obj.execute_clear();
            });

            klass.install_action("table.execute-copy-result", None, move |obj, _, _| {
                obj.execute_copy_result();
            });
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for TablePage {
        fn constructed(&self) {
            let obj = self.obj();
            self.parent_constructed();

            obj.setup_config();
            obj.setup_font_setting();
        }
    }
    impl WidgetImpl for TablePage {}
    impl BinImpl for TablePage {}
}

glib::wrapper! {
    pub struct TablePage(ObjectSubclass<imp::TablePage>)
    @extends gtk::Widget, adw::Bin,
    @implements gtk::Accessible;
}

impl TablePage {
    pub fn new() -> Self {
        let page: TablePage = glib::Object::new();
        page
    }
    fn execute_copy_result(&self) {
        let clipboard = self.clipboard();
        let buffer = self.imp().out_view.get().buffer();
        let content = buffer.text(&buffer.bounds().0, &buffer.bounds().1, false);
        clipboard.set_text(content.as_str());
    }

    fn execute_transform(&self) {
        let ibuffer: gtk::TextBuffer = self.imp().in_view.get().buffer();
        let content = ibuffer.text(&ibuffer.bounds().0, &ibuffer.bounds().1, false);

        // 当输入为 0 的时候不覆盖，这样可以编辑 svgbob 窗口并转换
        if content.len() != 0 {
            let omode = match self.imp().table_mode.get().selected() {
                0 => TableMode::Markdown,
                1 => TableMode::MarkdownGFM,
                2 => TableMode::Asciidoc,
                _ => TableMode::Markdown,
            };
            let cellw = 40;
            let linew = 99;

            let mut formator: TableFormator = TableFormator::new(cellw as usize, linew as usize);
            let otext: String = formator.do_format(content.as_str(), &omode);

            let obuffer = self.imp().out_view.get().buffer();
            obuffer.set_text(otext.as_str());
        }
    }

    fn execute_clear(&self) {
        let ibuffer: gtk::TextBuffer = self.imp().in_view.get().buffer();
        ibuffer.set_text("");
    }

    fn setup_config(&self) {}

    fn setup_font_setting(&self) {
        let imp = self.imp();

        if let Some(display) = gdk::Display::default() {
            gtk::style_context_add_provider_for_display(
                &display,
                &imp.provider,
                gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
        }
        self.refresh_font();
    }

    fn refresh_font(&self) {}
}

impl Default for TablePage {
    fn default() -> Self {
        Self::new()
    }
}
