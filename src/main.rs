#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

mod compare;
mod decode;

use compare::{diff, flatten, Inventory, Row};
use eframe::egui::{self, Color32, CornerRadius, RichText, Stroke};
use std::path::PathBuf;

const BG: Color32 = Color32::from_rgb(32, 32, 32);
const CARD: Color32 = Color32::from_rgb(43, 43, 43);
const CARD_HI: Color32 = Color32::from_rgb(55, 55, 55);
const STROKE: Color32 = Color32::from_rgb(64, 64, 64);
const ACCENT: Color32 = Color32::from_rgb(96, 205, 255);
const TEXT: Color32 = Color32::from_rgb(255, 255, 255);
const SUBTLE: Color32 = Color32::from_rgb(170, 170, 170);
const GOOD: Color32 = Color32::from_rgb(108, 203, 95);
const WARN: Color32 = Color32::from_rgb(252, 225, 0);
const BAD: Color32 = Color32::from_rgb(255, 153, 164);

fn apply_fluent_theme(ctx: &egui::Context) {
    // Use Segoe UI (the WinUI font) when running on Windows.
    if let Ok(bytes) = std::fs::read(r"C:\Windows\Fonts\segoeui.ttf") {
        let mut fonts = egui::FontDefinitions::default();
        fonts
            .font_data
            .insert("segoe".into(), std::sync::Arc::new(egui::FontData::from_owned(bytes)));
        fonts.families.get_mut(&egui::FontFamily::Proportional).unwrap().insert(0, "segoe".into());
        ctx.set_fonts(fonts);
    }
    let mut v = egui::Visuals::dark();
    let r = CornerRadius::same(6);
    v.panel_fill = BG;
    v.window_fill = CARD;
    v.extreme_bg_color = Color32::from_rgb(28, 28, 28);
    v.override_text_color = Some(TEXT);
    v.selection.bg_fill = ACCENT.gamma_multiply(0.35);
    v.selection.stroke = Stroke::new(1.0, ACCENT);
    v.hyperlink_color = ACCENT;
    for w in [&mut v.widgets.noninteractive, &mut v.widgets.inactive, &mut v.widgets.hovered, &mut v.widgets.active] {
        w.corner_radius = r;
    }
    v.widgets.noninteractive.bg_stroke = Stroke::new(1.0, STROKE);
    v.widgets.inactive.bg_fill = CARD_HI;
    v.widgets.inactive.weak_bg_fill = CARD_HI;
    v.widgets.inactive.bg_stroke = Stroke::new(1.0, STROKE);
    v.widgets.hovered.bg_fill = Color32::from_rgb(66, 66, 66);
    v.widgets.hovered.weak_bg_fill = Color32::from_rgb(66, 66, 66);
    v.widgets.hovered.bg_stroke = Stroke::new(1.0, STROKE);
    v.widgets.active.bg_fill = ACCENT;
    v.widgets.active.weak_bg_fill = ACCENT;
    ctx.set_visuals(v);
    ctx.global_style_mut(|s| {
        s.spacing.item_spacing = egui::vec2(10.0, 10.0);
        s.spacing.button_padding = egui::vec2(14.0, 7.0);
        s.spacing.interact_size.y = 32.0;
    });
}

fn card<R>(ui: &mut egui::Ui, add: impl FnOnce(&mut egui::Ui) -> R) -> R {
    egui::Frame::new()
        .fill(CARD)
        .stroke(Stroke::new(1.0, STROKE))
        .corner_radius(CornerRadius::same(8))
        .inner_margin(egui::Margin::same(16))
        .show(ui, add)
        .inner
}

fn accent_button(text: &str) -> egui::Button<'static> {
    egui::Button::new(RichText::new(text.to_owned()).color(Color32::BLACK).strong())
        .fill(ACCENT)
        .corner_radius(CornerRadius::same(6))
}

struct Slot {
    label: String,
    path: Option<PathBuf>,
    inv: Option<Inventory>,
    error: Option<String>,
}

