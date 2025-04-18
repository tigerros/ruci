macro_rules! impl_message {
    ($name:ident $(<$lt:lifetime>)?) => {
        impl traits::sealed::Message for $name $(<$lt>)? {}
        impl traits::Message for $name $(<$lt>)? {}
        impl traits::sealed::Message for &'_ $name $(<$lt>)? {}
        impl traits::Message for &'_ $name $(<$lt>)? {}
        impl $crate::traits::sealed::Message for $name $(<$lt>)? {}
        impl $crate::traits::Message for $name $(<$lt>)? {}
        impl $crate::traits::sealed::Message for &'_ $name $(<$lt>)? {}
        impl $crate::traits::Message for &'_ $name $(<$lt>)? {}
    };
}

pub(crate) use impl_message;