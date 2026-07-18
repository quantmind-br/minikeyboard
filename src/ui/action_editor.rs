//! Sidebar action editor with capability-gated categories.

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use gtk::prelude::*;

use gtk::{gio, glib};

use super::shortcut_capture;
use crate::app::{AppModel, SharedController};
use crate::profile::bindings::{self, AppBinding};
use crate::domain::action::{
    Action, ActionKind, ConsumerAction, KEY_CHOICES, Modifiers, MouseAction, Stroke,
};
use crate::protocol::codec::verified_action_kinds;

pub struct ActionEditor {
    pub root: gtk::Box,
    stack: gtk::Stack,
    opaque_label: gtk::Label,
    help_label: gtk::Label,
}

impl ActionEditor {
    pub fn new(controller: SharedController) -> Self {
        let root = gtk::Box::new(gtk::Orientation::Vertical, 12);
        root.set_margin_start(12);
        root.set_margin_end(12);
        root.set_margin_top(12);
        root.set_margin_bottom(12);
        root.set_width_request(260);
        root.set_hexpand(false);

        let title = gtk::Label::new(Some("Action editor"));
        title.add_css_class("title-4");
        title.set_halign(gtk::Align::Start);
        root.append(&title);

        let help_label = gtk::Label::new(None);
        help_label.add_css_class("action-category-help");
        help_label.set_wrap(true);
        help_label.set_xalign(0.0);
        help_label.set_max_width_chars(36);
        root.append(&help_label);

        let opaque_label = gtk::Label::new(None);
        opaque_label.set_wrap(true);
        opaque_label.set_xalign(0.0);
        opaque_label.set_max_width_chars(36);
        root.append(&opaque_label);

        let stack = gtk::Stack::new();
        stack.set_vexpand(true);

        // Category chooser
        let categories = gtk::Box::new(gtk::Orientation::Vertical, 6);
        let verified = verified_action_kinds();

        let make_row = |name: &str, kind: ActionKind, child: &gtk::Widget| {
            let sensitive = verified.contains(&kind);
            let frame = gtk::Frame::new(Some(name));
            let box_ = gtk::Box::new(gtk::Orientation::Vertical, 6);
            box_.set_margin_start(8);
            box_.set_margin_end(8);
            box_.set_margin_top(8);
            box_.set_margin_bottom(8);
            if !sensitive {
                let hint = gtk::Label::new(Some(
                    "Protocol mapping not verified for this action.",
                ));
                hint.add_css_class("action-category-help");
                hint.set_wrap(true);
                box_.append(&hint);
            }
            child.set_sensitive(sensitive);
            box_.append(child);
            frame.set_child(Some(&box_));
            frame
        };

        // Keyboard
        let kb_box = gtk::Box::new(gtk::Orientation::Vertical, 6);
        let labels: Vec<&str> = KEY_CHOICES.iter().map(|k| k.label).collect();
        let key_model = gtk::StringList::new(&labels);
        let key_drop = gtk::DropDown::new(Some(key_model), gtk::Expression::NONE);
        let ctrl = gtk::CheckButton::with_label("Ctrl");
        let shift = gtk::CheckButton::with_label("Shift");
        let alt = gtk::CheckButton::with_label("Alt");
        let gui = gtk::CheckButton::with_label("Win");
        let mod_box = gtk::FlowBox::new();
        mod_box.set_selection_mode(gtk::SelectionMode::None);
        mod_box.set_max_children_per_line(2);
        mod_box.insert(&ctrl, -1);
        mod_box.insert(&shift, -1);
        mod_box.insert(&alt, -1);
        mod_box.insert(&gui, -1);
        let apply_kb = gtk::Button::with_label("Apply keyboard");
        {
            let controller = controller.clone();
            let key_drop = key_drop.clone();
            let ctrl = ctrl.clone();
            let shift = shift.clone();
            let alt = alt.clone();
            let gui = gui.clone();
            apply_kb.connect_clicked(move |_| {
                let idx = key_drop.selected() as usize;
                let usage = KEY_CHOICES.get(idx).map(|k| k.usage).unwrap_or(0x04);
                let mut mods = Modifiers::empty();
                if ctrl.is_active() {
                    mods |= Modifiers::CTRL;
                }
                if shift.is_active() {
                    mods |= Modifiers::SHIFT;
                }
                if alt.is_active() {
                    mods |= Modifiers::ALT;
                }
                if gui.is_active() {
                    mods |= Modifiers::GUI;
                }
                let mut c = controller.borrow_mut();
                if let Err(e) = c.apply_action_to_selected(
                    Action::Keyboard {
                        modifiers: mods,
                        usage,
                    },
                    None,
                ) {
                    c.model.status = e.to_string();
                }
            });
        }
        // Shortcut recorder: capture a combo straight from the keyboard.
        let record_btn = gtk::ToggleButton::with_label("Gravar atalho");
        record_btn.set_tooltip_text(Some(
            "Clique e pressione a combinação desejada (Esc cancela)",
        ));
        let record_hint = gtk::Label::new(None);
        record_hint.add_css_class("action-category-help");
        record_hint.set_wrap(true);
        record_hint.set_xalign(0.0);
        record_hint.set_max_width_chars(36);
        // Capture straight from evdev in a worker thread: combos the
        // compositor swallows (Super+…) never reach GTK, but they do reach
        // /dev/input. One shortcut per activation; Esc cancels.
        let recording: Rc<RefCell<Option<Arc<AtomicBool>>>> = Rc::new(RefCell::new(None));
        {
            let controller = controller.clone();
            let hint = record_hint.clone();
            let key_drop = key_drop.clone();
            let (r_ctrl, r_shift, r_alt, r_gui) =
                (ctrl.clone(), shift.clone(), alt.clone(), gui.clone());
            let recording = recording.clone();
            record_btn.connect_toggled(move |b| {
                if !b.is_active() {
                    // Manual untoggle (or completion below): stop the thread.
                    if let Some(flag) = recording.borrow_mut().take() {
                        flag.store(true, Ordering::Relaxed);
                    }
                    return;
                }
                let flag = Arc::new(AtomicBool::new(false));
                *recording.borrow_mut() = Some(flag.clone());
                hint.set_text("Pressione a combinação desejada (Esc cancela)…");
                let rx = shortcut_capture::spawn(flag);
                let b = b.clone();
                let hint = hint.clone();
                let controller = controller.clone();
                let key_drop = key_drop.clone();
                let (r_ctrl, r_shift, r_alt, r_gui) = (
                    r_ctrl.clone(),
                    r_shift.clone(),
                    r_alt.clone(),
                    r_gui.clone(),
                );
                glib::MainContext::default().spawn_local(async move {
                    let result = rx.recv().await.ok().flatten();
                    b.set_active(false);
                    let Some(captured) = result else {
                        hint.set_text(
                            "Nada capturado: tempo esgotado ou sem acesso a /dev/input \
                             (grupo 'input').",
                        );
                        return;
                    };
                    if captured.usage == 0x29 && captured.modifiers.is_empty() {
                        hint.set_text("Gravação cancelada.");
                        return;
                    }
                    let mods = captured.modifiers;
                    r_ctrl.set_active(mods.contains(Modifiers::CTRL));
                    r_shift.set_active(mods.contains(Modifiers::SHIFT));
                    r_alt.set_active(mods.contains(Modifiers::ALT));
                    r_gui.set_active(mods.contains(Modifiers::GUI));
                    if let Some(i) =
                        KEY_CHOICES.iter().position(|k| k.usage == captured.usage)
                    {
                        key_drop.set_selected(i as u32);
                    }
                    let action = Action::Keyboard {
                        modifiers: mods,
                        usage: captured.usage,
                    };
                    hint.set_text(&format!("Gravado: {}", action.short_label()));
                    let mut c = controller.borrow_mut();
                    if let Err(e) = c.apply_action_to_selected(action, None) {
                        c.model.status = e.to_string();
                    }
                });
            });
        }
        kb_box.append(&record_btn);
        kb_box.append(&record_hint);
        kb_box.append(&key_drop);
        kb_box.append(&mod_box);
        kb_box.append(&apply_kb);
        categories.append(&make_row("Keyboard", ActionKind::Keyboard, kb_box.upcast_ref()));

        // Macro (sequence) — up to 20 strokes per record.
        let macro_box = gtk::Box::new(gtk::Orientation::Vertical, 6);
        let macro_strokes: Rc<RefCell<Vec<Stroke>>> = Rc::new(RefCell::new(Vec::new()));
        let macro_list = gtk::Label::new(Some("Macro is empty."));
        macro_list.set_wrap(true);
        macro_list.set_xalign(0.0);
        macro_list.set_max_width_chars(36);
        let macro_key_model =
            gtk::StringList::new(&KEY_CHOICES.iter().map(|k| k.label).collect::<Vec<_>>());
        let macro_key_drop = gtk::DropDown::new(Some(macro_key_model), gtk::Expression::NONE);
        let m_ctrl = gtk::CheckButton::with_label("Ctrl");
        let m_shift = gtk::CheckButton::with_label("Shift");
        let m_alt = gtk::CheckButton::with_label("Alt");
        let m_gui = gtk::CheckButton::with_label("Win");
        let m_mod_box = gtk::FlowBox::new();
        m_mod_box.set_selection_mode(gtk::SelectionMode::None);
        m_mod_box.set_max_children_per_line(2);
        m_mod_box.insert(&m_ctrl, -1);
        m_mod_box.insert(&m_shift, -1);
        m_mod_box.insert(&m_alt, -1);
        m_mod_box.insert(&m_gui, -1);
        let add_stroke = gtk::Button::with_label("Add stroke");
        let clear_macro = gtk::Button::with_label("Clear strokes");
        let apply_macro = gtk::Button::with_label("Apply macro");

        let describe = |strokes: &[Stroke]| -> String {
            if strokes.is_empty() {
                "Macro is empty.".into()
            } else {
                strokes
                    .iter()
                    .map(|s| {
                        Action::Keyboard {
                            modifiers: s.modifiers,
                            usage: s.usage,
                        }
                        .short_label()
                    })
                    .collect::<Vec<_>>()
                    .join(" → ")
            }
        };
        {
            let strokes = macro_strokes.clone();
            let list = macro_list.clone();
            let key_drop = macro_key_drop.clone();
            let (ctrl, shift, alt, gui) =
                (m_ctrl.clone(), m_shift.clone(), m_alt.clone(), m_gui.clone());
            add_stroke.connect_clicked(move |_| {
                if strokes.borrow().len() >= 20 {
                    list.set_text("Macro limit is 20 strokes.");
                    return;
                }
                let idx = key_drop.selected() as usize;
                let usage = KEY_CHOICES.get(idx).map(|k| k.usage).unwrap_or(0x04);
                let mut mods = Modifiers::empty();
                if ctrl.is_active() {
                    mods |= Modifiers::CTRL;
                }
                if shift.is_active() {
                    mods |= Modifiers::SHIFT;
                }
                if alt.is_active() {
                    mods |= Modifiers::ALT;
                }
                if gui.is_active() {
                    mods |= Modifiers::GUI;
                }
                strokes.borrow_mut().push(Stroke {
                    modifiers: mods,
                    usage,
                });
                list.set_text(&describe(&strokes.borrow()));
            });
        }
        {
            let strokes = macro_strokes.clone();
            let list = macro_list.clone();
            clear_macro.connect_clicked(move |_| {
                strokes.borrow_mut().clear();
                list.set_text("Macro is empty.");
            });
        }
        {
            let strokes = macro_strokes.clone();
            let controller = controller.clone();
            apply_macro.connect_clicked(move |_| {
                let strokes = strokes.borrow().clone();
                if strokes.is_empty() {
                    controller.borrow_mut().model.status =
                        "Add at least one stroke before applying a macro.".into();
                    return;
                }
                let mut c = controller.borrow_mut();
                if let Err(e) = c.apply_action_to_selected(Action::Sequence { strokes }, None) {
                    c.model.status = e.to_string();
                }
            });
        }
        // Sequence recorder: each combo pressed becomes one stroke.
        let seq_btn = gtk::ToggleButton::with_label("Gravar sequência");
        seq_btn.set_tooltip_text(Some(
            "Grava cada combinação pressionada como um stroke (Esc encerra)",
        ));
        {
            let seq_recording: Rc<RefCell<Option<Arc<AtomicBool>>>> =
                Rc::new(RefCell::new(None));
            let strokes = macro_strokes.clone();
            let list = macro_list.clone();
            seq_btn.connect_toggled(move |b| {
                if !b.is_active() {
                    if let Some(flag) = seq_recording.borrow_mut().take() {
                        flag.store(true, Ordering::Relaxed);
                    }
                    return;
                }
                let flag = Arc::new(AtomicBool::new(false));
                *seq_recording.borrow_mut() = Some(flag.clone());
                list.set_text("Gravando: pressione as combinações; Esc encerra…");
                let rx = shortcut_capture::spawn_sequence(flag.clone());
                let b = b.clone();
                let strokes = strokes.clone();
                let list = list.clone();
                glib::MainContext::default().spawn_local(async move {
                    while let Ok(c) = rx.recv().await {
                        if c.usage == 0x29 && c.modifiers.is_empty() {
                            break; // bare Esc ends the recording
                        }
                        let mut sk = strokes.borrow_mut();
                        if sk.len() >= 20 {
                            break;
                        }
                        sk.push(Stroke {
                            modifiers: c.modifiers,
                            usage: c.usage,
                        });
                        list.set_text(&describe(&sk));
                    }
                    flag.store(true, Ordering::Relaxed);
                    b.set_active(false);
                    list.set_text(&describe(&strokes.borrow()));
                });
            });
        }
        macro_box.append(&macro_list);
        macro_box.append(&macro_key_drop);
        macro_box.append(&m_mod_box);
        let macro_buttons = gtk::Box::new(gtk::Orientation::Horizontal, 6);
        macro_buttons.append(&add_stroke);
        macro_buttons.append(&clear_macro);
        macro_box.append(&macro_buttons);
        macro_box.append(&seq_btn);
        macro_box.append(&apply_macro);
        categories.append(&make_row("Macro", ActionKind::Sequence, macro_box.upcast_ref()));

        // Applications & scripts: bind a shell command to a marker chord
        // (F13–F24 + modifiers) written to the key; minikeyboard-daemon
        // watches the device and launches the command.
        let app_box = gtk::Box::new(gtk::Orientation::Vertical, 6);
        let app_help = gtk::Label::new(Some(
            "Grava um marcador (F13–F24) na tecla; o daemon executa o comando. \
             Ative: systemctl --user enable --now minikeyboard-daemon",
        ));
        app_help.add_css_class("action-category-help");
        app_help.set_wrap(true);
        app_help.set_xalign(0.0);
        app_help.set_max_width_chars(36);
        let cmd_entry = gtk::Entry::new();
        cmd_entry.set_placeholder_text(Some("Comando (ex.: firefox, ~/scripts/x.sh)"));
        let pick_btn = gtk::Button::with_label("Escolher arquivo…");
        let name_entry = gtk::Entry::new();
        name_entry.set_placeholder_text(Some("Nome exibido (opcional)"));
        let bind_btn = gtk::Button::with_label("Vincular à tecla");
        let unbind_btn = gtk::Button::with_label("Remover vínculo");
        {
            let cmd_entry = cmd_entry.clone();
            pick_btn.connect_clicked(move |b| {
                let dialog = gtk::FileDialog::builder()
                    .title("Escolher aplicação ou script")
                    .build();
                let window = b.root().and_downcast::<gtk::Window>();
                let cmd_entry = cmd_entry.clone();
                dialog.open(window.as_ref(), gio::Cancellable::NONE, move |result| {
                    if let Ok(file) = result
                        && let Some(path) = file.path()
                    {
                        let p = path.to_string_lossy();
                        let quoted = if p.contains(' ') {
                            format!("\"{p}\"")
                        } else {
                            p.into_owned()
                        };
                        cmd_entry.set_text(&quoted);
                    }
                });
            });
        }
        {
            let controller = controller.clone();
            let cmd_entry = cmd_entry.clone();
            let name_entry = name_entry.clone();
            bind_btn.connect_clicked(move |_| {
                let command = cmd_entry.text().trim().to_string();
                if command.is_empty() {
                    controller.borrow_mut().model.status =
                        "Informe o comando ou escolha um arquivo.".into();
                    return;
                }
                let selected = controller
                    .borrow()
                    .model
                    .selected_position_config()
                    .map(|p| p.action.clone());
                if selected.is_none() {
                    controller.borrow_mut().model.status =
                        "Selecione uma tecla antes de vincular.".into();
                    return;
                }
                let mut list = bindings::load_bindings().unwrap_or_default();
                // Reuse the key's current marker when re-binding, otherwise
                // allocate a fresh chord.
                let existing = match selected {
                    Some(Action::Keyboard { modifiers, usage })
                        if bindings::is_marker(usage) =>
                    {
                        Some((usage, modifiers))
                    }
                    _ => None,
                };
                let Some((usage, mods)) =
                    existing.or_else(|| bindings::alloc_marker(&list))
                else {
                    controller.borrow_mut().model.status =
                        "Sem marcadores livres (limite de 192 vínculos).".into();
                    return;
                };
                let name = {
                    let n = name_entry.text().trim().to_string();
                    if n.is_empty() {
                        command
                            .split_whitespace()
                            .next()
                            .and_then(|w| w.rsplit('/').next())
                            .unwrap_or("app")
                            .trim_matches('"')
                            .to_string()
                    } else {
                        n
                    }
                };
                list.retain(|b| !(b.usage == usage && b.modifiers == mods));
                list.push(AppBinding {
                    usage,
                    modifiers: mods,
                    name,
                    command,
                });
                if let Err(e) = bindings::save_bindings(&list) {
                    controller.borrow_mut().model.status = e.to_string();
                    return;
                }
                let mut c = controller.borrow_mut();
                match c.apply_action_to_selected(
                    Action::Keyboard {
                        modifiers: mods,
                        usage,
                    },
                    None,
                ) {
                    Ok(()) => {
                        c.model.status = "Vínculo salvo. Faça Write e ative o daemon: \
                             systemctl --user enable --now minikeyboard-daemon"
                            .into();
                    }
                    Err(e) => c.model.status = e.to_string(),
                }
            });
        }
        {
            let controller = controller.clone();
            unbind_btn.connect_clicked(move |_| {
                let selected = controller
                    .borrow()
                    .model
                    .selected_position_config()
                    .map(|p| p.action.clone());
                let Some(Action::Keyboard { modifiers, usage }) = selected else {
                    controller.borrow_mut().model.status =
                        "A tecla selecionada não tem vínculo de aplicação.".into();
                    return;
                };
                if !bindings::is_marker(usage) {
                    controller.borrow_mut().model.status =
                        "A tecla selecionada não tem vínculo de aplicação.".into();
                    return;
                }
                let mut list = bindings::load_bindings().unwrap_or_default();
                list.retain(|b| !(b.usage == usage && b.modifiers == modifiers));
                if let Err(e) = bindings::save_bindings(&list) {
                    controller.borrow_mut().model.status = e.to_string();
                    return;
                }
                let mut c = controller.borrow_mut();
                if let Err(e) = c.apply_action_to_selected(Action::Empty, None) {
                    c.model.status = e.to_string();
                } else {
                    c.model.status = "Vínculo removido.".into();
                }
            });
        }
        app_box.append(&app_help);
        app_box.append(&cmd_entry);
        app_box.append(&pick_btn);
        app_box.append(&name_entry);
        let app_buttons = gtk::Box::new(gtk::Orientation::Horizontal, 6);
        app_buttons.append(&bind_btn);
        app_buttons.append(&unbind_btn);
        app_box.append(&app_buttons);
        categories.append(&make_row(
            "Aplicações e Scripts",
            ActionKind::Keyboard,
            app_box.upcast_ref(),
        ));

        // Media
        let media_box = gtk::Box::new(gtk::Orientation::Vertical, 4);
        for (label, action) in [
            ("Play/Pause", ConsumerAction::PlayPause),
            ("Stop", ConsumerAction::Stop),
            ("Previous Track", ConsumerAction::PreviousTrack),
            ("Next Track", ConsumerAction::NextTrack),
            ("Mute", ConsumerAction::Mute),
            ("Volume+", ConsumerAction::VolumeUp),
            ("Volume-", ConsumerAction::VolumeDown),
            ("Calculator", ConsumerAction::Calculator),
        ] {
            let btn = gtk::Button::with_label(label);
            let controller = controller.clone();
            btn.connect_clicked(move |_| {
                let mut c = controller.borrow_mut();
                if let Err(e) = c.apply_action_to_selected(Action::Consumer { action }, None) {
                    c.model.status = e.to_string();
                }
            });
            media_box.append(&btn);
        }
        categories.append(&make_row("Media", ActionKind::Consumer, media_box.upcast_ref()));

        // Mouse
        let mouse_box = gtk::Box::new(gtk::Orientation::Vertical, 4);
        for (label, action) in [
            ("Left Click", MouseAction::LeftClick),
            ("Middle Click", MouseAction::MiddleClick),
            ("Right Click", MouseAction::RightClick),
            ("Wheel Up", MouseAction::WheelUp),
            ("Wheel Down", MouseAction::WheelDown),
        ] {
            let btn = gtk::Button::with_label(label);
            let controller = controller.clone();
            btn.connect_clicked(move |_| {
                let mut c = controller.borrow_mut();
                if let Err(e) = c.apply_action_to_selected(
                    Action::Mouse {
                        action,
                        modifiers: Modifiers::empty(),
                    },
                    None,
                ) {
                    c.model.status = e.to_string();
                }
            });
            mouse_box.append(&btn);
        }
        categories.append(&make_row("Mouse", ActionKind::Mouse, mouse_box.upcast_ref()));

        // Lighting
        let light_box = gtk::Box::new(gtk::Orientation::Vertical, 6);
        let mode_spin = gtk::SpinButton::with_range(0.0, 5.0, 1.0);
        let color_spin = gtk::SpinButton::with_range(0.0, 7.0, 1.0);
        let apply_light = gtk::Button::with_label("Apply lighting");
        {
            let controller = controller.clone();
            let mode_spin = mode_spin.clone();
            let color_spin = color_spin.clone();
            apply_light.connect_clicked(move |_| {
                let mode = mode_spin.value() as u8;
                let color = color_spin.value() as u8;
                let mut c = controller.borrow_mut();
                if let Err(e) =
                    c.apply_action_to_selected(Action::Lighting { mode, color }, None)
                {
                    c.model.status = e.to_string();
                }
            });
        }
        light_box.append(&gtk::Label::new(Some("Mode (0–5)")));
        light_box.append(&mode_spin);
        light_box.append(&gtk::Label::new(Some("Color (0–7)")));
        light_box.append(&color_spin);
        light_box.append(&apply_light);
        categories.append(&make_row(
            "Lighting",
            ActionKind::Lighting,
            light_box.upcast_ref(),
        ));

        // Delay
        let delay_box = gtk::Box::new(gtk::Orientation::Vertical, 6);
        let delay_spin = gtk::SpinButton::with_range(0.0, 65535.0, 1.0);
        let apply_delay = gtk::Button::with_label("Apply delay");
        {
            let controller = controller.clone();
            let delay_spin = delay_spin.clone();
            apply_delay.connect_clicked(move |_| {
                let delay_ms = delay_spin.value() as u16;
                let mut c = controller.borrow_mut();
                if let Err(e) =
                    c.apply_action_to_selected(Action::Delay { delay_ms }, Some(delay_ms))
                {
                    c.model.status = e.to_string();
                }
            });
        }
        delay_box.append(&delay_spin);
        delay_box.append(&apply_delay);
        categories.append(&make_row("Delay", ActionKind::Delay, delay_box.upcast_ref()));

        // Clear
        let clear_btn = gtk::Button::with_label("Clear position");
        clear_btn.set_sensitive(verified.contains(&ActionKind::Empty));
        {
            let controller = controller.clone();
            clear_btn.connect_clicked(move |_| {
                let mut c = controller.borrow_mut();
                if let Err(e) = c.apply_action_to_selected(Action::Empty, None) {
                    c.model.status = e.to_string();
                }
            });
        }
        categories.append(&clear_btn);

        let scroll = gtk::ScrolledWindow::new();
        scroll.set_child(Some(&categories));
        scroll.set_vexpand(true);
        stack.add_child(&scroll);
        root.append(&stack);

        // Silence unused key_model if rebuilt
        let _ = key_model;

        Self {
            root,
            stack,
            opaque_label,
            help_label,
        }
    }