impl Slot {
    fn new(label: &str) -> Self {
        Self { label: label.into(), path: None, inv: None, error: None }
    }
    fn load(&mut self, path: PathBuf) {
        match decode::load(&path) {
            Ok(v) => {
                let inv = flatten(&v);
                if inv.is_empty() {
                    self.inv = None;
                    self.error = Some("Decrypted fine, but no items were found in it.".into());
                } else {
                    self.inv = Some(inv);
                    self.error = None;
                }
            }
            Err(e) => {
                self.inv = None;
                self.error = Some(e);
            }
        }
        self.path = Some(path);
    }
}

#[derive(PartialEq, Clone, Copy)]
enum Mode {
    All,
    OnlyA,
    OnlyB,
    MoreA,
    MoreB,
    Same,
}

struct App {
    slots: [Slot; 2],
    rows: Vec<Row>,
    categories: Vec<String>,
    category: Option<String>,
    mode: Mode,
    search: String,
}

impl App {
    fn new() -> Self {
        Self {
            slots: [Slot::new("You"), Slot::new("Charlie")],
            rows: vec![],
            categories: vec![],
            category: None,
            mode: Mode::OnlyB,
            search: String::new(),
        }
    }

    fn pick(&mut self, i: usize) {
        if let Some(p) = rfd::FileDialog::new()
            .add_filter("AlecaFrame data", &["dat", "json"])
            .add_filter("All files", &["*"])
            .pick_file()
        {
            self.slots[i].load(p);
            self.rebuild();
        }
    }

    fn rebuild(&mut self) {
        self.rows.clear();
        if let (Some(a), Some(b)) = (&self.slots[0].inv, &self.slots[1].inv) {
            self.rows = diff(a, b);
        }
        let mut cats: Vec<String> = self.rows.iter().map(|r| r.category.clone()).collect();
        cats.sort();
        cats.dedup();
        self.categories = cats;
        if self.category.as_ref().is_some_and(|c| !self.categories.contains(c)) {
            self.category = None;
        }
    }

    fn matches(&self, r: &Row) -> bool {
        let ok = match self.mode {
            Mode::All => true,
            Mode::OnlyA => r.a > 0 && r.b == 0,
            Mode::OnlyB => r.b > 0 && r.a == 0,
            Mode::MoreA => r.a > r.b && r.b > 0,
            Mode::MoreB => r.b > r.a && r.a > 0,
            Mode::Same => r.a == r.b,
        };
        ok && self.category.as_ref().is_none_or(|c| &r.category == c)
            && (self.search.is_empty()
                || r.name.to_lowercase().contains(&self.search.to_lowercase())
                || r.path.to_lowercase().contains(&self.search.to_lowercase()))
    }

    fn slot_card(&mut self, ui: &mut egui::Ui, i: usize) {
        let mut pick = false;
        card(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.set_min_height(120.0);
            let s = &mut self.slots[i];
            ui.label(RichText::new(if i == 0 { "INVENTORY A" } else { "INVENTORY B" }).size(11.0).color(SUBTLE));
            ui.add(egui::TextEdit::singleline(&mut s.label).font(egui::TextStyle::Heading).frame(egui::Frame::NONE));
            match (&s.inv, &s.error, &s.path) {
                (Some(inv), _, Some(p)) => {
                    let total: u64 = inv.values().map(|e| e.count).sum();
                    ui.label(RichText::new(format!("✔  {} unique items · {} total", inv.len(), total)).color(GOOD));
                    ui.label(RichText::new(p.file_name().unwrap_or_default().to_string_lossy()).color(SUBTLE).size(12.0));
                }
                (_, Some(e), _) => {
                    ui.label(RichText::new(format!("⚠  {e}")).color(BAD));
                }
                _ => {
                    ui.label(RichText::new("Drop a lastData.dat here, or browse").color(SUBTLE));
                }
            }
            if ui.add(accent_button("Open file…")).clicked() {
                pick = true;
            }
        });
        if pick {
            self.pick(i);
        }
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();
        // Drag & drop: first file fills the first empty slot, second the next.
        let dropped: Vec<PathBuf> = ctx.input(|i| i.raw.dropped_files.iter().map(|f| f.path().to_path_buf()).collect());
        if !dropped.is_empty() {
            for p in dropped {
                let idx = if self.slots[0].inv.is_none() { 0 } else if self.slots[1].inv.is_none() { 1 } else { 1 };
                self.slots[idx].load(p);
            }
            self.rebuild();
        }

