use crate::cfg;
use crate::input;
use crate::ui;
use hudhook::*;
use hudhook::imgui;

pub struct Overlay;

impl ImguiRenderLoop for Overlay {
    fn render(&mut self, ui: &mut imgui::Ui) {
        input::on_render(ui);

        ui::instance(ui);
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

    fn build_rect(
        draw_list: &DrawListMut<'_>,
        min: [f32; 2],
        max: [f32; 2],
        color: ImColor32,
        rounding: f32,
        flags: DrawFlags,
        filled: bool,
        thickness: Option<f32>,
    ) {
        let mut rect = draw_list
            .add_rect(min, max, color)
            .filled(filled)
            .rounding(rounding);
        if let Some(value) = thickness {
            rect = rect.thickness(value);
        }
        rect = rect
            .round_top_left(flags.contains(DrawFlags::ROUND_CORNERS_TOP_LEFT))
            .round_top_right(flags.contains(DrawFlags::ROUND_CORNERS_TOP_RIGHT))
            .round_bot_left(flags.contains(DrawFlags::ROUND_CORNERS_BOT_LEFT))
            .round_bot_right(flags.contains(DrawFlags::ROUND_CORNERS_BOT_RIGHT));
        rect.build();
    }

    fn stroke_spec(
        filled: bool,
        outline: bool,
        color: ImColor32,
        thickness: f32,
        outline_color: ImColor32,
        outline_thickness: f32,
    ) -> Option<(ImColor32, f32)> {
        if filled {
            outline.then_some((outline_color, outline_thickness))
        } else {
            Some((color, thickness))
        }
    }

    fn draw_quad_fill(
        draw_list: &DrawListMut<'_>,
        p1: [f32; 2],
        p2: [f32; 2],
        p3: [f32; 2],
        p4: [f32; 2],
        color: ImColor32,
    ) {
        draw_list.add_triangle(p1, p2, p3, color).filled(true).build();
        draw_list.add_triangle(p1, p3, p4, color).filled(true).build();
    }

    fn draw_quad_lines(
        draw_list: &DrawListMut<'_>,
        p1: [f32; 2],
        p2: [f32; 2],
        p3: [f32; 2],
        p4: [f32; 2],
        color: ImColor32,
        thickness: f32,
    ) {
        draw_list.add_line(p1, p2, color).thickness(thickness).build();
        draw_list.add_line(p2, p3, color).thickness(thickness).build();
        draw_list.add_line(p3, p4, color).thickness(thickness).build();
        draw_list.add_line(p4, p1, color).thickness(thickness).build();
    }

    fn polyline_points(points: &[[f32; 2]], closed: bool) -> Vec<[f32; 2]> {
        let mut out = Vec::with_capacity(points.len() + usize::from(closed && points.len() > 1));
        out.extend_from_slice(points);
        if closed && points.len() > 1 {
            out.push(points[0]);
        }
        out
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
        defaults: DrawDefaults,
    }

    impl<'ui> DrawContext<'ui> {
        pub fn foreground(ui: &'ui Ui) -> Self {
            Self::new(ui, DrawLayer::Foreground)
        }

        pub fn background(ui: &'ui Ui) -> Self {
            Self::new(ui, DrawLayer::Background)
        }

        pub fn window(ui: &'ui Ui) -> Self {
            Self::new(ui, DrawLayer::Window)
        }

        pub fn new(ui: &'ui Ui, layer: DrawLayer) -> Self {
            Self {
                ui,
                layer,
                defaults: DrawDefaults::default(),
            }
        }

        pub fn defaults(&self) -> &DrawDefaults {
            &self.defaults
        }

        pub fn defaults_mut(&mut self) -> &mut DrawDefaults {
            &mut self.defaults
        }

        pub fn layer(&self) -> DrawLayer {
            self.layer
        }

        pub fn ui(&self) -> &'ui Ui {
            self.ui
        }

        pub fn rect<'ctx>(&'ctx self, min: [f32; 2], max: [f32; 2], color: ImColor32) -> RectCmd<'ctx, 'ui> {
            RectCmd {
                ctx: self,
                min,
                max,
                color,
                filled: true,
                outline: false,
                outline_color: None,
                rounding: None,
                flags: None,
                thickness: None,
                outline_thickness: None,
            }
        }

