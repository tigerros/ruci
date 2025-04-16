macro_rules! impl_message {
    ($name:ident $(<$lt:lifetime>)?) => {
        impl<'a> super::traits::sealed::Message for &'a $name $(<$lt>)? {}
        impl<'a> super::traits::Message for &'a $name $(<$lt>)? {}
    };
    
    (copy $name:ident $(<$lt:lifetime>)?) => {
        impl super::traits::sealed::Message for $name $(<$lt>)? {}
        impl super::traits::Message for $name $(<$lt>)? {}
        impl<'a> super::traits::sealed::Message for &'a $name $(<$lt>)? {}
        impl<'a> super::traits::Message for &'a $name $(<$lt>)? {}
    };
}

pub(crate) use impl_message;