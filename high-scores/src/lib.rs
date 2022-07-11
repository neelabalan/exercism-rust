#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a [u32],
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        HighScores { scores }
    }
    pub fn scores(&self) -> &[u32] {
        self.scores
    }
    pub fn latest(&self) -> Option<u32> {
        self.scores.last().cloned()
    }
    pub fn personal_best(&self) -> Option<u32> {
        self.scores.iter().max().cloned()
    }
    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut return_vec = self.scores.to_vec();
        return_vec.sort_unstable();
        // sort_unstable_by(|x, y| y.cmp(x));
        return_vec.reverse();
        return_vec.truncate(3);
        return return_vec;
    }
}
