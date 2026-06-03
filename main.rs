use eframe::egui::{self, Align, Color32, FontId, Key, Layout, RichText, Stroke, Vec2};

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([410.0, 690.0])
            .with_min_inner_size([370.0, 620.0])
            .with_title("Calculator"),
        ..Default::default()
    };

    eframe::run_native(
        "Calculator",
        options,
        Box::new(|cc| Ok(Box::new(GlassCalculator::new(cc)))),
    )
}

#[derive(Clone, Copy, PartialEq)]
enum Theme {
    Obsidian,
    Emerald,
    NeonViolet,
    Arctic,
}

impl Theme {
    const ALL: [Theme; 4] = [
        Theme::Obsidian,
        Theme::Emerald,
        Theme::NeonViolet,
        Theme::Arctic,
    ];

    fn name(self) -> &'static str {
        match self {
            Theme::Obsidian => "Obsidian",
            Theme::Emerald => "Emerald",
            Theme::NeonViolet => "Neon Violet",
            Theme::Arctic => "Arctic Glass",
        }
    }

    fn colors(self) -> ThemeColors {
        match self {
            Theme::Obsidian => ThemeColors {
                bg: Color32::from_rgb(5, 7, 11),
                glow_a: Color32::from_rgba_premultiplied(70, 145, 255, 45),
                glow_b: Color32::from_rgba_premultiplied(122, 84, 255, 36),
                accent: Color32::from_rgb(102, 175, 255),
                accent_soft: Color32::from_rgba_premultiplied(102, 175, 255, 36),
                panel: Color32::from_rgba_premultiplied(18, 21, 29, 220),
                button: Color32::from_rgba_premultiplied(32, 39, 55, 235),
                operator: Color32::from_rgba_premultiplied(42, 104, 190, 235),
            },
            Theme::Emerald => ThemeColors {
                bg: Color32::from_rgb(3, 10, 9),
                glow_a: Color32::from_rgba_premultiplied(14, 255, 180, 44),
                glow_b: Color32::from_rgba_premultiplied(0, 125, 89, 42),
                accent: Color32::from_rgb(42, 239, 178),
                accent_soft: Color32::from_rgba_premultiplied(42, 239, 178, 34),
                panel: Color32::from_rgba_premultiplied(7, 25, 22, 222),
                button: Color32::from_rgba_premultiplied(18, 43, 39, 235),
                operator: Color32::from_rgba_premultiplied(10, 133, 99, 235),
            },
            Theme::NeonViolet => ThemeColors {
                bg: Color32::from_rgb(9, 5, 14),
                glow_a: Color32::from_rgba_premultiplied(223, 55, 255, 48),
                glow_b: Color32::from_rgba_premultiplied(56, 178, 255, 37),
                accent: Color32::from_rgb(229, 104, 255),
                accent_soft: Color32::from_rgba_premultiplied(229, 104, 255, 40),
                panel: Color32::from_rgba_premultiplied(25, 14, 34, 224),
                button: Color32::from_rgba_premultiplied(42, 28, 53, 235),
                operator: Color32::from_rgba_premultiplied(131, 38, 161, 235),
            },
            Theme::Arctic => ThemeColors {
                bg: Color32::from_rgb(5, 12, 18),
                glow_a: Color32::from_rgba_premultiplied(71, 229, 255, 44),
                glow_b: Color32::from_rgba_premultiplied(200, 238, 255, 30),
                accent: Color32::from_rgb(96, 224, 255),
                accent_soft: Color32::from_rgba_premultiplied(96, 224, 255, 37),
                panel: Color32::from_rgba_premultiplied(14, 27, 38, 218),
                button: Color32::from_rgba_premultiplied(22, 43, 57, 235),
                operator: Color32::from_rgba_premultiplied(13, 123, 151, 235),
            },
        }
    }
}

struct ThemeColors {
    bg: Color32,
    glow_a: Color32,
    glow_b: Color32,
    accent: Color32,
    accent_soft: Color32,
    panel: Color32,
    button: Color32,
    operator: Color32,
}

struct GlassCalculator {
    expression: String,
    result: String,
    theme: Theme,
    just_evaluated: bool,
}

