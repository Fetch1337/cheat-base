use crate::input;
use crate::ui;
use hudhook::imgui;
use hudhook::*;

pub struct Overlay;

impl ImguiRenderLoop for Overlay {
    fn render(&mut self, ui: &mut imgui::Ui) {
        input::on_render(ui);
        ui::on_render(ui);
        draw::render_example(ui, None);
    }
}

#[allow(dead_code)]
pub mod draw {
    use hudhook::imgui::draw_list::DrawFlags;
    use hudhook::imgui::{self, DrawListMut, FontId, ImColor32, Ui};

    const TEXT_OUTLINE_DIR4: [[f32; 2]; 4] = [[-1.0, 0.0], [1.0, 0.0], [0.0, -1.0], [0.0, 1.0]];
    const TEXT_OUTLINE_DIR8: [[f32; 2]; 8] = [
        [-1.0, 0.0],
        [1.0, 0.0],
        [0.0, -1.0],
        [0.0, 1.0],
        [-1.0, -1.0],
        [1.0, -1.0],
        [-1.0, 1.0],
        [1.0, 1.0],
    ];

    macro_rules! impl_common_draw_methods {
        () => {
            pub fn filled(mut self, value: bool) -> Self {
                self.filled = value;
                self
            }
            pub fn outline(mut self) -> Self {
                self.outline = true;
                self
            }
            pub fn outline_color(mut self, color: ImColor32) -> Self {
                self.outline = true;
                self.outline_color = Some(color);
                self
            }
            pub fn thickness(mut self, value: f32) -> Self {
                self.thickness = Some(value);
                self
            }
            pub fn outline_thickness(mut self, value: f32) -> Self {
                self.outline_thickness = Some(value);
                self
            }
        };
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum DrawLayer {
        Window,
        Background,
        Foreground,
    }

    #[derive(Clone, Copy, Debug)]
    pub struct DrawDefaults {
        pub outline_color: ImColor32,
        pub rounding: f32,
        pub thickness: f32,
        pub outline_thickness: f32,
        pub flags: DrawFlags,
        pub segments: u32,
        pub font: Option<FontId>,
        pub font_size: Option<f32>,
    }

    impl Default for DrawDefaults {
        fn default() -> Self {
            Self {
                outline_color: ImColor32::BLACK,
                rounding: 0.0,
                thickness: 1.0,
                outline_thickness: 1.0,
                flags: DrawFlags::ROUND_CORNERS_ALL,
                segments: 0,
                font: None,
                font_size: None,
            }
        }
    }

    pub struct DrawContext<'ui> {
        ui: &'ui Ui,
        layer: DrawLayer,
        pub defaults: DrawDefaults,
    }

    impl<'ui> DrawContext<'ui> {
        pub fn new(ui: &'ui Ui, layer: DrawLayer) -> Self {
            Self {
                ui,
                layer,
                defaults: DrawDefaults::default(),
            }
        }

        pub fn foreground(ui: &'ui Ui) -> Self {
            Self::new(ui, DrawLayer::Foreground)
        }
        pub fn background(ui: &'ui Ui) -> Self {
            Self::new(ui, DrawLayer::Background)
        }
        pub fn window(ui: &'ui Ui) -> Self {
            Self::new(ui, DrawLayer::Window)
        }

        fn get_draw_list(&self) -> DrawListMut<'_> {
            match self.layer {
                DrawLayer::Window => self.ui.get_window_draw_list(),
                DrawLayer::Background => self.ui.get_background_draw_list(),
                DrawLayer::Foreground => self.ui.get_foreground_draw_list(),
            }
        }

        pub fn rect(&self, min: [f32; 2], max: [f32; 2], color: ImColor32) -> RectCmd<'_, 'ui> {
            RectCmd {
                ctx: self,
                min,
                max,
                color,
                filled: false,
                outline: false,
                outline_color: None,
                rounding: None,
                flags: None,
                thickness: None,
                outline_thickness: None,
            }
        }

        pub fn circle(
            &self,
            center: [f32; 2],
            radius: f32,
            color: ImColor32,
        ) -> CircleCmd<'_, 'ui> {
            CircleCmd {
                ctx: self,
                center,
                radius,
                color,
                filled: false,
                outline: false,
                outline_color: None,
                thickness: None,
                outline_thickness: None,
                segments: None,
            }
        }

        pub fn line(&self, p1: [f32; 2], p2: [f32; 2], color: ImColor32) -> LineCmd<'_, 'ui> {
            LineCmd {
                ctx: self,
                p1,
                p2,
                color,
                thickness: None,
            }
        }