    pub fn render(&self, model: &AppModel) {
        if let Some(pos) = model.selected_position_config() {
            match &pos.action {
                Action::Opaque { mode } => {
                    self.opaque_label.set_text(&format!(
                        "Opaque mode {mode}. This action is preserved but cannot be edited safely."
                    ));
                    self.opaque_label.set_visible(true);
                }
                Action::Keyboard { modifiers, usage } if bindings::is_marker(*usage) => {
                    let list = bindings::load_bindings().unwrap_or_default();
                    let text = match bindings::find(&list, *modifiers, *usage) {
                        Some(b) => format!("Aplicação: {} — {}", b.name, b.command),
                        None => format!(
                            "Current: {} (marcador sem vínculo)",
                            pos.action.short_label()
                        ),
                    };
                    self.opaque_label.set_text(&text);
                    self.opaque_label.set_visible(true);
                }
                other => {
                    self.opaque_label
                        .set_text(&format!("Current: {}", other.short_label()));
                    self.opaque_label.set_visible(true);
                }
            }
            if !model.session_state.allows_edit() {
                self.help_label
                    .set_text("Editing locked in the current session state.");
            } else {
                self.help_label.set_text("");
            }
        } else {
            self.opaque_label.set_text("Select a key to edit.");
            self.help_label.set_text("");
        }
        let _ = &self.stack;
    }
}
