prpr::tl_file!("tags");

use crate::page::Fader;
use macroquad::prelude::*;
use prpr::{
    ext::{semi_black, RectExt},
    scene::{request_input, return_input, show_message, take_input},
    ui::{DRectButton, Scroll, Ui},
};

pub struct Tags {
    input_id: &'static str,
    pub tags: Vec<String>,
    btns: Vec<DRectButton>,
    add: DRectButton,
}

impl Tags {
    pub fn new(input_id: &'static str) -> Self {
        Self {
            input_id,
            tags: Vec::new(),
            btns: Vec::new(),
            add: DRectButton::new(),
        }
    }

    pub fn add(&mut self, s: String) {
        self.tags.push(s);
        self.btns.push(DRectButton::new());
    }

    pub fn set(&mut self, tags: Vec<String>) {
        self.btns = vec![DRectButton::new(); tags.len()];
        self.tags = tags;
    }

    pub fn touch(&mut self, touch: &Touch, t: f32) -> bool {
        for (index, btn) in self.btns.iter_mut().enumerate() {
            if btn.touch(touch, t) {
                self.tags.remove(index);
                self.btns.remove(index);
                return true;
            }
        }
        if self.add.touch(touch, t) {
            request_input(self.input_id, "");
            return true;
        }
        false
    }

    pub fn render(&mut self, ui: &mut Ui, mw: f32, t: f32, alpha: f32) -> f32 {
        let row_height = 0.1;
        let tmw = 0.3;
        let sz = 0.5;
        let margin = 0.03;
        let pad = 0.01;

        let mut h = 0.;
        let mut x = 0.;
        let mut draw = |btn: &mut DRectButton, text: &str| {
            let w = ui.text(text).size(sz).measure().w.clamp(0.08, tmw);
            if x + w + (margin + pad) * 2. > mw {
                x = 0.;
                h += row_height;
            }
            btn.render_text(ui, Rect::new(x, h, w + (margin + pad) * 2., row_height).feather(-pad), t, alpha, text, sz, true);
            x += w + (margin + pad) * 2.;
        };
        for (tag, btn) in self.tags.iter().zip(self.btns.iter_mut()) {
            draw(btn, tag);
        }
        draw(&mut self.add, "+");
        h + row_height
    }

    pub fn try_add(&mut self, s: &str) {
        if !s.chars().all(|it| it == '-' || it.is_alphanumeric()) {
            show_message(tl!("invalid-tag")).error();
            return;
        }
        if self.tags.iter().all(|it| it != s) {
            self.add(s.into());
        }
    }
}

pub struct TagsDialog {
    fader: Fader,
    show: bool,

    scroll: Scroll,
    pub tags: Tags,
    pub unwanted: Option<Tags>,

    btn_cancel: DRectButton,
    btn_confirm: DRectButton,
    pub confirmed: Option<bool>,
}

impl TagsDialog {
    pub fn new(search_mode: bool) -> Self {
        Self {
            fader: Fader::new().with_distance(-0.4).with_time(0.5),
            show: false,

            scroll: Scroll::new(),
            tags: Tags::new("add_tag"),
            unwanted: if search_mode { Some(Tags::new("add_tag_unwanted")) } else { None },

            btn_cancel: DRectButton::new(),
            btn_confirm: DRectButton::new(),
            confirmed: None,
        }
    }

    pub fn showing(&self) -> bool {
        self.show
    }

    pub fn enter(&mut self, t: f32) {
        self.fader.sub(t);
    }

    pub fn dismiss(&mut self, t: f32) {
        self.show = false;
        self.fader.back(t);
    }

    pub fn touch(&mut self, touch: &Touch, t: f32) -> bool {
        if self.fader.transiting() {
            return true;
        }
        if self.show {
            if !Ui::dialog_rect().contains(touch.position) {
                self.dismiss(t);
                return true;
            }
            if self.scroll.touch(touch, t) {
                return true;
            }
            if self.tags.touch(touch, t) {
                self.scroll.y_scroller.halt();
                return true;
            }
            if let Some(unwanted) = &mut self.unwanted {
                if unwanted.touch(touch, t) {
                    self.scroll.y_scroller.halt();
                    return true;
                }
            }
            if self.btn_cancel.touch(touch, t) {
                self.confirmed = Some(false);
                self.dismiss(t);
                return true;
            }
            if self.btn_confirm.touch(touch, t) {
                self.confirmed = Some(true);
                self.dismiss(t);
                return true;
            }
            return true;
        }
        false
    }

    pub fn update(&mut self, t: f32) {
        if let Some(done) = self.fader.done(t) {
            self.show = !done;
        }
        self.scroll.update(t);
        if let Some((id, text)) = take_input() {
            match id.as_str() {
                "add_tag" => {
                    self.tags.try_add(text.trim());
                }
                "add_tag_unwanted" => {
                    self.unwanted.as_mut().unwrap().try_add(text.trim());
                }
                _ => {
                    return_input(id, text);
                }
            }
        }
    }

    pub fn render(&mut self, ui: &mut Ui, t: f32) {
        self.fader.reset();
        if self.show || self.fader.transiting() {
            let p = if self.show { 1. } else { -self.fader.progress(t) };
            ui.fill_rect(ui.screen_rect(), semi_black(p * 0.7));
            self.fader.for_sub(|f| {
                f.render(ui, t, |ui, c| {
                    let wr = Ui::dialog_rect();
                    ui.fill_path(&wr.rounded(0.02), Color { a: c.a, ..ui.background() });
                    ui.scissor(Some(wr));
                    let r = ui
                        .text(if self.unwanted.is_some() { tl!("filter") } else { tl!("edit") })
                        .pos(wr.x + 0.04, wr.y + 0.033)
                        .size(0.9)
                        .color(c)
                        .draw();
                    let mw = wr.w - 0.02;
                    let bh = 0.09;
                    ui.scope(|ui| {
                        ui.dx(r.x);
                        ui.dy(r.bottom() + 0.02);
                        self.scroll.size((mw, wr.h - r.y - 0.04 - if self.unwanted.is_some() { 0. } else { bh }));
                        self.scroll.render(ui, |ui| {
                            let mut h = 0.;
                            if self.unwanted.is_some() {
                                let th = ui.text(tl!("wanted")).size(0.5).color(c).draw().h + 0.01;
                                ui.dy(th);
                                h += th;
                            }
                            let th = self.tags.render(ui, mw, t, c.a);
                            ui.dy(th);
                            h += th;
                            if let Some(unwanted) = &mut self.unwanted {
                                ui.dy(0.02);
                                h += 0.02;
                                let th = ui.text(tl!("unwanted")).size(0.5).color(c).draw().h + 0.01;
                                ui.dy(th);
                                h += th;
                                h += unwanted.render(ui, mw, t, c.a);
                            }
                            (mw, h)
                        });
                    });
                    if self.unwanted.is_none() {
                        let pad = 0.02;
                        let bw = (wr.w - pad * 3.) / 2.;
                        let mut r = Rect::new(wr.x + pad, wr.bottom() - 0.02 - bh, bw, bh);
                        self.btn_cancel.render_text(ui, r, t, c.a, tl!("cancel"), 0.5, true);
                        r.x += bw + pad;
                        self.btn_confirm.render_text(ui, r, t, c.a, tl!("confirm"), 0.5, true);
                    }
                });
            });
        }
    }
}
