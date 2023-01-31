use chrono::{DateTime, Utc};

use super::schema::*;

#[derive(Queryable)]
pub struct DbChannel {
    // Internal data about the channel
    pub id: i64,
    pub remote: String,
    pub slug: String,
    
    // Data specified by the channel
    pub d_title: String,
    pub d_link: String,
    pub d_description: String,
    pub d_categories: Vec<String>,
    pub d_image: Option<String>,
    pub d_author: Option<String>,
    pub d_subtitle: Option<String>,

    // Cached media
    pub c_image: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name=channels)]
pub struct InsChannel {
    // Internal data about the channel
    pub remote: String,
    pub slug: String,
    
    // Data specified by the channel
    pub d_title: String,
    pub d_link: String,
    pub d_description: String,
    pub d_categories: Vec<String>,
    pub d_image: Option<String>,
    pub d_author: Option<String>,
    pub d_subtitle: Option<String>,

    // Cached media
    pub c_image: Option<String>,
}

#[derive(Queryable)]
pub struct DbItem {
    // Internal data about the item
    pub id: i64,
    pub channel: i64,
    pub has_enclosure: bool,
    pub discovered: DateTime<Utc>,

    // Data specified by the item
    pub d_title: Option<String>,
    pub d_link: Option<String>,
    pub d_description: Option<String>,
    pub d_author: Option<String>,
    pub d_categories: Vec<String>,
    pub d_encl_url: Option<String>,
    pub d_encl_length: Option<String>,
    pub d_encl_mime_type: Option<String>,
    pub d_guid: Option<String>,
    pub d_pub_date: Option<DateTime<Utc>>,
    pub d_content: Option<String>,
    pub d_image: Option<String>,
    pub d_duration: Option<i32>,

    // Cached media
    pub c_encl_url: Option<String>,
    pub c_image: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name=items)]
pub struct InsItem {
    // Internal data about the item
    pub channel: i64,
    pub has_enclosure: bool,
    pub discovered: DateTime<Utc>,

    // Data specified by the item
    pub d_title: Option<String>,
    pub d_link: Option<String>,
    pub d_description: Option<String>,
    pub d_author: Option<String>,
    pub d_categories: Vec<String>,
    pub d_encl_url: Option<String>,
    pub d_encl_length: Option<String>,
    pub d_encl_mime_type: Option<String>,
    pub d_guid: Option<String>,
    pub d_pub_date: Option<DateTime<Utc>>,
    pub d_content: Option<String>,
    pub d_image: Option<String>,
    pub d_duration: Option<i32>,

    // Cached media
    pub c_encl_url: Option<String>,
    pub c_image: Option<String>,
}