        pub fn text<'txt>(
            &self,
            pos: [f32; 2],
            text: &'txt str,
            color: ImColor32,
        ) -> TextCmd<'_, 'ui, 'txt> {
            TextCmd {
                ctx: self,
                pos,
                text,
                color,
                outline: false,
                outline_color: None,
                outline_thickness: None,
                font: None,
                size: None,
            }
        }

        pub fn polygon<'pts>(
            &self,
            points: &'pts [[f32; 2]],
            color: ImColor32,
        ) -> PolygonCmd<'_, 'ui, 'pts> {
            PolygonCmd {
                ctx: self,
                points,
                color,
                filled: true,
                outline: false,
                outline_color: None,
                closed: true,
                thickness: None,
                outline_thickness: None,
            }
        }
    }

    pub struct RectCmd<'ctx, 'ui> {
        ctx: &'ctx DrawContext<'ui>,
        min: [f32; 2],
        max: [f32; 2],
        color: ImColor32,
        filled: bool,
        outline: bool,
        outline_color: Option<ImColor32>,
        rounding: Option<f32>,
        flags: Option<DrawFlags>,
        thickness: Option<f32>,
        outline_thickness: Option<f32>,
    }

    impl RectCmd<'_, '_> {
        impl_common_draw_methods!();
        pub fn rounding(mut self, value: f32) -> Self {
            self.rounding = Some(value);
            self
        }

        pub fn draw(self) {
            let dl = self.ctx.get_draw_list();
            let rounding = self.rounding.unwrap_or(self.ctx.defaults.rounding);
            let flags = self.flags.unwrap_or(self.ctx.defaults.flags);

            if self.filled {
                dl.add_rect(self.min, self.max, self.color)
                    .filled(true)
                    .rounding(rounding)
                    .round_top_left(flags.contains(DrawFlags::ROUND_CORNERS_TOP_LEFT))
                    .round_top_right(flags.contains(DrawFlags::ROUND_CORNERS_TOP_RIGHT))
                    .round_bot_left(flags.contains(DrawFlags::ROUND_CORNERS_BOT_LEFT))
                    .round_bot_right(flags.contains(DrawFlags::ROUND_CORNERS_BOT_RIGHT))
                    .build();
            }

            if self.outline || !self.filled {
                let color = self.outline_color.unwrap_or(if self.filled {
                    self.ctx.defaults.outline_color
                } else {
                    self.color
                });
                let thick = self
                    .outline_thickness
                    .or(self.thickness)
                    .unwrap_or(self.ctx.defaults.thickness);

                dl.add_rect(self.min, self.max, color)
                    .filled(false)
                    .thickness(thick)
                    .rounding(rounding)
                    .build();
            }
        }
    }

    pub struct CircleCmd<'ctx, 'ui> {
        ctx: &'ctx DrawContext<'ui>,
        center: [f32; 2],
        radius: f32,
        color: ImColor32,
        filled: bool,
        outline: bool,
        outline_color: Option<ImColor32>,
        thickness: Option<f32>,
        outline_thickness: Option<f32>,
        segments: Option<u32>,
    }

    impl CircleCmd<'_, '_> {
        impl_common_draw_methods!();
        pub fn segments(mut self, value: u32) -> Self {
            self.segments = Some(value);
            self
        }

        pub fn draw(self) {
            let dl = self.ctx.get_draw_list();
            let segs = self.segments.unwrap_or(self.ctx.defaults.segments);

            if self.filled {
                dl.add_circle(self.center, self.radius, self.color)
                    .filled(true)
                    .num_segments(segs)
                    .build();
            }

            if self.outline || !self.filled {
                let color = self.outline_color.unwrap_or(if self.filled {
                    self.ctx.defaults.outline_color
                } else {
                    self.color
                });
                let thick = self
                    .outline_thickness
                    .or(self.thickness)
                    .unwrap_or(self.ctx.defaults.thickness);

                dl.add_circle(self.center, self.radius, color)
                    .filled(false)
                    .thickness(thick)
                    .num_segments(segs)
                    .build();
            }
        }
    }

    pub struct LineCmd<'ctx, 'ui> {
        ctx: &'ctx DrawContext<'ui>,
        p1: [f32; 2],
        p2: [f32; 2],
        color: ImColor32,
        thickness: Option<f32>,
    }

    impl LineCmd<'_, '_> {
        pub fn thickness(mut self, value: f32) -> Self {
            self.thickness = Some(value);
            self
        }
        pub fn draw(self) {
            let thick = self.thickness.unwrap_or(self.ctx.defaults.thickness);
            self.ctx
                .get_draw_list()
                .add_line(self.p1, self.p2, self.color)
                .thickness(thick)
                .build();
        }
    }

    pub struct TextCmd<'ctx, 'ui, 'txt> {
        ctx: &'ctx DrawContext<'ui>,
        pos: [f32; 2],
        text: &'txt str,
        color: ImColor32,
        outline: bool,
        outline_color: Option<ImColor32>,
        outline_thickness: Option<f32>,
        font: Option<FontId>,
        size: Option<f32>,
    }

    impl TextCmd<'_, '_, '_> {
        pub fn outline(mut self) -> Self {
            self.outline = true;
            self
        }
        pub fn outline_color(mut self, color: ImColor32) -> Self {
            self.outline_color = Some(color);
            self.outline = true;
            self
        }
        pub fn outline_thickness(mut self, value: f32) -> Self {
            self.outline_thickness = Some(value);
            self
        }
        pub fn font(mut self, font: FontId) -> Self {
            self.font = Some(font);
            self
        }
        pub fn size(mut self, size: f32) -> Self {
            self.size = Some(size);
            self
        }

        pub fn draw(self) {
            let ui = self.ctx.ui;
            let _font_token = self.font.map(|f| ui.push_font(f));

            let mut reset_scale = false;
            if let Some(target_size) = self.size {
                let current_font_size = ui.current_font_size();
                if (target_size - current_font_size).abs() > 0.1 {
                    ui.set_window_font_scale(target_size / ui.current_font().font_size);
                    reset_scale = true;
                }
            }

            let dl = self.ctx.get_draw_list();
            if self.outline {
                let out_color = self
                    .outline_color
                    .unwrap_or(self.ctx.defaults.outline_color);
                let out_thick = self
                    .outline_thickness
                    .unwrap_or(self.ctx.defaults.outline_thickness);
                let dirs = if out_thick <= 1.0 {
                    &TEXT_OUTLINE_DIR4[..]
                } else {
                    &TEXT_OUTLINE_DIR8[..]
                };

                for d in dirs {
                    dl.add_text(
                        [
                            self.pos[0] + d[0] * out_thick,
                            self.pos[1] + d[1] * out_thick,
                        ],
                        out_color,
                        self.text,
                    );
                }
            }

            dl.add_text(self.pos, self.color, self.text);

            if reset_scale {
                ui.set_window_font_scale(1.0);
            }
        }
    }

    pub struct PolygonCmd<'ctx, 'ui, 'pts> {
        ctx: &'ctx DrawContext<'ui>,
        points: &'pts [[f32; 2]],
        color: ImColor32,
        filled: bool,
        outline: bool,
        outline_color: Option<ImColor32>,
        closed: bool,
        thickness: Option<f32>,
        outline_thickness: Option<f32>,
    }

    impl PolygonCmd<'_, '_, '_> {
        impl_common_draw_methods!();
        pub fn closed(mut self, value: bool) -> Self {
            self.closed = value;
            self
        }

        pub fn draw(self) {
            if self.points.len() < 2 {
                return;
            }
            let dl = self.ctx.get_draw_list();

            if self.filled {
                dl.add_polyline(self.points.iter().cloned().collect::<Vec<_>>(), self.color)
                    .filled(true)
                    .build();
            }

            if self.outline || !self.filled {
                let color = self.outline_color.unwrap_or(if self.filled {
                    self.ctx.defaults.outline_color
                } else {
                    self.color
                });
                let thick = self
                    .outline_thickness
                    .or(self.thickness)
                    .unwrap_or(self.ctx.defaults.thickness);

                dl.add_polyline(self.points.iter().cloned().collect::<Vec<_>>(), color)
                    .filled(false)
                    .thickness(thick)
                    .build();
            }
        }
    }

    pub fn render_example(ui: &imgui::Ui, maybe_font: Option<FontId>) {
        let draw = DrawContext::foreground(ui);

        draw.rect(
            [40.0, 40.0],
            [220.0, 120.0],
            ImColor32::from_rgb(35, 120, 220),
        )
        .rounding(6.0)
        .outline()
        .outline_thickness(2.0)
        .draw();

        draw.circle([320.0, 90.0], 28.0, ImColor32::from_rgb(90, 220, 130))
            .outline()
            .segments(32)
            .draw();

        let mut text = draw.text([44.0, 46.0], "Text", ImColor32::WHITE).outline();
        if let Some(font) = maybe_font {
            text = text.font(font).size(18.0);
        }
        text.draw();
    }
}
