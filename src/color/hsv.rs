// Copyright 2013 The Lmath Developers. For a full listing of the authors,
// refer to the AUTHORS file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::num;
use std::cast;

use color::Channel;
use color::{RGB, ToRGB, RGBA, ToRGBA};

#[path = "../num_macros.rs"]
mod num_macros;

#[deriving(Clone, Eq)]
pub struct HSV<T> { h: T, s: T, v: T }

impl<T:Channel + Float> HSV<T> {
    pub fn new(h: T, s: T, v: T) -> HSV<T> {
        HSV { h: h, s: s, v: v }
    }
}

pub trait ToHSV {
    pub fn to_hsv<U:Channel + Float>(&self) -> HSV<U>;
}

impl<T:Clone + Channel + Float> ToHSV for HSV<T> {
    #[inline]
    pub fn to_hsv<U:Channel + Float>(&self) -> HSV<U> {
        HSV::new((*self).h.to_channel(),
                 (*self).s.to_channel(),
                 (*self).v.to_channel())
    }
}

impl<T:Clone + Channel + Float> ToRGB for HSV<T> {
    pub fn to_rgb<U:Channel>(&self) -> RGB<U> {
        // Algorithm taken from the Wikipedia article on HSL and HSV:
        // http://en.wikipedia.org/wiki/HSL_and_HSV#From_HSV

        let chr = (*self).v * (*self).s;
        let h = (*self).h / num::cast(60);

        // the 2nd largest component
        let x = chr * (one!(T) - ((h % two!(T)) - one!(T)).abs());

        let mut rgb = cond! (
            (h < num::cast(1)) { RGB::new(chr.clone(), x, zero!(T)) }
            (h < num::cast(2)) { RGB::new(x, chr.clone(), zero!(T)) }
            (h < num::cast(3)) { RGB::new(zero!(T), chr.clone(), x) }
            (h < num::cast(4)) { RGB::new(zero!(T), x, chr.clone()) }
            (h < num::cast(5)) { RGB::new(x, zero!(T), chr.clone()) }
            (h < num::cast(6)) { RGB::new(chr.clone(), zero!(T), x) }
            _                  { RGB::new(zero!(T), zero!(T), zero!(T)) }
        );

        // match the value by adding the same amount to each component
        let mn = (*self).v - chr;

        rgb.r = rgb.r + mn;
        rgb.g = rgb.g + mn;
        rgb.b = rgb.b + mn;

        rgb.to_rgb::<U>()
    }
}

#[deriving(Clone, Eq)]
pub struct HSVA<T> { h: T, s: T, v: T, a: T }

impl<T:Channel + Float> HSVA<T> {
    #[inline]
    pub fn new(h: T, s: T, v: T, a: T) -> HSVA<T> {
        HSVA { h: h, s: s, v: v, a: a }
    }

    #[inline]
    pub fn from_hsv_a(hsv: HSV<T>, a: T) -> HSVA<T> {
        unsafe { cast::transmute((hsv, a)) }
    }

    #[inline]
    pub fn hsv<'a>(&'a self) -> &'a HSV<T> {
        unsafe { cast::transmute(self) }
    }

    #[inline]
    pub fn hsv_mut<'a>(&'a mut self) -> &'a mut HSV<T> {
        unsafe { cast::transmute(self) }
    }
}

pub trait ToHSVA {
    pub fn to_hsva<U:Channel + Float>(&self) -> HSVA<U>;
}

impl<C: ToHSV, T:Clone + Channel + Float> ToHSVA for (C, T) {
    #[inline]
    pub fn to_hsva<U:Channel + Float>(&self) -> HSVA<U> {
        match *self {
            (ref hsv, ref a) =>  {
                HSVA::from_hsv_a(hsv.to_hsv(), a.to_channel())
            }
        }
    }
}

impl<T:Clone + Channel + Float> ToRGBA for HSVA<T> {
    #[inline]
    pub fn to_rgba<U:Channel>(&self) -> RGBA<U> {
        RGBA::from_rgb_a(self.hsv().to_rgb(), (*self).a.to_channel())
    }
}
