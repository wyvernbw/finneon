use image::{DynamicImage, GenericImageView};

pub struct Context<'a, U> {
    pub(crate) app: &'a crate::App<U>,
    pub(crate) image: &'a DynamicImage,
    pub(crate) fragcoord: glam::UVec2,
    pub(crate) fragcolor: glam::Vec4,
}
trait FromContext<'a, U> {
    fn from_context(ctx: &'a Context<'a, U>) -> Self;
}

pub struct Fragcoord(pub glam::UVec2);

impl<'a, U> FromContext<'a, U> for Fragcoord {
    fn from_context(ctx: &'a Context<'a, U>) -> Self {
        Fragcoord(ctx.fragcoord)
    }
}

pub struct Resolution(pub glam::Vec2);

impl<'a, U> FromContext<'a, U> for Resolution {
    fn from_context(ctx: &'a Context<'a, U>) -> Self {
        let res = ctx.image.dimensions();
        Resolution(glam::Vec2::new(res.0 as f32, res.1 as f32))
    }
}

pub struct Uv(pub glam::Vec2);

impl<'a, U> FromContext<'a, U> for Uv {
    fn from_context(ctx: &'a Context<'a, U>) -> Self {
        let Resolution(res) = Resolution::from_context(ctx);
        let fragcoord = glam::Vec2::new(ctx.fragcoord.x as f32, ctx.fragcoord.y as f32);
        let uv = fragcoord / res;
        Uv(uv)
    }
}

pub struct Uniforms<U>(pub U)
where
    U: Clone;

impl<'a, U> FromContext<'a, U> for Uniforms<U>
where
    U: Clone,
{
    fn from_context(ctx: &'a Context<'a, U>) -> Self {
        Uniforms(ctx.app.uniforms.clone())
    }
}

pub struct FragColor(pub glam::Vec4);

impl<'a, U> FromContext<'a, U> for FragColor {
    fn from_context(ctx: &'a Context<'a, U>) -> Self {
        FragColor(ctx.fragcolor)
    }
}

pub trait Handler<T, U> {
    fn handle(&self, ctx: &Context<'_, U>) -> glam::Vec4;
}

impl<T, U> Handler<T, U> for () {
    fn handle(&self, _: &Context<'_, U>) -> glam::Vec4 {
        glam::Vec4::new(0.0, 0.0, 0.0, 1.0)
    }
}

impl<T, U, F> Handler<(T,), U> for F
where
    F: Fn(T) -> glam::Vec4,
    T: for<'a> FromContext<'a, U>,
{
    fn handle(&self, ctx: &Context<'_, U>) -> glam::Vec4 {
        self(T::from_context(ctx))
    }
}

impl<T1, T2, U, F> Handler<(T1, T2), U> for F
where
    F: Fn(T1, T2) -> glam::Vec4,
    T1: for<'a> FromContext<'a, U>,
    T2: for<'a> FromContext<'a, U>,
{
    fn handle(&self, ctx: &Context<'_, U>) -> glam::Vec4 {
        self(T1::from_context(ctx), T2::from_context(ctx))
    }
}

macro_rules! impl_handler {
    ($($name:ident),*) => {
        impl<$($name,)* U, F> Handler<($($name,)*), U> for F
        where
            F: Fn($($name),*) -> glam::Vec4,
            $($name: for<'a> FromContext<'a, U>,)*
        {
            fn handle(&self, ctx: &Context<'_, U>) -> glam::Vec4 {
                self($($name::from_context(ctx),)*)
            }
        }
    };
}

impl_handler!(T1, T2, T3);
impl_handler!(T1, T2, T3, T4);
impl_handler!(T1, T2, T3, T4, T5);
impl_handler!(T1, T2, T3, T4, T5, T6);
impl_handler!(T1, T2, T3, T4, T5, T6, T7);
impl_handler!(T1, T2, T3, T4, T5, T6, T7, T8);
impl_handler!(T1, T2, T3, T4, T5, T6, T7, T8, T9);