        egui::CentralPanel::default().frame(egui::Frame::new().fill(BG).inner_margin(egui::Margin::same(24))).show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new("Inventory Compare").size(28.0).strong());
                ui.label(RichText::new("Warframe · AlecaFrame lastData.dat").color(SUBTLE));
            });

            ui.columns(2, |cols| {
                self.slot_card(&mut cols[0], 0);
                self.slot_card(&mut cols[1], 1);
            });

            if self.rows.is_empty() {
                if self.slots.iter().all(|s| s.inv.is_some()) {
                    ui.label("Nothing to compare.");
                } else {
                    ui.add_space(20.0);
                    ui.label(RichText::new("Load two inventories to see who has what.").color(SUBTLE).size(16.0));
                }
                return;
            }

            let (na, nb) = (self.slots[0].label.clone(), self.slots[1].label.clone());
            ui.horizontal_wrapped(|ui| {
                ui.add(egui::TextEdit::singleline(&mut self.search).hint_text("🔍  Have you got…?").desired_width(240.0));
                let modes = [
                    (Mode::OnlyB, format!("Only {nb} has")),
                    (Mode::OnlyA, format!("Only {na} has")),
                    (Mode::MoreB, format!("{nb} has more")),
                    (Mode::MoreA, format!("{na} has more")),
                    (Mode::Same, "Same".to_string()),
                    (Mode::All, "Everything".to_string()),
                ];
                for (m, label) in modes {
                    ui.selectable_value(&mut self.mode, m, label);
                }
                egui::ComboBox::from_id_salt("cat")
                    .selected_text(self.category.clone().unwrap_or_else(|| "All categories".into()))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.category, None, "All categories");
                        for c in self.categories.clone() {
                            ui.selectable_value(&mut self.category, Some(c.clone()), c);
                        }
                    });
            });

            let visible: Vec<&Row> = self.rows.iter().filter(|r| self.matches(r)).collect();
            ui.label(RichText::new(format!("{} items", visible.len())).color(SUBTLE));

            card(ui, |ui| {
                let w = ui.available_width();
                let (cname, ccat, cnum) = (w * 0.45, w * 0.22, 80.0);
                ui.horizontal(|ui| {
                    for (t, wd) in [("Item", cname), ("Category", ccat), (na.as_str(), cnum), (nb.as_str(), cnum)] {
                        ui.add_sized([wd, 20.0], egui::Label::new(RichText::new(t).color(SUBTLE).strong()).truncate());
                    }
                });
                ui.separator();
                egui::ScrollArea::vertical().auto_shrink([false, false]).show_rows(ui, 28.0, visible.len(), |ui, range| {
                    ui.spacing_mut().item_spacing.y = 0.0;
                    for r in &visible[range] {
                        ui.horizontal(|ui| {
                            ui.set_height(28.0);
                            ui.add_sized([cname, 24.0], egui::Label::new(&r.name).truncate()).on_hover_text(&r.path);
                            ui.add_sized([ccat, 24.0], egui::Label::new(RichText::new(&r.category).color(SUBTLE)).truncate());
                            let col = |n: u64, other: u64| if n == 0 { BAD } else if n > other { GOOD } else if n < other { WARN } else { TEXT };
                            ui.add_sized([cnum, 24.0], egui::Label::new(RichText::new(r.a.to_string()).color(col(r.a, r.b))));
                            ui.add_sized([cnum, 24.0], egui::Label::new(RichText::new(r.b.to_string()).color(col(r.b, r.a))));
                        });
                    }
                });
            });
        });
    }
}

fn main() -> eframe::Result {
    let opts = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1000.0, 720.0]).with_min_inner_size([760.0, 480.0]).with_title("Inventory Compare").with_drag_and_drop(true),
        ..Default::default()
    };
    eframe::run_native(
        "Inventory Compare",
        opts,
        Box::new(|cc| {
            apply_fluent_theme(&cc.egui_ctx);
            Ok(Box::new(App::new()))
        }),
    )
}
