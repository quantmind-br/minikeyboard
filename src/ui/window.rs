//! Main `adw::ApplicationWindow` with protocol rail, geometry grid, and editor.

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use adw::prelude::*;

use gtk::{gio, glib};

use super::action_editor::ActionEditor;
use super::status::{self, RailTone};
use crate::app::{AppController, SharedController};
use crate::device::session::SessionState;


pub struct MiniKeyboardWindow {
    pub window: adw::ApplicationWindow,
}

impl MiniKeyboardWindow {
    pub fn new(app: &adw::Application) -> Self {
        let controller: SharedController = Rc::new(RefCell::new(AppController::new()));

        let window = adw::ApplicationWindow::new(app);
        window.set_title(Some("Mini Keyboard"));
        window.set_default_size(960, 640);
        window.set_size_request(760, 520);

        // Load CSS
        let provider = gtk::CssProvider::new();
        provider.load_from_resource("/br/com/quantmind/MiniKeyboard/style.css");
        gtk::style_context_add_provider_for_display(
            &gtk::gdk::Display::default().expect("display"),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        let toolbar = adw::ToolbarView::new();
        let header = adw::HeaderBar::new();
        let title_box = gtk::Box::new(gtk::Orientation::Horizontal, 8);
        title_box.append(&gtk::Label::new(Some("Mini Keyboard")));
        let badge = gtk::Label::new(Some("Disconnected"));
        badge.add_css_class("connection-badge");
        title_box.append(&badge);
        header.set_title_widget(Some(&title_box));

        let refresh_btn = gtk::Button::from_icon_name("view-refresh-symbolic");
        refresh_btn.set_tooltip_text(Some("Refresh devices"));
        header.pack_start(&refresh_btn);

        let read_btn = gtk::Button::with_label("Read");
        read_btn.set_tooltip_text(Some("Read from device"));
        let write_btn = gtk::Button::with_label("Write");
        write_btn.set_tooltip_text(Some("Write changes"));
        write_btn.add_css_class("suggested-action");
        let revert_btn = gtk::Button::with_label("Revert");
        revert_btn.set_tooltip_text(Some("Revert changes"));
        header.pack_end(&write_btn);
        header.pack_end(&read_btn);
        header.pack_end(&revert_btn);
        toolbar.add_top_bar(&header);

        let command_bar = gtk::Box::new(gtk::Orientation::Horizontal, 6);
        command_bar.add_css_class("command-bar");
        let device_dropdown = gtk::DropDown::from_strings(&[]);
        device_dropdown.set_hexpand(true);
        device_dropdown.set_tooltip_text(Some("Select device"));
        let import_btn = gtk::Button::with_label("Import profile");
        let export_btn = gtk::Button::with_label("Export profile");
        let diag_btn = gtk::Button::with_label("Diagnostics");
        diag_btn.set_tooltip_text(Some("Export diagnostics"));
        command_bar.append(&device_dropdown);
        command_bar.append(&import_btn);
        command_bar.append(&export_btn);
        command_bar.append(&diag_btn);
        toolbar.add_top_bar(&command_bar);

        // Protocol rail
        let rail = gtk::Box::new(gtk::Orientation::Horizontal, 4);
        rail.add_css_class("protocol-rail");
        rail.set_halign(gtk::Align::Fill);
        toolbar.add_top_bar(&rail);

        // Main split
        let split = adw::OverlaySplitView::new();
        split.set_sidebar_position(gtk::PackType::End);
        split.set_show_sidebar(true);
        // Action editor requests 260px content plus 24px horizontal margins.
        split.set_min_sidebar_width(284.0);
        split.set_max_sidebar_width(360.0);

        let content = gtk::Box::new(gtk::Orientation::Vertical, 0);

        // Layer switcher
        let layer_box = gtk::Box::new(gtk::Orientation::Horizontal, 6);
        layer_box.set_margin_start(12);
        layer_box.set_margin_end(12);
        layer_box.set_margin_top(12);
        layer_box.set_halign(gtk::Align::Center);
        let layer_group = gtk::ToggleButton::new();
        let mut layer_buttons = Vec::new();
        for i in 0..3 {
            let btn = if i == 0 {
                let b = gtk::ToggleButton::with_label(&format!("Layer {}", i + 1));
                b.set_active(true);
                b
            } else {
                let b = gtk::ToggleButton::with_label(&format!("Layer {}", i + 1));
                b.set_group(Some(&layer_group));
                b
            };
            if i == 0 {
                // first is the group leader
            }
            layer_box.append(&btn);
            layer_buttons.push(btn);
        }
        // Fix grouping: make button 0 the group, others join it.
        for (i, btn) in layer_buttons.iter().enumerate() {
            if i > 0 {
                btn.set_group(Some(&layer_buttons[0]));
            }
        }
        let _ = layer_group;

        let key_grid = gtk::Grid::new();
        key_grid.add_css_class("key-grid");
        key_grid.set_row_spacing(8);
        key_grid.set_column_spacing(8);
        key_grid.set_halign(gtk::Align::Center);
        key_grid.set_valign(gtk::Align::Center);
        key_grid.set_vexpand(true);

        let empty_label = gtk::Label::new(Some(status::empty_state_message()));
        empty_label.add_css_class("empty-state");
        empty_label.add_css_class("title-3");
        empty_label.set_vexpand(true);

        let content_stack = gtk::Stack::new();
        content_stack.add_named(&empty_label, Some("empty"));
        let grid_wrap = gtk::Box::new(gtk::Orientation::Vertical, 0);
        grid_wrap.append(&layer_box);
        let scroll = gtk::ScrolledWindow::new();
        scroll.set_child(Some(&key_grid));
        scroll.set_vexpand(true);
        scroll.set_hexpand(true);
        scroll.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
        scroll.set_propagate_natural_width(false);
        grid_wrap.append(&scroll);
        content_stack.add_named(&grid_wrap, Some("grid"));
        content_stack.set_visible_child_name("empty");
        content.append(&content_stack);

        let status_bar = gtk::Label::new(None);
        status_bar.add_css_class("status-bar");
        status_bar.set_wrap(true);
        status_bar.set_halign(gtk::Align::Start);
        status_bar.set_xalign(0.0);
        content.append(&status_bar);

        split.set_content(Some(&content));

        let editor = ActionEditor::new(controller.clone());
        split.set_sidebar(Some(&editor.root));

        toolbar.set_content(Some(&split));
        window.set_content(Some(&toolbar));

        // Wire buttons
        {
            let c = controller.clone();
            refresh_btn.connect_clicked(move |_| {
                c.borrow().refresh();
            });
        }
        {
            let c = controller.clone();
            read_btn.connect_clicked(move |_| {
                c.borrow_mut().read_from_device();
            });
        }
        {
            let c = controller.clone();
            let window = window.clone();
            write_btn.connect_clicked(move |_| {
                let dirty = c.borrow().model.dirty_count();
                let target = c
                    .borrow()
                    .model
                    .selected_device
                    .as_ref()
                    .map(|d| d.label())
                    .unwrap_or_else(|| "unknown".into());
                let dialog = adw::AlertDialog::new(
                    Some("Write changes?"),
                    Some(&format!(
                        "Write {dirty} dirty position(s) to {target}?\n\nThis modifies device firmware storage."
                    )),
                );
                dialog.add_response("cancel", "Cancel");
                dialog.add_response("write", "Write");
                dialog.set_response_appearance("write", adw::ResponseAppearance::Destructive);
                dialog.set_default_response(Some("cancel"));
                dialog.set_close_response("cancel");
                let c2 = c.clone();
                dialog.connect_response(None, move |_, response| {
                    if response == "write" {
                        c2.borrow_mut().write_changes();
                    }
                });
                dialog.present(Some(&window));
            });
        }
        {
            let c = controller.clone();
            let window = window.clone();
            revert_btn.connect_clicked(move |_| {
                let dirty = c.borrow().model.dirty_count();
                if dirty == 0 {
                    return;
                }
                let dialog = adw::AlertDialog::new(
                    Some("Revert changes?"),
                    Some(&format!("Discard {dirty} dirty position(s)?")),
                );
                dialog.add_response("cancel", "Cancel");
                dialog.add_response("revert", "Revert");
                dialog.set_response_appearance("revert", adw::ResponseAppearance::Destructive);
                dialog.set_default_response(Some("cancel"));
                dialog.set_close_response("cancel");
                let c2 = c.clone();
                dialog.connect_response(None, move |_, response| {
                    if response == "revert" {
                        c2.borrow_mut().revert_changes();
                    }
                });
                dialog.present(Some(&window));
            });
        }
        {
            let c = controller.clone();
            let window = window.clone();
            export_btn.connect_clicked(move |_| {
                let dialog = gtk::FileDialog::builder()
                    .title("Export profile")
                    .initial_name("profile.json")
                    .build();
                let filter = gtk::FileFilter::new();
                filter.add_pattern("*.json");
                filter.set_name(Some("JSON profiles"));
                let filters = gio::ListStore::new::<gtk::FileFilter>();
                filters.append(&filter);
                dialog.set_filters(Some(&filters));
                let c2 = c.clone();
                dialog.save(
                    Some(&window),
                    gio::Cancellable::NONE,
                    move |result| {
                        if let Ok(file) = result
                            && let Some(path) = file.path()
                        {
                            let mut ctrl = c2.borrow_mut();
                            match ctrl.export_profile(path, false) {
                                Ok(()) => ctrl.model.status = "Profile exported.".into(),
                                Err(e) => ctrl.model.status = e.to_string(),
                            }
                        }
                    },
                );
            });
        }
        {
            let c = controller.clone();
            let window = window.clone();
            import_btn.connect_clicked(move |_| {
                let dialog = gtk::FileDialog::builder().title("Import profile").build();
                let filter = gtk::FileFilter::new();
                filter.add_pattern("*.json");
                filter.set_name(Some("JSON profiles"));
                let filters = gio::ListStore::new::<gtk::FileFilter>();
                filters.append(&filter);
                dialog.set_filters(Some(&filters));
                let c2 = c.clone();
                dialog.open(
                    Some(&window),
                    gio::Cancellable::NONE,
                    move |result| {
                        if let Ok(file) = result
                            && let Some(path) = file.path()
                        {
                            let mut ctrl = c2.borrow_mut();
                            if let Err(e) = ctrl.import_profile(path) {
                                ctrl.model.status = e.to_string();
                            }
                        }
                    },
                );
            });
        }
        {
            let c = controller.clone();
            let window = window.clone();
            diag_btn.connect_clicked(move |_| {
                let dialog = gtk::FileDialog::builder()
                    .title("Export diagnostics")
                    .initial_name("minikeyboard-diagnostics.json")
                    .build();
                let c2 = c.clone();
                dialog.save(
                    Some(&window),
                    gio::Cancellable::NONE,
                    move |result| {
                        if let Ok(file) = result
                            && let Some(path) = file.path()
                        {
                            let mut ctrl = c2.borrow_mut();
                            match ctrl.export_diagnostics(path) {
                                Ok(()) => {
                                    ctrl.model.status = "Diagnostics exported.".into();
                                }
                                Err(e) => ctrl.model.status = e.to_string(),
                            }
                        }
                    },
                );
            });
        }

        for (i, btn) in layer_buttons.iter().enumerate() {
            let c = controller.clone();
            btn.connect_toggled(move |b| {
                if b.is_active() {
                    c.borrow_mut().select_layer(i);
                }
            });
        }

        // Close with dirty prompt
        {
            let c = controller.clone();
            window.connect_close_request(move |win| {
                let dirty = c.borrow().model.dirty_count();
                if dirty > 0 {
                    let dialog = adw::AlertDialog::new(
                        Some("Unsaved changes"),
                        Some(&format!(
                            "You have {dirty} dirty position(s). Close anyway?"
                        )),
                    );
                    dialog.add_response("cancel", "Cancel");
                    dialog.add_response("close", "Close");
                    dialog.set_response_appearance("close", adw::ResponseAppearance::Destructive);
                    dialog.set_default_response(Some("cancel"));
                    dialog.set_close_response("cancel");
                    let win_for_destroy = win.clone();
                    dialog.connect_response(None, move |_, response| {
                        if response == "close" {
                            win_for_destroy.destroy();
                        }
                    });
                    dialog.present(Some(win));
                    return glib::Propagation::Stop;
                }
                glib::Propagation::Proceed
            });
        }

        // Shared widget handles for render
        let ui = Rc::new(RenderUi {
            device_dropdown: device_dropdown.clone(),
            badge: badge.clone(),
            rail: rail.clone(),
            key_grid: key_grid.clone(),
            content_stack: content_stack.clone(),
            status_bar: status_bar.clone(),
            write_btn: write_btn.clone(),
            read_btn: read_btn.clone(),
            revert_btn: revert_btn.clone(),
            layer_buttons: layer_buttons.clone(),
            editor,
            key_buttons: RefCell::new(Vec::new()),
            rendering: Cell::new(false),
            device_labels: RefCell::new(Vec::new()),
            last_fingerprint: RefCell::new(String::new()),
        });

        // Device dropdown selection (wired after RenderUi so programmatic
        // model/selection updates during render() can be ignored).
        {
            let c = controller.clone();
            let ui = ui.clone();
            device_dropdown.connect_notify_local(Some("selected"), move |dd, _| {
                if ui.rendering.get() {
                    return;
                }
                let idx = dd.selected() as usize;
                let path = {
                    let model = &c.borrow().model;
                    model.devices.get(idx).map(|d| d.path.clone())
                };
                if let Some(path) = path {
                    let mut ctrl = c.borrow_mut();
                    let already = ctrl
                        .model
                        .selected_device
                        .as_ref()
                        .is_some_and(|d| d.path == path);
                    if !already || ctrl.model.session_state == SessionState::Disconnected {
                        ctrl.select_device(&path);
                        ctrl.connect_selected();
                    }
                }
            });
        }

        // Initial render
        render(&controller, &ui);

        // Worker event loop on main context
        if let Some(rx) = controller.borrow().event_receiver() {
            let controller = controller.clone();
            let ui = ui.clone();
            glib::MainContext::default().spawn_local(async move {
                while let Ok(ev) = rx.recv().await {
                    let is_devices =
                        matches!(ev, crate::device::worker::WorkerEvent::Devices(_));
                    controller.borrow_mut().handle_worker_event(ev);
                    if is_devices {
                        // Connect automatically once a device is available.
                        let mut ctrl = controller.borrow_mut();
                        if ctrl.model.session_state == SessionState::Disconnected
                            && ctrl.model.selected_device.is_some()
                        {
                            ctrl.connect_selected();
                        }
                    }
                    render(&controller, &ui);
                }
            });
        }

        // Periodic re-render for mock / status updates from callbacks
        {
            let controller = controller.clone();
            let ui = ui.clone();
            glib::timeout_add_local(std::time::Duration::from_millis(200), move || {
                render(&controller, &ui);
                glib::ControlFlow::Continue
            });
        }

        // Auto-connect first device in non-mock mode after short delay
        if controller.borrow().model.mock_scenario.is_none() {
            let c = controller.clone();
            glib::timeout_add_local_once(std::time::Duration::from_millis(300), move || {
                c.borrow().refresh();
            });
        }

        Self { window }
    }
}

struct RenderUi {
    device_dropdown: gtk::DropDown,
    badge: gtk::Label,
    rail: gtk::Box,
    key_grid: gtk::Grid,
    content_stack: gtk::Stack,
    status_bar: gtk::Label,
    write_btn: gtk::Button,
    read_btn: gtk::Button,
    revert_btn: gtk::Button,
    layer_buttons: Vec<gtk::ToggleButton>,
    editor: ActionEditor,
    key_buttons: RefCell<Vec<gtk::ToggleButton>>,
    rendering: Cell<bool>,
    device_labels: RefCell<Vec<String>>,
    last_fingerprint: RefCell<String>,
}

fn render(controller: &SharedController, ui: &Rc<RenderUi>) {
    let model = controller.borrow().model.clone();

    // Skip when nothing changed; a no-op rebuild every 200 ms plus on every
    // worker event keeps the main loop saturated and the window unresponsive.
    let fingerprint = format!("{model:?}");
    if *ui.last_fingerprint.borrow() == fingerprint {
        return;
    }
    *ui.last_fingerprint.borrow_mut() = fingerprint;

    // Widget updates below fire notify/toggled handlers; mark them programmatic.
    ui.rendering.set(true);

    // Devices
    let labels: Vec<String> = model
        .devices
        .iter()
        .map(|d| {
            let tag = match d.support {
                crate::domain::config::SupportLevel::Experimental => " [experimental]",
                crate::domain::config::SupportLevel::Unknown => " [unknown]",
                crate::domain::config::SupportLevel::Validated => "",
            };
            format!("{}{tag}", d.label())
        })
        .collect();
    if *ui.device_labels.borrow() != labels {
        let strs: Vec<&str> = labels.iter().map(|s| s.as_str()).collect();
        let list = gtk::StringList::new(&strs);
        ui.device_dropdown.set_model(Some(&list));
        *ui.device_labels.borrow_mut() = labels;
    }
    if let Some(sel) = &model.selected_device
        && let Some(idx) = model.devices.iter().position(|d| d.path == sel.path)
        && ui.device_dropdown.selected() != idx as u32
    {
        ui.device_dropdown.set_selected(idx as u32);
    }

    // Badge
    ui.badge.set_text(&status::badge_text(&model));
    ui.badge.set_css_classes(&[]);
    for cls in status::badge_class(&model).split_whitespace() {
        ui.badge.add_css_class(cls);
    }

    // Protocol rail
    while let Some(child) = ui.rail.first_child() {
        ui.rail.remove(&child);
    }
    let steps = status::rail_steps(model.session_state);
    for (i, (label, tone)) in steps.iter().enumerate() {
        if i > 0 {
            let sep = gtk::Label::new(Some("→"));
            sep.add_css_class("protocol-rail-sep");
            ui.rail.append(&sep);
        }
        let step = gtk::Label::new(Some(label));
        for cls in tone.css_class().split_whitespace() {
            step.add_css_class(cls);
        }
        // Highlight current
        let highlight = match model.session_state {
            SessionState::Disconnected => *label == "Disconnected",
            SessionState::Opening | SessionState::Identifying => *label == "Identifying",
            SessionState::ReadOnlyUnknown => *label == "Read-only",
            SessionState::ReadyClean | SessionState::ReadyDirty | SessionState::Reading => {
                *label == "Ready"
            }
            SessionState::Writing => *label == "Writing",
            SessionState::ErrorRecoverable => *label == "Error",
        };
        if highlight && *tone == RailTone::Idle {
            step.add_css_class("active");
        }
        ui.rail.append(&step);
    }

    // Status
    ui.status_bar.set_text(&model.status);

    // Buttons
    ui.write_btn.set_sensitive(model.write_enabled());
    if let Some(reason) = status::write_blocked_reason(&model) {
        ui.write_btn.set_tooltip_text(Some(&reason));
    } else {
        ui.write_btn.set_tooltip_text(Some("Write dirty positions"));
    }
    let can_read = matches!(
        model.session_state,
        SessionState::ReadyClean
            | SessionState::ReadyDirty
            | SessionState::ReadOnlyUnknown
            | SessionState::ErrorRecoverable
    ) && model.selected_device.is_some()
        && model.mock_scenario.is_none();
    ui.read_btn.set_sensitive(can_read);
    ui.revert_btn.set_sensitive(model.dirty_count() > 0);

    // Layers
    for (i, btn) in ui.layer_buttons.iter().enumerate() {
        btn.set_active(i == model.selected_layer);
    }

    // Geometry grid
    while let Some(child) = ui.key_grid.first_child() {
        ui.key_grid.remove(&child);
    }
    ui.key_buttons.borrow_mut().clear();

    if let Some(variant) = &model.variant {
        ui.content_stack.set_visible_child_name("grid");
        let app_bindings =
            crate::profile::bindings::load_bindings().unwrap_or_default();
        let mut group: Option<gtk::ToggleButton> = None;
        for pos in &variant.geometry.positions {
            let assigned = model.config.as_ref().and_then(|cfg| {
                cfg.layers
                    .get(model.selected_layer)
                    .and_then(|l| l.positions.iter().find(|p| p.logical_index == pos.logical_index))
                    .map(|p| {
                        let base = match &p.action {
                            crate::domain::action::Action::Keyboard { modifiers, usage }
                                if crate::profile::bindings::is_marker(*usage) =>
                            {
                                crate::profile::bindings::find(
                                    &app_bindings,
                                    *modifiers,
                                    *usage,
                                )
                                .map(|b| format!("▶ {}", b.name))
                                .unwrap_or_else(|| p.action.short_label())
                            }
                            _ => p.action.short_label(),
                        };
                        if p.dirty { format!("• {base}") } else { base }
                    })
            });
            // Fixed position caption (K1..K12, E1 ◀/⏺/▶) above the assigned
            // action so every physical control stays identifiable per layer.
            let caption = gtk::Label::new(Some(&pos.label));
            caption.add_css_class("key-caption");
            let action_label = gtk::Label::new(Some(assigned.as_deref().unwrap_or("—")));
            action_label.add_css_class("key-action");
            action_label.set_ellipsize(gtk::pango::EllipsizeMode::End);
            action_label.set_max_width_chars(9);
            let key_box = gtk::Box::new(gtk::Orientation::Vertical, 2);
            key_box.set_valign(gtk::Align::Center);
            key_box.append(&caption);
            key_box.append(&action_label);
            let btn = gtk::ToggleButton::new();
            btn.set_child(Some(&key_box));
            btn.add_css_class("key-button");
            if pos.kind == crate::domain::geometry::DisplayKind::Knob {
                btn.add_css_class("knob-button");
            }
            if let Some(action) = &assigned {
                btn.set_tooltip_text(Some(&format!("{} — {}", pos.label, action)));
            } else {
                btn.set_tooltip_text(Some(&pos.label));
            }
            btn.set_accessible_role(gtk::AccessibleRole::Button);
            if let Some(cfg) = &model.config
                && let Some(p) = cfg
                    .layers
                    .get(model.selected_layer)
                    .and_then(|l| l.positions.iter().find(|p| p.logical_index == pos.logical_index))
                && p.dirty
            {
                btn.add_css_class("dirty-key");
            }
            if model.selected_position == Some(pos.logical_index) {
                btn.set_active(true);
            }
            if let Some(g) = &group {
                btn.set_group(Some(g));
            } else {
                group = Some(btn.clone());
            }
            let c = controller.clone();
            let idx = pos.logical_index;
            btn.connect_toggled(move |b| {
                if b.is_active() {
                    c.borrow_mut().select_position(idx);
                }
            });
            ui.key_grid.attach(
                &btn,
                pos.column as i32,
                pos.row as i32,
                pos.col_span as i32,
                1,
            );
            ui.key_buttons.borrow_mut().push(btn);
        }
    } else {
        ui.content_stack.set_visible_child_name("empty");
    }

    ui.editor.render(&model);
    ui.rendering.set(false);
}
