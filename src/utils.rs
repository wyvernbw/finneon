pub trait Step {
    fn step(self, edge: Self) -> Self;
    fn smoothstep(self, edge0: Self, edge1: Self) -> Self;
}

impl Step for f32 {
    fn step(self, edge: Self) -> Self {
        if self < edge {
            0.0
        } else {
            1.0
        }
    }
    fn smoothstep(self, edge0: Self, edge1: Self) -> Self {
        let t = ((self - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
        t * t * (3.0 - 2.0 * t)
    }
}

impl Step for glam::Vec2 {
    fn step(self, edge: Self) -> Self {
        glam::Vec2::new(self.x.step(edge.x), self.y.step(edge.y))
    }
    fn smoothstep(self, edge0: Self, edge1: Self) -> Self {
        glam::Vec2::new(
            self.x.smoothstep(edge0.x, edge1.x),
            self.y.smoothstep(edge0.y, edge1.y),
        )
    }
}

impl Step for glam::Vec3 {
    fn step(self, edge: Self) -> Self {
        glam::Vec3::new(
            self.x.step(edge.x),
            self.y.step(edge.y),
            self.z.step(edge.z),
        )
    }
    fn smoothstep(self, edge0: Self, edge1: Self) -> Self {
        glam::Vec3::new(
            self.x.smoothstep(edge0.x, edge1.x),
            self.y.smoothstep(edge0.y, edge1.y),
            self.z.smoothstep(edge0.z, edge1.z),
        )
    }
}

impl Step for glam::Vec4 {
    fn step(self, edge: Self) -> Self {
        glam::Vec4::new(
            self.x.step(edge.x),
            self.y.step(edge.y),
            self.z.step(edge.z),
            self.w.step(edge.w),
        )
    }
    fn smoothstep(self, edge0: Self, edge1: Self) -> Self {
        glam::Vec4::new(
            self.x.smoothstep(edge0.x, edge1.x),
            self.y.smoothstep(edge0.y, edge1.y),
            self.z.smoothstep(edge0.z, edge1.z),
            self.w.smoothstep(edge0.w, edge1.w),
        )
    }
}