impl GlassCalculator {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let app = Self {
            expression: String::new(),
            result: "0".to_owned(),
            theme: Theme::Obsidian,
            just_evaluated: false,
        };
        app.apply_visuals(&cc.egui_ctx);
        app
    }

    fn apply_visuals(&self, ctx: &egui::Context) {
        let c = self.theme.colors();
        let mut visuals = egui::Visuals::dark();
        visuals.panel_fill = c.bg;
        visuals.window_fill = c.panel;
        visuals.extreme_bg_color = Color32::from_rgba_premultiplied(255, 255, 0, 8);
        visuals.override_text_color = Some(Color32::from_rgb(232, 238, 247));
        visuals.selection.bg_fill = c.accent_soft;
        visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, Color32::from_rgb(224, 231, 240));
        visuals.widgets.hovered.fg_stroke = Stroke::new(1.0, Color32::WHITE);
        visuals.widgets.active.fg_stroke = Stroke::new(1.0, Color32::WHITE);
        ctx.set_visuals(visuals);
    }

    fn append(&mut self, token: &str) {
        if self.just_evaluated && token.chars().all(|c| c.is_ascii_digit() || c == '.') {
            self.expression.clear();
        }
        self.just_evaluated = false;
        self.expression.push_str(token);
    }

    fn clear(&mut self) {
        self.expression.clear();
        self.result = "0".to_owned();
        self.just_evaluated = false;
    }

    fn backspace(&mut self) {
        self.expression.pop();
        self.just_evaluated = false;
    }

    fn toggle_sign(&mut self) {
        if self.expression.is_empty() {
            self.expression.push('-');
        } else if self.expression.starts_with('-') {
            self.expression.remove(0);
        } else {
            self.expression.insert(0, '-');
        }
    }

    fn percent(&mut self) {
        if !self.expression.is_empty() {
            self.expression = format!("({})/100", self.expression);
            self.evaluate();
        }
    }

    fn evaluate(&mut self) {
        if self.expression.trim().is_empty() {
            return;
        }
        match Parser::new(&self.expression).parse() {
            Ok(value) if value.is_finite() => {
                self.result = format_number(value);
                self.expression = self.result.clone();
                self.just_evaluated = true;
            }
            Ok(_) | Err(_) => {
                self.result = "Error".to_owned();
                self.just_evaluated = true;
            }
        }
    }

    fn keyboard_input(&mut self, ctx: &egui::Context) {
        ctx.input(|input| {
            if input.key_pressed(Key::Enter) {
                self.evaluate();
            }
            if input.key_pressed(Key::Backspace) {
                self.backspace();
            }
            if input.key_pressed(Key::Escape) || input.key_pressed(Key::Delete) {
                self.clear();
            }

            for event in &input.events {
                if let egui::Event::Text(text) = event {
                    for ch in text.chars() {
                        match ch {
                            '0'..='9' | '.' | '(' | ')' => self.append(&ch.to_string()),
                            '+' | '-' | '*' | '/' => self.append(&ch.to_string()),
                            '=' => self.evaluate(),
                            '%' => self.percent(),
                            _ => {}
                        }
                    }
                }
            }
        });
    }

    fn background(&self, ctx: &egui::Context) {
        let c = self.theme.colors();
        let rect = ctx.content_rect();
        let painter = ctx.layer_painter(egui::LayerId::background());
        painter.rect_filled(rect, 0.0, c.bg);
        painter.circle_filled(rect.left_top() + Vec2::new(72.0, 104.0), 155.0, c.glow_a);
        painter.circle_filled(rect.right_bottom() - Vec2::new(55.0, 110.0), 180.0, c.glow_b);
        painter.circle_filled(rect.center() + Vec2::new(110.0, -40.0), 75.0, c.accent_soft);
    }

    fn glass_button(
        &mut self,
        ui: &mut egui::Ui,
        label: &str,
        action: ButtonAction,
        strong: bool,
        size: Vec2,
    ) {
        let c = self.theme.colors();
        let fill = if strong { c.operator } else { c.button };
        let text_color = if strong { c.accent } else { Color32::from_rgb(235, 42, 248) };
        let button = egui::Button::new(RichText::new(label).size(22.0).color(text_color))
            .min_size(size)
            .fill(fill)
            .stroke(Stroke::new(1.0, Color32::from_rgba_premultiplied(0, 255, 0, 24)))
            .corner_radius(16.0);

        if ui.add(button).clicked() {
            match action {
                ButtonAction::Append(value) => self.append(value),
                ButtonAction::Clear => self.clear(),
                ButtonAction::Backspace => self.backspace(),
                ButtonAction::Sign => self.toggle_sign(),
                ButtonAction::Percent => self.percent(),
                ButtonAction::Equals => self.evaluate(),
            }
        }
    }
}

