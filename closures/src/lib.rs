#[derive(Debug)]
pub enum ShirtColor{
    RED,BLUE
}
pub struct Inventory{
    pub shirts:Vec<ShirtColor>
}
impl Inventory{
    pub fn most_stocked(&self)->ShirtColor{
        let mut red=0;
        let mut blue=0;
        for shirt in &self.shirts{
            match shirt{
                ShirtColor::RED=>red+=1,
                ShirtColor::BLUE=>blue+=1
            }
        }
        if red>blue {
            return ShirtColor::RED;
        }
        ShirtColor::BLUE
    }
    pub fn giveaway(&self,user_preference:Option<ShirtColor>)->ShirtColor{
        user_preference.unwrap_or_else(|| self.most_stocked())
    }
}