// Copyright 2024 the Cartero authors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.
//
// SPDX-License-Identifier: GPL-3.0-or-later

use std::path::PathBuf;

use adw::prelude::*;
use adw::AboutWindow;
use gettextrs::gettext;
use glib::subclass::types::ObjectSubclassIsExt;
use glib::Object;
use gtk::gio::{self, ActionEntryBuilder, Settings};
use gtk::prelude::ActionMapExtManual;

use crate::config::{self, APP_ID, BASE_ID, RESOURCE_PATH};
use crate::win::CarteroWindow;

mod imp {
    use std::cell::OnceCell;

    use adw::prelude::*;
    use adw::subclass::application::AdwApplicationImpl;
    use glib::subclass::{object::ObjectImpl, types::ObjectSubclass};
    use gtk::gio::Settings;
    use gtk::subclass::prelude::*;
    use gtk::subclass::{application::GtkApplicationImpl, prelude::ApplicationImpl};

    use super::*;

    #[derive(Default)]
    pub struct CarteroApplication {
        pub(super) window: OnceCell<CarteroWindow>,

        pub(super) settings: OnceCell<Settings>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for CarteroApplication {
        const NAME: &'static str = "CarteroApplication";
        type Type = super::CarteroApplication;
        type ParentType = adw::Application;
    }

    impl ObjectImpl for CarteroApplication {}

    impl ApplicationImpl for CarteroApplication {
        fn activate(&self) {
            self.parent_activate();
            self.obj().get_window().present();
        }

        fn startup(&self) {
            self.parent_startup();
            gtk::Window::set_default_icon_name(APP_ID);

            let obj = self.obj();
            obj.set_accels_for_action("win.new", &["<Primary>t"]);
            obj.set_accels_for_action("win.open", &["<Primary>o"]);
            obj.set_accels_for_action("win.save", &["<Primary>s"]);
            obj.set_accels_for_action("win.close", &["<Primary>w"]);
            obj.set_accels_for_action("win.request", &["<Primary>Return"]);
            obj.set_accels_for_action("app.quit", &["<Primary>q"]);
            obj.setup_app_actions();
        }
    }

    impl GtkApplicationImpl for CarteroApplication {}

    impl AdwApplicationImpl for CarteroApplication {}
}

glib::wrapper! {
    pub struct CarteroApplication(ObjectSubclass<imp::CarteroApplication>)
        @extends gio::Application, gtk::Application, adw::Application,
        @implements gio::ActionMap, gio::ActionGroup;

}

impl Default for CarteroApplication {
    fn default() -> Self {
        Self::new()
    }
}

impl CarteroApplication {
    pub fn get() -> Self {
        gio::Application::default()
            .and_downcast::<CarteroApplication>()
            .unwrap()
    }

    pub fn new() -> Self {
        Object::builder()
            .property("application-id", APP_ID)
            .property("resource-base-path", RESOURCE_PATH)
            .build()
    }

    pub fn get_window(&self) -> &CarteroWindow {
        let imp = self.imp();
        imp.window.get_or_init(|| {
            let settings = self.settings();
            let open_files = settings.get::<Vec<String>>("open-files");

            let win = CarteroWindow::new(self);
            if open_files.is_empty() {
                win.add_endpoint(None);
            } else {
                for open_file in open_files {
                    let typed = open_file.split_once(':');
                    if let Some((_type, path)) = typed {
                        let path = PathBuf::from(path);
                        win.add_endpoint(Some(&path));
                    }
                }
            }
            win
        })
    }

    pub fn settings(&self) -> &Settings {
        self.imp().settings.get_or_init(|| Settings::new(BASE_ID))
    }

    fn setup_app_actions(&self) {
        let about = ActionEntryBuilder::new("about")
            .activate(|app: &CarteroApplication, _, _| {
                let win = app.get_window();
                let about = AboutWindow::builder()
                    .transient_for(win)
                    .modal(true)
                    .application_name("Cartero")
                    .application_icon(config::APP_ID)
                    .version(config::VERSION)
                    .website("https://github.com/danirod/cartero")
                    .issue_url("https://github.com/danirod/cartero/issues")
                    .support_url("https://github.com/danirod/cartero/discussions")
                    .developer_name(gettext("The Cartero authors"))
                    .copyright(gettext("© 2024 the Cartero authors"))
                    .license_type(gtk::License::Gpl30)
                    .build();
                about.present();
            })
            .build();

        let quit = ActionEntryBuilder::new("quit")
            .activate(glib::clone!(@weak self as app => move |_, _, _| {
                app.get_window().close();
            }))
            .build();

        self.add_action_entries([about, quit]);
    }
}
