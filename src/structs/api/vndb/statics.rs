pub static VNDB_EMBED_COLOR: &str = "#225588";
pub static VNDB_EMBED_AUTHOR_URL: &str = "https://vndb.org";
pub static VNDB_EMBED_AUTHOR_ICON_URL: &str = "https://i.ibb.co/FktgZQ0v/vndb.png";
pub static VNDB_VISUAL_NOVEL_FIELDS: &str = "id,title,alttitle,titles{lang,title,latin,official,main},aliases,olang,devstatus,released,languages,platforms,image{id,url,dims,sexual,violence,votecount},length,length_minutes,length_votes,description,rating,popularity,votecount,tags{rating,spoiler,lie,id,name,aliases,description,category,searchable,applicable,vn_count}";
pub static VNDB_CHARACTER_FIELDS: &str = "id,name,original,aliases,description,image{id,url,dims,sexual,violence,votecount},blood_type,height,weight,bust,waist,hips,cup,age,birthday,sex,vns{id,title,role},traits{id,name,aliases,description,searchable,applicable,group_id,group_name,char_count,spoiler,lie}";
pub static VNDB_TAG_FIELDS: &str = "id,name,aliases,description,category,searchable,applicable,vn_count";
pub static VNDB_TRAIT_FIELDS: &str = "id,name,aliases,description,searchable,applicable,group_id,group_name,char_count";
