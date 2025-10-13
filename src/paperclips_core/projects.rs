use crate::PaperClips;

pub struct Project {
    /// # id
    id: String,
    /// # title
    title: String,
    /// # description
    description: String,
    /// # uses
    uses: u8,
    /// # priceTag
    requirements: (String, Box<dyn Fn(&mut PaperClips) -> bool>),
    /// # trigger
    trigger: Box<dyn Fn(&mut PaperClips) -> bool>,
    /// # effect
    /// what it should run when bought
    effect: Box<dyn Fn(&mut PaperClips)>,
    /// # flag
    /// "is enabled"
    flag: bool,
}