        pub fn rect_multicolor<'ctx>(
            &'ctx self,
            min: [f32; 2],
            max: [f32; 2],
            top_left: ImColor32,
            top_right: ImColor32,
            bottom_right: ImColor32,
            bottom_left: ImColor32,
        ) -> RectMultiColorCmd<'ctx, 'ui> {
            RectMultiColorCmd {
                ctx: self,
                min,
                max,
                top_left,
                top_right,
                bottom_right,
                bottom_left,
                outline: false,
                outline_color: None,
                rounding: None,
                flags: None,
                outline_thickness: None,
            }
        }

        pub fn circle<'ctx>(&'ctx self, center: [f32; 2], radius: f32, color: ImColor32) -> CircleCmd<'ctx, 'ui> {
            CircleCmd {
                ctx: self,
                center,
                radius,
                color,
                filled: true,
                outline: false,
                outline_color: None,
                thickness: None,
                outline_thickness: None,
                segments: None,
            }
        }

        pub fn arc<'ctx>(
            &'ctx self,
            center: [f32; 2],
            radius: f32,
            min_angle: f32,
            max_angle: f32,
            color: ImColor32,
        ) -> ArcCmd<'ctx, 'ui> {
            ArcCmd {
                ctx: self,
                center,
                radius,
                min_angle,
                max_angle,
                color,
                thickness: None,
                segments: None,
            }
        }

        pub fn line<'ctx>(&'ctx self, p1: [f32; 2], p2: [f32; 2], color: ImColor32) -> LineCmd<'ctx, 'ui> {
            LineCmd {
                ctx: self,
                p1,
                p2,
                color,
                thickness: None,
            }
        }

        pub fn triangle<'ctx>(
            &'ctx self,
            p1: [f32; 2],
            p2: [f32; 2],
            p3: [f32; 2],
            color: ImColor32,
        ) -> TriangleCmd<'ctx, 'ui> {
            TriangleCmd {
                ctx: self,
                p1,
                p2,
                p3,
                color,
                filled: true,
                outline: false,
                outline_color: None,
                thickness: None,
                outline_thickness: None,
            }
        }

        pub fn quad<'ctx>(
            &'ctx self,
            p1: [f32; 2],
            p2: [f32; 2],
            p3: [f32; 2],
            p4: [f32; 2],
            color: ImColor32,
        ) -> QuadCmd<'ctx, 'ui> {
            QuadCmd {
                ctx: self,
                p1,
                p2,
                p3,
                p4,
                color,
                filled: true,
                outline: false,
                outline_color: None,
                thickness: None,
                outline_thickness: None,
            }
        }

        pub fn polygon<'ctx, 'pts>(
            &'ctx self,
            points: &'pts [[f32; 2]],
            color: ImColor32,
        ) -> PolygonCmd<'ctx, 'ui, 'pts> {
            PolygonCmd {
                ctx: self,
                points,
                color,
                filled: true,
                outline: false,
                outline_color: None,
                closed: None,
                thickness: None,
                outline_thickness: None,
                flags: None,
            }
        }

        pub fn text<'ctx, 'txt>(
            &'ctx self,
            pos: [f32; 2],
            text: &'txt str,
            color: ImColor32,
        ) -> TextCmd<'ctx, 'ui, 'txt> {
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

        fn with_draw_list<R>(&self, f: impl FnOnce(&DrawListMut<'_>) -> R) -> R {
            let draw_list = match self.layer {
                DrawLayer::Window => self.ui.get_window_draw_list(),
                DrawLayer::Background => self.ui.get_background_draw_list(),
                DrawLayer::Foreground => self.ui.get_foreground_draw_list(),
            };
            f(&draw_list)
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

        pub fn rounding(mut self, value: f32) -> Self {
            self.rounding = Some(value);
            self
        }

        pub fn flags(mut self, flags: DrawFlags) -> Self {
            self.flags = Some(flags);
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

        pub fn draw(self) {
            let rounding = self.rounding.unwrap_or(self.ctx.defaults.rounding);
            let flags = self.flags.unwrap_or(self.ctx.defaults.flags);
            let thickness = self.thickness.unwrap_or(self.ctx.defaults.thickness);
            let outline_color = self.outline_color.unwrap_or(self.ctx.defaults.outline_color);
            let outline_thickness = self
                .outline_thickness
                .unwrap_or(self.ctx.defaults.outline_thickness);

            self.ctx.with_draw_list(|draw_list| {
                if self.filled {
                    build_rect(
                        draw_list, self.min, self.max, self.color, rounding, flags, true, None,
                    );
                }

                if let Some((stroke_color, stroke_thickness)) = stroke_spec(
                    self.filled,
                    self.outline,
                    self.color,
                    thickness,
                    outline_color,
                    outline_thickness,
                ) {
                    build_rect(
                        draw_list,
                        self.min,
                        self.max,
                        stroke_color,
                        rounding,
                        flags,
                        false,
                        Some(stroke_thickness),
                    );
                }
            });
        }
    }

    pub struct RectMultiColorCmd<'ctx, 'ui> {
        ctx: &'ctx DrawContext<'ui>,
        min: [f32; 2],
        max: [f32; 2],
        top_left: ImColor32,
        top_right: ImColor32,
        bottom_right: ImColor32,
        bottom_left: ImColor32,
        outline: bool,
        outline_color: Option<ImColor32>,
        rounding: Option<f32>,
        flags: Option<DrawFlags>,
        outline_thickness: Option<f32>,
    }

    impl RectMultiColorCmd<'_, '_> {
        pub fn outline(mut self) -> Self {
            self.outline = true;
            self
        }

        pub fn outline_color(mut self, color: ImColor32) -> Self {
            self.outline = true;
            self.outline_color = Some(color);
            self
        }

        pub fn outline_thickness(mut self, value: f32) -> Self {
            self.outline_thickness = Some(value);
            self
        }

        pub fn rounding(mut self, value: f32) -> Self {
            self.rounding = Some(value);
            self
        }

        pub fn flags(mut self, flags: DrawFlags) -> Self {
            self.flags = Some(flags);
            self
        }

        pub fn draw(self) {
            let outline_color = self.outline_color.unwrap_or(self.ctx.defaults.outline_color);
            let outline_thickness = self
                .outline_thickness
                .unwrap_or(self.ctx.defaults.outline_thickness);
            let rounding = self.rounding.unwrap_or(self.ctx.defaults.rounding);
            let flags = self.flags.unwrap_or(self.ctx.defaults.flags);

            self.ctx.with_draw_list(|draw_list| {
                draw_list.add_rect_filled_multicolor(
                    self.min,
                    self.max,
                    self.top_left,
                    self.top_right,
                    self.bottom_right,
                    self.bottom_left,
                );

                if self.outline {
                    build_rect(
                        draw_list,
                        self.min,
                        self.max,
                        outline_color,
                        rounding,
                        flags,
                        false,
                        Some(outline_thickness),
                    );
                }
            });
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

        pub fn segments(mut self, value: u32) -> Self {
            self.segments = Some(value);
            self
        }

        pub fn draw(self) {
            let segments = self.segments.unwrap_or(self.ctx.defaults.segments);
            let thickness = self.thickness.unwrap_or(self.ctx.defaults.thickness);
            let outline_color = self.outline_color.unwrap_or(self.ctx.defaults.outline_color);
            let outline_thickness = self
                .outline_thickness
                .unwrap_or(self.ctx.defaults.outline_thickness);

            self.ctx.with_draw_list(|draw_list| {
                if self.filled {
                    draw_list
                        .add_circle(self.center, self.radius, self.color)
                        .num_segments(segments)
                        .filled(true)
                        .build();
                }

                if let Some((stroke_color, stroke_thickness)) = stroke_spec(
                    self.filled,
                    self.outline,
                    self.color,
                    thickness,
                    outline_color,
                    outline_thickness,
                ) {
                    draw_list
                        .add_circle(self.center, self.radius, stroke_color)
                        .num_segments(segments)
                        .thickness(stroke_thickness)
                        .filled(false)
                        .build();
                }
            });
        }
    }

    pub struct ArcCmd<'ctx, 'ui> {
        ctx: &'ctx DrawContext<'ui>,
        center: [f32; 2],
        radius: f32,
        min_angle: f32,
        max_angle: f32,
        color: ImColor32,
        thickness: Option<f32>,
        segments: Option<u32>,
    }

    impl ArcCmd<'_, '_> {
        pub fn thickness(mut self, value: f32) -> Self {
            self.thickness = Some(value);
            self
        }

        pub fn segments(mut self, value: u32) -> Self {
            self.segments = Some(value);
            self
        }

        pub fn draw(self) {
            let thickness = self.thickness.unwrap_or(self.ctx.defaults.thickness);
            let segments = self
                .segments
                .unwrap_or(self.ctx.defaults.segments)
                .max(2)
                .max(24);

            let mut points = Vec::with_capacity((segments + 1) as usize);
            let span = self.max_angle - self.min_angle;
            let step = span / segments as f32;
            for i in 0..=segments {
                let a = self.min_angle + step * i as f32;
                points.push([
                    self.center[0] + self.radius * a.cos(),
                    self.center[1] + self.radius * a.sin(),
                ]);
            }

            self.ctx.with_draw_list(|draw_list| {
                draw_list
                    .add_polyline(points, self.color)
                    .filled(false)
                    .thickness(thickness)
                    .build();
            });
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
            let thickness = self.thickness.unwrap_or(self.ctx.defaults.thickness);
            self.ctx.with_draw_list(|draw_list| {
                draw_list
                    .add_line(self.p1, self.p2, self.color)
                    .thickness(thickness)
                    .build();
            });
        }
    }

    pub struct TriangleCmd<'ctx, 'ui> {
        ctx: &'ctx DrawContext<'ui>,
        p1: [f32; 2],
        p2: [f32; 2],
        p3: [f32; 2],
        color: ImColor32,
        filled: bool,
        outline: bool,
        outline_color: Option<ImColor32>,
        thickness: Option<f32>,
        outline_thickness: Option<f32>,
    }

    impl TriangleCmd<'_, '_> {
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

        pub fn draw(self) {
            let thickness = self.thickness.unwrap_or(self.ctx.defaults.thickness);
            let outline_color = self.outline_color.unwrap_or(self.ctx.defaults.outline_color);
            let outline_thickness = self
                .outline_thickness
                .unwrap_or(self.ctx.defaults.outline_thickness);

            self.ctx.with_draw_list(|draw_list| {
                if self.filled {
                    draw_list
                        .add_triangle(self.p1, self.p2, self.p3, self.color)
                        .filled(true)
                        .build();
                }

                if let Some((stroke_color, stroke_thickness)) = stroke_spec(
                    self.filled,
                    self.outline,
                    self.color,
                    thickness,
                    outline_color,
                    outline_thickness,
                ) {
                    draw_list
                        .add_triangle(self.p1, self.p2, self.p3, stroke_color)
                        .filled(false)
                        .thickness(stroke_thickness)
                        .build();
                }
            });
        }
    }

    pub struct QuadCmd<'ctx, 'ui> {
        ctx: &'ctx DrawContext<'ui>,
        p1: [f32; 2],
        p2: [f32; 2],
        p3: [f32; 2],
        p4: [f32; 2],
        color: ImColor32,
        filled: bool,
        outline: bool,
        outline_color: Option<ImColor32>,
        thickness: Option<f32>,
        outline_thickness: Option<f32>,
    }

    impl QuadCmd<'_, '_> {
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

        pub fn draw(self) {
            let thickness = self.thickness.unwrap_or(self.ctx.defaults.thickness);
            let outline_color = self.outline_color.unwrap_or(self.ctx.defaults.outline_color);
            let outline_thickness = self
                .outline_thickness
                .unwrap_or(self.ctx.defaults.outline_thickness);

            self.ctx.with_draw_list(|draw_list| {
                if self.filled {
                    draw_quad_fill(draw_list, self.p1, self.p2, self.p3, self.p4, self.color);
                }

                if let Some((stroke_color, stroke_thickness)) = stroke_spec(
                    self.filled,
                    self.outline,
                    self.color,
                    thickness,
                    outline_color,
                    outline_thickness,
                ) {
                    draw_quad_lines(
                        draw_list,
                        self.p1,
                        self.p2,
                        self.p3,
                        self.p4,
                        stroke_color,
                        stroke_thickness,
                    );
                }
            });
        }
    }

    pub struct PolygonCmd<'ctx, 'ui, 'pts> {
        ctx: &'ctx DrawContext<'ui>,
        points: &'pts [[f32; 2]],
        color: ImColor32,
        filled: bool,
        outline: bool,
        outline_color: Option<ImColor32>,
        closed: Option<bool>,
        thickness: Option<f32>,
        outline_thickness: Option<f32>,
        flags: Option<DrawFlags>,
    }

    impl PolygonCmd<'_, '_, '_> {
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

        pub fn closed(mut self, value: bool) -> Self {
            self.closed = Some(value);
            self
        }

        pub fn flags(mut self, flags: DrawFlags) -> Self {
            self.flags = Some(flags);
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

        pub fn draw(self) {
            if self.points.len() < 2 {
                return;
            }

            let thickness = self.thickness.unwrap_or(self.ctx.defaults.thickness);
            let outline_color = self.outline_color.unwrap_or(self.ctx.defaults.outline_color);
            let outline_thickness = self
                .outline_thickness
                .unwrap_or(self.ctx.defaults.outline_thickness);
            let flags = self.flags.unwrap_or(self.ctx.defaults.flags);
            let closed = self.closed.unwrap_or(flags.contains(DrawFlags::CLOSED));

            self.ctx.with_draw_list(|draw_list| {
                if self.filled {
                    draw_list
                        .add_polyline(self.points.to_vec(), self.color)
                        .filled(true)
                        .build();
                }

                if let Some((stroke_color, stroke_thickness)) = stroke_spec(
                    self.filled,
                    self.outline,
                    self.color,
                    thickness,
                    outline_color,
                    outline_thickness,
                ) {
                    draw_list
                        .add_polyline(polyline_points(self.points, closed), stroke_color)
                        .filled(false)
                        .thickness(stroke_thickness)
                        .build();
                }
            });
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
            self.outline = true;
            self.outline_color = Some(color);
            self
        }

        pub fn thickness(mut self, value: f32) -> Self {
            self.outline_thickness = Some(value);
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
            let font = self.font.or(self.ctx.defaults.font);
            let font_size = self.size.or(self.ctx.defaults.font_size);
            let outline_color = self.outline_color.unwrap_or(self.ctx.defaults.outline_color);
            let outline_thickness = self
                .outline_thickness
                .unwrap_or(self.ctx.defaults.outline_thickness)
                .max(0.0);

            let prev_base_size = ui.current_font().font_size.max(f32::EPSILON);
            let prev_scale = ui.current_font_size() / prev_base_size;

            let _font_token = font.map(|id| ui.push_font(id));

            if let Some(target_size) = font_size {
                let base_size = ui.current_font().font_size.max(f32::EPSILON);
                ui.set_window_font_scale((target_size / base_size).max(0.01));
            }

            self.ctx.with_draw_list(|draw_list| {
                if self.outline && outline_thickness > 0.0 {
                    let dirs: &[[f32; 2]] = if outline_thickness <= 1.0 {
                        &TEXT_OUTLINE_DIR4
                    } else {
                        &TEXT_OUTLINE_DIR8
                    };

                    for dir in dirs {
                        draw_list.add_text(
                            [
                                self.pos[0] + dir[0] * outline_thickness,
                                self.pos[1] + dir[1] * outline_thickness,
                            ],
                            outline_color,
                            self.text,
                        );
                    }
                }

                draw_list.add_text(self.pos, self.color, self.text);
            });

            if font_size.is_some() {
                ui.set_window_font_scale(prev_scale.max(0.01));
            }
        }
    }

    #[allow(dead_code)]
    pub fn render_example(ui: &imgui::Ui, maybe_font: Option<FontId>) {
        let mut draw = DrawContext::foreground(ui);
        draw.defaults_mut().outline_color = ImColor32::BLACK;
        draw.defaults_mut().thickness = 1.5;
        draw.defaults_mut().outline_thickness = 1.0;

        draw.rect([40.0, 40.0], [220.0, 120.0], ImColor32::from_rgb(35, 120, 220))
            .outline()
            .rounding(6.0)
            .thickness(2.0)
            .draw();

        draw.circle([320.0, 90.0], 28.0, ImColor32::from_rgb(90, 220, 130))
            .outline_color(ImColor32::BLACK)
            .segments(32)
            .draw();

        let mut text = draw.text([44.0, 46.0], "hello", ImColor32::WHITE).outline();
        if let Some(font) = maybe_font {
            text = text.font(font).size(16.0);
        }
        text.draw();
    }
}
