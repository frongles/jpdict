
#[derive(Debug)]
pub struct JMdict {
    pub entry : Vec<Entry>
}

#[derive(Debug, Default)]
pub struct Entry {
    pub ent_seq : i64,
    pub k_ele   : Vec<Kanji>,
    pub r_ele   : Vec<Reading>,
    pub sense   : Vec<Sense>,
}

#[derive(Debug, Default)]
pub struct Kanji{
    pub keb     : String,
    pub ke_inf   : Vec<String>,
    pub ke_pri  : Vec<String>,
}


#[derive(Debug, Default)]
pub struct Reading {
    pub reb     : String,
    pub r_inf   : Vec<String>,
    pub re_pri   : Vec<String>,
    pub re_restr: Vec<String>,
    pub re_nokanji: String,
}

#[derive(Debug, Default)]
pub struct Sense {
    pub stagk   : Vec<String>,
    pub stagr   : Vec<String>,
    pub pos     : Vec<String>,
    pub x_ref   : Vec<String>,
    pub ant     : Vec<String>,
    pub field   : Vec<String>,
    pub misc    : Vec<String>,
    pub s_inf   : Vec<String>,
    pub lsource : Vec<String>,
    pub dial    : Vec<String>,
    pub gloss   : Vec<String>,
    pub example : Vec<String>,
}