impl eframe::App for GlassCalculator {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();

        self.keyboard_input(&ctx);
        self.background(&ctx);

        let c = self.theme.colors();
        egui::CentralPanel::default()
            .frame(
                egui::Frame::new()
                    .fill(Color32::TRANSPARENT)
                    .inner_margin(egui::Margin::same(14)),
            )
            .show_inside(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new("^_~")
                            .size(13.0)
                            .color(c.accent)
                            .strong(),
                    );
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        let old_theme = self.theme;
                        egui::ComboBox::from_id_salt("theme_select")
                            .selected_text(self.theme.name())
                            .width(118.0)
                            .show_ui(ui, |ui| {
                                for theme in Theme::ALL {
                                    ui.selectable_value(&mut self.theme, theme, theme.name());
                                }
                            });
                        if self.theme != old_theme {
                            self.apply_visuals(&ctx);
                        }
                    });
                });

                ui.add_space(12.0);
                let panel_frame = egui::Frame::new()
                    .fill(c.panel)
                    .stroke(Stroke::new(1.0, Color32::from_rgba_premultiplied(255, 255, 255, 30)))
                    .corner_radius(24.0)
                    .inner_margin(egui::Margin::same(15));

                panel_frame.show(ui, |ui| {
                    ui.set_width(ui.available_width());
                    ui.with_layout(Layout::top_down(Align::Max), |ui| {
                        ui.label(
                            RichText::new(if self.expression.is_empty() { "0" } else { &self.expression })
                                .font(FontId::proportional(19.0))
                                .color(Color32::from_rgb(151, 0, 180)),
                        );
                        ui.add_space(5.0);
                        ui.label(
                            RichText::new(&self.result)
                                .font(FontId::proportional(39.0))
                                .color(Color32::from_rgb(246, 249, 252)),
                        );
                    });
                });

                ui.add_space(12.0);

                // Calculate the key size from the remaining window area.
                // This keeps all five keypad rows visible if the window is small.
                let keypad_margin = 11.0;
                let key_gap = 7.0;
                let footer_height = 24.0;
                let usable_width = ui.available_width() - (keypad_margin * 2.0) - (key_gap * 3.0);
                let key_width = (usable_width / 4.0).max(58.0);
                let usable_height = ui.available_height() - footer_height - (keypad_margin * 2.0) - (key_gap * 4.0);
                let key_height = (usable_height / 5.0).clamp(42.0, 58.0);
                let key_size = Vec2::new(key_width, key_height);

                let keypad = egui::Frame::new()
                    .fill(Color32::TRANSPARENT)
                    .stroke(Stroke::new(
                        1.0,
                        Color32::from_rgba_premultiplied(255, 0, 255, 10),
                    ))
                    .corner_radius(24.0)
                    .inner_margin(egui::Margin::same(13));

                keypad.show(ui, |ui| {
                    egui::Grid::new("keypad")
                        .spacing(Vec2::new(key_gap, key_gap))
                        .show(ui, |ui| {
                            self.glass_button(ui, "C", ButtonAction::Clear, true, key_size);
                            self.glass_button(ui, "±", ButtonAction::Sign, false, key_size);
                            self.glass_button(ui, "%", ButtonAction::Percent, false, key_size);
                            self.glass_button(ui, "÷", ButtonAction::Append("/"), true, key_size);
                            ui.end_row();

                            self.glass_button(ui, "7", ButtonAction::Append("7"), false, key_size);
                            self.glass_button(ui, "8", ButtonAction::Append("8"), false, key_size);
                            self.glass_button(ui, "9", ButtonAction::Append("9"), false, key_size);
                            self.glass_button(ui, "×", ButtonAction::Append("*"), true, key_size);
                            ui.end_row();

                            self.glass_button(ui, "4", ButtonAction::Append("4"), false, key_size);
                            self.glass_button(ui, "5", ButtonAction::Append("5"), false, key_size);
                            self.glass_button(ui, "6", ButtonAction::Append("6"), false, key_size);
                            self.glass_button(ui, "−", ButtonAction::Append("-"), true, key_size);
                            ui.end_row();

                            self.glass_button(ui, "1", ButtonAction::Append("1"), false, key_size);
                            self.glass_button(ui, "2", ButtonAction::Append("2"), false, key_size);
                            self.glass_button(ui, "3", ButtonAction::Append("3"), false, key_size);
                            self.glass_button(ui, "+", ButtonAction::Append("+"), true, key_size);
                            ui.end_row();

                            self.glass_button(ui, "⌫", ButtonAction::Backspace, false, key_size);
                            self.glass_button(ui, "0", ButtonAction::Append("0"), false, key_size);
                            self.glass_button(ui, ".", ButtonAction::Append("."), false, key_size);
                            self.glass_button(ui, "=", ButtonAction::Equals, true, key_size);
                            ui.end_row();
                        });
                });

                ui.add_space(7.0);
                ui.label(
                    RichText::new("Keyboard: numbers  +  −  ×  ÷  Enter  Backspace  Esc")
                        .size(10.0)
                        .color(Color32::from_rgb(115, 130, 148)),
                );
            });
    }
}

