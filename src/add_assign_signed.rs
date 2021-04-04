pub trait AddAssignSigned<Rhs = Self> {
    fn add_assign_signed(&mut self, rhs: Rhs);
}

impl AddAssignSigned<i64> for u32 {
    fn add_assign_signed(&mut self, other: i64) {
        if other.is_negative() {
            *self -= other.abs() as u32;
        } else {
            *self += other as u32;
        }
    }
}
