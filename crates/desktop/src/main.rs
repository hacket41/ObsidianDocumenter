mod config;
mod doc_writer;

use config::{AppConfig, APP_ID};
use gtk::prelude::*;
use gtk::{
    glib, Application, ApplicationWindow, Box as GtkBox, Button, CssProvider,
    FileChooserAction, FileChooserDialog, Label, Orientation, ResponseType, TextView,
};
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_startup(|_| load_css());
    app.connect_activate(build_ui);
    app.run()
}

fn load_css() {
    let provider = CssProvider::new();
    // Loaded relative to the crate at compile time so the binary is
    // self-contained; edit style/app.css and rebuild to see changes.
    provider.load_from_string(include_str!("../style/app.css"));

    gtk::style_context_add_provider_for_display(
        &gtk::gdk::Display::default().expect("no display available"),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn build_ui(app: &Application) {
    let mut config = AppConfig::load();

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Obsidian Agent")
        .default_width(1280)
        .default_height(820)
        .build();

    if config.vault_path.is_none() {
        prompt_for_vault_folder(&window, move |chosen| {
            let mut cfg = AppConfig::load();
            cfg.vault_path = Some(chosen);
            let _ = cfg.save();
        });
        config.vault_path = Some(default_vault_path());
    }

    let vault_path = config.vault_path.clone().unwrap_or_else(default_vault_path);

    let root = GtkBox::new(Orientation::Horizontal, 0);
    root.append(&build_sidebar());
    root.append(&build_center_pane(vault_path.clone()));
    root.append(&build_graph_pane());

    window.set_child(Some(&root));
    window.present();

    let vault_for_timer = vault_path.clone();
    glib::timeout_add_seconds_local(180, move || {
        let summary = "Placeholder activity summary — real code-change tracking comes in Phase 2.";
        match doc_writer::write_snapshot(&vault_for_timer, summary) {
            Ok(path) => println!("Wrote snapshot: {:?}", path),
            Err(e) => eprintln!("Failed to write snapshot: {e}"),
        }
        glib::ControlFlow::Continue
    });
}

fn default_vault_path() -> PathBuf {
    dirs::document_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("ObsidianAgentVault")
}

fn prompt_for_vault_folder(window: &ApplicationWindow, on_chosen: impl Fn(PathBuf) + 'static) {
    let dialog = FileChooserDialog::new(
        Some("Choose a folder for your Obsidian Agent vault"),
        Some(window),
        FileChooserAction::SelectFolder,
        &[("Cancel", ResponseType::Cancel), ("Select", ResponseType::Accept)],
    );

    let on_chosen = Rc::new(on_chosen);
    dialog.connect_response(move |dialog, response| {
        if response == ResponseType::Accept {
            if let Some(file) = dialog.file() {
                if let Some(path) = file.path() {
                    on_chosen(path);
                }
            }
        }
        dialog.close();
    });

    dialog.show();
}

fn build_sidebar() -> GtkBox {
    let sidebar = GtkBox::new(Orientation::Vertical, 2);
    sidebar.set_width_request(220);
    sidebar.add_css_class("sidebar");

    let title = Label::new(Some("◤ Obsidian Agent"));
    title.set_halign(gtk::Align::Start);
    title.add_css_class("sidebar-title");
    sidebar.append(&title);

    for label in ["+ New Chat", "Search", "Notes", "Tools", "Vault"] {
        let btn = Button::with_label(label);
        btn.add_css_class("sidebar-btn");
        btn.set_halign(gtk::Align::Fill);
        sidebar.append(&btn);
    }

    let section = Label::new(Some("SETTINGS"));
    section.set_halign(gtk::Align::Start);
    section.add_css_class("sidebar-section-label");
    sidebar.append(&section);

    let settings_btn = Button::with_label("Preferences");
    settings_btn.add_css_class("sidebar-btn");
    sidebar.append(&settings_btn);

    sidebar
}

fn build_center_pane(vault_path: PathBuf) -> GtkBox {
    let center = GtkBox::new(Orientation::Vertical, 0);
    center.set_hexpand(true);
    center.add_css_class("center-pane");

    let header =
