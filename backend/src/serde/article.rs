use crate::model::article::Article;
use serde::de::{self, Deserialize, Deserializer, MapAccess, Visitor};
use serde::ser::{Serialize, SerializeStruct, Serializer};
use std::fmt;

const FIELDS: &[&str] = &["id", "user_id", "title", "text", "tags"];

enum Field {
    Id,
    UserId,
    Title,
    Text,
    Tags,
}

struct FieldVisitor;

impl<'de> Visitor<'de> for FieldVisitor {
    type Value = Field;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("fields for task struct")
    }
    fn visit_str<E>(self, value: &str) -> Result<Field, E>
    where
        E: de::Error,
    {
        match value {
            "id" => Ok(Field::Id),
            "user_id" => Ok(Field::UserId),
            "title" => Ok(Field::Title),
            "text" => Ok(Field::Text),
            "tags" => Ok(Field::Tags),
            _ => Err(de::Error::unknown_field(value, FIELDS)),
        }
    }
}

impl<'de> Deserialize<'de> for Field {
    fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_identifier(FieldVisitor)
    }
}

struct ArticleVisitor;

impl<'de> Visitor<'de> for ArticleVisitor {
    type Value = Article;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("task struct")
    }

    fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
    where
        V: MapAccess<'de>,
    {
        let mut user_id = None;
        let mut id = None;
        let mut title = None;
        let mut text = None;
        let mut tags = None;
        while let Some(key) = map.next_key()? {
            match key {
                Field::Id => id = Some(map.next_value()?),
                Field::UserId => user_id = Some(map.next_value()?),
                Field::Title => title = Some(map.next_value()?),
                Field::Text => text = Some(map.next_value()?),
                Field::Tags => tags = Some(map.next_value()?),
            }
        }
        let id = id.ok_or_else(|| de::Error::missing_field("id"))?;
        let user_id = user_id.ok_or_else(|| de::Error::missing_field("user_id"))?;
        let title = title.ok_or_else(|| de::Error::missing_field("title"))?;
        let text = text.ok_or_else(|| de::Error::missing_field("text"))?;
        let tags = tags.ok_or_else(|| de::Error::missing_field("tags"))?;
        Ok(Article {
            id,
            user_id,
            title,
            text,
            tags,
        })
    }
}

impl<'de> Deserialize<'de> for Article {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_struct("Article", FIELDS, ArticleVisitor)
    }
}

impl Serialize for Article {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Article", 5)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("user_id", &self.user_id)?;
        state.serialize_field("title", &self.title)?;
        state.serialize_field("text", &self.text)?;
        state.serialize_field("tags", &self.tags)?;
        state.end()
    }
}
