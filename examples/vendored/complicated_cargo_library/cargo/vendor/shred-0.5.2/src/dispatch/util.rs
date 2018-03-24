pub fn check_intersection<'i, 'j, T, I, J>(mut i: I, j: J) -> bool
where
    I: Iterator<Item = &'i T>,
    J: Iterator<Item = &'j T> + Clone,
    T: PartialEq + 'i + 'j,
{
    i.any(|elem_i| j.clone().any(|elem_j| *elem_j == *elem_i))
}
