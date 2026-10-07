mod config;
mod doc_writer;

use config::AppConfig;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Box as GtkBox, Button, FileChooserAction, FileChooserDialog, Label, Orientation, ResponseType, TextView};
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

const APP_ID: &str = "dev.obsidianagent.app";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    let mut config = AppConfig::load();

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Obsidian Agent")
        .default_width(1200)
        .default_height(800)
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

    let sidebar = build_sidebar();
    let center = build_center_pane(vault_path.clone());
    let graph = build_graph_pane();

    root.append(&sidebar);
    root.append(&center);
    root.append(&graph);

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
    let sidebar = GtkBox::new(Orientation::Vertical, 4);
    sidebar.set_width_request(220);
    sidebar.add_css_class("sidebar");

    for label in ["New Chat", "Search", "Notes", "Tools", "Settings"] {
        let btn = Button::with_label(label);
        btn.add_css_class("flat");
        sidebar.append(&btn);
    }

    sidebar
}

fn build_center_pane(vault_path: PathBuf) -> GtkBox {
    let center = GtkBox::new(Orientation::Vertical, 8);
    center.set_hexpand(true);

    let header = Label::new(Some(&format!("Vault: {}", vault_path.display())));
    header.set_halign(gtk::Align::Start);

    let messages = TextView::new();
    messages.set_editable(false);
    messages.set_vexpand(true);

    let input_row = GtkBox::new(Orientation::Horizontal, 4);
    let input = gtk::Entry::new();
    input.set_hexpand(true);
    input.set_placeholder_text(Some("Ask the agent..."));
    let send_btn = Button::with_label("Send");

    let messages_ref = Rc::new(RefCell::new(messages.clone()));
    let input_clone = input.clone();
    send_btn.connect_clicked(move |_| {
        let text = input_clone.text().to_string();
        if text.is_empty() {
            return;
        }
        let buf = messages_ref.borrow().buffer();
        let mut end = buf.end_iter();
        buf.insert(&mut end, &format!("You: {text}\n"));
        input_clone.set_text("");
    });

    input_row.append(&input);
    input_row.append(&send_btn);

    center.append(&header);
    center.append(&messages);
    center.append(&input_row);

    center
}

fn build_graph_pane() -> GtkBox {
    let graph = GtkBox::new(Orientation::Vertical, 4);
    graph.set_width_request(400);

    let label = Label::new(Some("Graph view — renders from vault links (Phase 2)"));
    graph.append(&label);

    let placeholder = gtk::DrawingArea::new();
    placeholder.set_vexpand(true);
    placeholder.set_draw_func(|_area, cr, width, height| {
        cr.set_source_rgb(0.07, 0.07, 0.1);
        let _ = cr.paint();
        let _ = (width, height);
    });
    graph.append(&placeholder);

    graph
}
