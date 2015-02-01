use {
    Buffer2d,
    Rect,
    Packer,
};

pub struct GuillotinePacker<B: Buffer2d> {
    width: u32,
    height: u32,
    buf: B,
    free_areas: Vec<Rect>,
    margin: u32,
}

impl<B: Buffer2d> GuillotinePacker<B> {
    // Best Area Fit
    fn find_free_area(&self, w: u32, h: u32) -> Option<(usize, Rect)> {
        let mut index = None;
        let mut min_area = None;
        let mut rect = Rect::new(0, 0, 0, 0);

        for i in 0 .. self.free_areas.len() {
            let ref area = self.free_areas[i];
            let a = area.area();

            if w <= area.w && h <= area.h {
                if min_area.is_none() || a < min_area.unwrap() {
                    index = Some(i);
                    min_area = Some(a);
                    rect.x = area.x;
                    rect.y = area.y;
                    rect.w = w;
                    rect.h = h;
                }
            } else if h <= area.w && w <= area.h {
                if min_area.is_none() || a < min_area.unwrap() {
                    index = Some(i);
                    min_area = Some(a);
                    rect.x = area.x;
                    rect.y = area.y;
                    rect.w = h;
                    rect.h = w;
                }
            }
        }

        index.map(|i| (i, rect))
    }

    // Shorter Axis Split
    fn split(&mut self, index: usize, rect: &Rect) {
        let area = self.free_areas.remove(index);

        // Split horizontally
        if area.w < area.h {
            self.free_areas.push(Rect {
                x: area.x + rect.w,
                y: area.y,
                w: area.w - rect.w,
                h: rect.h,
            });

            self.free_areas.push(Rect {
                x: area.x,
                y: area.y + rect.h,
                w: area.w,
                h: area.h - rect.h,
            });
        }
        // Split vertically
        else {
            self.free_areas.push(Rect {
                x: area.x,
                y: area.y + rect.h,
                w: rect.w,
                h: area.h - rect.h,
            });

            self.free_areas.push(Rect {
                x: area.x + rect.w,
                y: area.y,
                w: area.w - rect.w,
                h: area.h,
            });
        }
    }
}

impl<B: Buffer2d> Packer for GuillotinePacker<B> {
    type Buffer = B;

    fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    fn set_dimensions(&mut self, w: u32, h: u32) {
        self.width = w;
        self.height = h;
    }

    fn new(buf: B) -> GuillotinePacker<B> {
        let (width, height) = buf.dimensions();
        let mut free_areas = Vec::new();
        free_areas.push(Rect {
            x: 0,
            y: 0,
            w: width,
            h: height,
        });

        GuillotinePacker {
            buf: buf,
            free_areas: free_areas,
            margin: 0,
            width: width,
            height: height
        }
    }

    fn pack<O: Buffer2d<Pixel=B::Pixel>>(&mut self, buf: &O) -> Option<Rect> {
        let (mut width, mut height) = buf.dimensions();
        width += self.margin;
        height += self.margin;
        if let Some((i, mut rect)) = self.find_free_area(width, height) {
            if width == rect.w {
                self.buf.patch(rect.x, rect.y, buf);
            } else {
                self.buf.patch_rotated(rect.x, rect.y, buf);
            }

            self.split(i, &rect);

            rect.w -= self.margin;
            rect.h -= self.margin;
            Some(rect)
        } else { None }
    }

    fn buf(&self) -> &B {
        &self.buf
    }

    fn buf_mut(&mut self) -> &mut B {
        &mut self.buf
    }

    fn into_buf(self) -> B {
        self.buf
    }

    fn set_margin(&mut self, val: u32) {
        self.margin = val;
    }
}