#[derive(Clone, Copy)]
enum ButtonAction {
    Append(&'static str),
    Clear,
    Backspace,
    Sign,
    Percent,
    Equals,
}

fn format_number(number: f64) -> String {
    let mut out = format!("{number:.10}");
    while out.contains('.') && out.ends_with('0') {
        out.pop();
    }
    if out.ends_with('.') {
        out.pop();
    }
    if out == "-0" {
        "0".to_owned()
    } else {
        out
    }
}

struct Parser<'a> {
    chars: Vec<char>,
    pos: usize,
    _source: &'a str,
}

impl<'a> Parser<'a> {
    fn new(source: &'a str) -> Self {
        Self {
            chars: source.chars().filter(|ch| !ch.is_whitespace()).collect(),
            pos: 0,
            _source: source,
        }
    }

    fn parse(mut self) -> Result<f64, String> {
        let value = self.expression()?;
        if self.pos != self.chars.len() {
            return Err("Unexpected remaining characters".to_owned());
        }
        Ok(value)
    }

    fn expression(&mut self) -> Result<f64, String> {
        let mut value = self.term()?;
        loop {
            match self.peek() {
                Some('+') => {
                    self.pos += 1;
                    value += self.term()?;
                }
                Some('-') => {
                    self.pos += 1;
                    value -= self.term()?;
                }
                _ => return Ok(value),
            }
        }
    }

    fn term(&mut self) -> Result<f64, String> {
        let mut value = self.factor()?;
        loop {
            match self.peek() {
                Some('*') => {
                    self.pos += 1;
                    value *= self.factor()?;
                }
                Some('/') => {
                    self.pos += 1;
                    let divisor = self.factor()?;
                    if divisor == 0.0 {
                        return Err("Cannot divide by zero".to_owned());
                    }
                    value /= divisor;
                }
                _ => return Ok(value),
            }
        }
    }

    fn factor(&mut self) -> Result<f64, String> {
        match self.peek() {
            Some('+') => {
                self.pos += 1;
                self.factor()
            }
            Some('-') => {
                self.pos += 1;
                Ok(-self.factor()?)
            }
            Some('(') => {
                self.pos += 1;
                let value = self.expression()?;
                if self.peek() != Some(')') {
                    return Err("Missing close parenthesis".to_owned());
                }
                self.pos += 1;
                Ok(value)
            }
            _ => self.number(),
        }
    }

    fn number(&mut self) -> Result<f64, String> {
        let start = self.pos;
        let mut seen_dot = false;
        while let Some(ch) = self.peek() {
            if ch.is_ascii_digit() {
                self.pos += 1;
            } else if ch == '.' && !seen_dot {
                seen_dot = true;
                self.pos += 1;
            } else {
                break;
            }
        }
        if self.pos == start {
            return Err("Expected a number".to_owned());
        }
        self.chars[start..self.pos]
            .iter()
            .collect::<String>()
            .parse::<f64>()
            .map_err(|_| "Invalid number".to_owned())
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }
}

