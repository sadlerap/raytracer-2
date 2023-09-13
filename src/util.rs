#[derive(Debug)]
pub struct Range<Idx> {
    pub start: Idx,
    pub end: Idx,
}

impl<Idx> From<std::ops::Range<Idx>> for Range<Idx> {
    fn from(value: std::ops::Range<Idx>) -> Self {
        Range {
            start: value.start,
            end: value.end,
        }
    }
}

impl<Idx: Clone> Clone for Range<Idx> {
    fn clone(&self) -> Self {
        Self {
            start: self.start.clone(),
            end: self.end.clone(),
        }
    }
}

impl<Idx: Clone + Copy> Copy for Range<Idx> {}

impl<Idx> Range<Idx>
where
    Idx: PartialEq + PartialOrd,
{
    pub const fn new(start: Idx, end: Idx) -> Self {
        Self { start, end }
    }

    #[allow(dead_code)]
    pub fn contains(&self, value: Idx) -> bool {
        self.start <= value && value <= self.end
    }

    pub fn clamp(self, value: Idx) -> Idx {
        if value < self.start {
            self.start
        } else if value > self.end {
            self.end
        } else {
            value
        }
    }
}
