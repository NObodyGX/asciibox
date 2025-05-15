use adw::subclass::prelude::*;
use gtk;
use gtk::gdk;
use gtk::glib;
use gtk::glib::property::PropertySet;
use std::cell::RefCell;

mod imp {

    use super::*;

    #[derive(Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/github/nobodygx/asciibox/ui/image_preview_dialog.ui")]
    pub struct ImagePreviewDialog {
        #[template_child]
        pub image: TemplateChild<gtk::Image>,

        pub content: RefCell<String>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ImagePreviewDialog {
        const NAME: &'static str = "ImagePreviewDialog";
        type Type = super::ImagePreviewDialog;
        type ParentType = adw::Dialog;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for ImagePreviewDialog {
        fn constructed(&self) {
            self.parent_constructed();
        }
    }

    impl WidgetImpl for ImagePreviewDialog {}

    impl AdwDialogImpl for ImagePreviewDialog {}
}

glib::wrapper! {
    pub struct ImagePreviewDialog(ObjectSubclass<imp::ImagePreviewDialog>)
        @extends adw::Dialog, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl ImagePreviewDialog {
    pub fn new() -> Self {
        let obj: ImagePreviewDialog = glib::Object::new();
        obj
    }

    pub fn set_svg(&self, content: String) {
        let texture: gdk::Texture =
            gdk::Texture::from_bytes(&glib::Bytes::from(content.as_bytes()))
                .expect("load svg error");
        self.imp().image.get().set_paintable(Some(&texture));

        self.imp().content.set(content);
    }
}
