//! This module contains the implementation of the arithmetic operations for the `FrameData` struct.

use super::FrameData;
use super::MonoFrameData;
use crate::MonoPixel;
use rgb::Gray;

impl<T> MonoFrameData<T>
where
    T: crate::MonoPixel,
{
    pub fn zeros(width: u32, height: u32) -> FrameData<Gray<T>> {
        FrameData::<Gray<T>> {
            width,
            height,
            data: vec![Gray::<T>::new(T::zero()); (width * height) as usize],
        }
    }

    pub fn ones(width: u32, height: u32) -> FrameData<Gray<T>> {
        FrameData::<Gray<T>> {
            width,
            height,
            data: vec![Gray::<T>::new(T::one()); (width * height) as usize],
        }
    }
}

impl<T> std::ops::Shl<usize> for &MonoFrameData<T>
where
    T: crate::MonoPixel,
{
    type Output = MonoFrameData<T>;

    fn shl(self, shift: usize) -> MonoFrameData<T> {
        MonoFrameData::<T> {
            width: self.width,
            height: self.height,
            data: self
                .data
                .iter()
                .map(|a| Gray::<T>::new(a.0 << shift))
                .collect(),
        }
    }
}

impl<T> std::ops::Shr<usize> for &MonoFrameData<T>
where
    T: MonoPixel,
{
    type Output = MonoFrameData<T>;

    fn shr(self, shift: usize) -> MonoFrameData<T> {
        MonoFrameData::<T> {
            width: self.width,
            height: self.height,
            data: self
                .data
                .iter()
                .map(|a| Gray::<T>::new(a.0 >> shift))
                .collect(),
        }
    }
}

impl<T> std::ops::ShlAssign<usize> for MonoFrameData<T>
where
    T: MonoPixel,
{
    fn shl_assign(&mut self, shift: usize) {
        self.data
            .iter_mut()
            .for_each(|a| *a = Gray::<T>::new(a.0 << shift));
    }
}

impl<T> std::ops::ShrAssign<usize> for MonoFrameData<T>
where
    T: MonoPixel,
{
    fn shr_assign(&mut self, shift: usize) {
        self.data
            .iter_mut()
            .for_each(|a| *a = Gray::<T>::new(a.0 >> shift));
    }
}

impl<T, T2> std::ops::Add<&FrameData<Gray<T2>>> for &FrameData<Gray<T>>
where
    T: MonoPixel,
    T2: MonoPixel,
{
    type Output = FrameData<Gray<T>>;

    fn add(self, other: &FrameData<Gray<T2>>) -> FrameData<Gray<T>> {
        assert_eq!(self.width, other.width);
        assert_eq!(self.height, other.height);

        FrameData::<Gray<T>> {
            width: self.width,
            height: self.height,
            data: self
                .data
                .iter()
                .zip(other.data.iter())
                .map(|(a, b)| *a + T::from(b.0).unwrap())
                .collect(),
        }
    }
}

impl<T, T2> std::ops::AddAssign<&MonoFrameData<T2>> for MonoFrameData<T>
where
    T: MonoPixel,
    T2: MonoPixel,
{
    fn add_assign(&mut self, other: &MonoFrameData<T2>) {
        assert_eq!(self.width, other.width);
        assert_eq!(self.height, other.height);

        self.data
            .iter_mut()
            .zip(other.data.iter())
            .for_each(|(a, b)| *a = *a + T::from(b.0).unwrap());
    }
}

impl<T, T2> std::ops::Sub<&MonoFrameData<T2>> for &MonoFrameData<T>
where
    T: MonoPixel,
    T2: MonoPixel,
{
    type Output = MonoFrameData<T>;

    fn sub(self, other: &MonoFrameData<T2>) -> MonoFrameData<T> {
        assert_eq!(self.width, other.width);
        assert_eq!(self.height, other.height);

        MonoFrameData::<T> {
            width: self.width,
            height: self.height,
            data: self
                .data
                .iter()
                .zip(other.data.iter())
                .map(|(a, b)| *a - T::from(b.0).unwrap())
                .collect(),
        }
    }
}

impl<T, T2> std::ops::SubAssign<&MonoFrameData<T2>> for MonoFrameData<T>
where
    T: MonoPixel,
    T2: MonoPixel,
{
    fn sub_assign(&mut self, other: &MonoFrameData<T2>) {
        assert_eq!(self.width, other.width);
        assert_eq!(self.height, other.height);

        self.data
            .iter_mut()
            .zip(other.data.iter())
            .for_each(|(a, b)| *a = *a - T::from(b.0).unwrap());
    }
}

impl<T, T2> std::ops::Mul<T2> for &MonoFrameData<T>
where
    T: MonoPixel,
    T2: MonoPixel,
{
    type Output = MonoFrameData<T>;
    fn mul(self, other: T2) -> MonoFrameData<T> {
        let othert = T::from(other).unwrap();
        MonoFrameData::<T> {
            width: self.width,
            height: self.height,
            data: self.data.iter().map(|a| *a * othert).collect(),
        }
    }
}

impl<T> std::ops::Div<T> for &MonoFrameData<T>
where
    T: MonoPixel,
{
    type Output = MonoFrameData<T>;
    fn div(self, other: T) -> MonoFrameData<T> {
        let othert = T::from(other).unwrap();
        MonoFrameData::<T> {
            width: self.width,
            height: self.height,
            data: self.data.iter().map(|a| *a / othert).collect(),
        }
    }
}

impl<T> std::ops::Mul<&MonoFrameData<T>> for &MonoFrameData<T>
where
    T: MonoPixel,
{
    type Output = MonoFrameData<T>;

    fn mul(self, other: &MonoFrameData<T>) -> MonoFrameData<T> {
        assert_eq!(self.width, other.width);
        assert_eq!(self.height, other.height);

        MonoFrameData::<T> {
            width: self.width,
            height: self.height,
            data: self
                .data
                .iter()
                .zip(other.data.iter())
                .map(|(a, b)| *a * *b)
                .collect(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_zeros() {
        let frame = MonoFrameData::<u16>::zeros(3, 3);
        assert_eq!(frame.width, 3);
        assert_eq!(frame.height, 3);
        //assert_eq!(frame.data, vec![0; 9].iter().map(|x| Gray::<u16>::new(x)));
    }

    #[test]
    fn test_ones() {
        let frame = MonoFrameData::<u16>::ones(3, 3);
        assert_eq!(frame.width, 3);
        assert_eq!(frame.height, 3);
        //assert_eq!(frame.data, vec![1; 9]);
    }

    #[test]
    fn test_add() {
        let frame1 = MonoFrameData::<u16>::ones(3, 3);
        let frame2 = MonoFrameData::<u16>::ones(3, 3);
        let frame3 = &frame1 + &frame2;
        assert_eq!(frame3.width, 3);
        assert_eq!(frame3.height, 3);
        //assert_eq!(frame3.data, vec![2; 9]);
    }

    #[test]
    fn test_add_types() {
        let frame1 = MonoFrameData::<u16>::ones(3, 3);
        let frame2 = MonoFrameData::<u8>::ones(3, 3);
        let frame3 = &frame1 + &frame2;
        assert_eq!(frame3.width, 3);
        assert_eq!(frame3.height, 3);
        //assert_eq!(frame3.data, vec![2; 9]);
    }
}
