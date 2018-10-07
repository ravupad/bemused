use crate::model::task::Task;
use serde::de::{self, Deserialize, Deserializer, MapAccess, Visitor};
use serde::ser::{Serialize, SerializeStruct, Serializer};
use std::fmt;

const FIELDS: &[&str] = &[
    "user_id",
    "id",
    "text",
    "note",
    "category",
    "schedule_time",
    "schedule_interval_value",
    "schedule_interval_type",
    "completed",
];

enum Field {
    UserId,
    Id,
    Text,
    Note,
    Category,
    ScheduleTime,
    ScheduleIntervalValue,
    ScheduleIntervalType,
    Completed,
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
            "user_id" => Ok(Field::UserId),
            "id" => Ok(Field::Id),
            "text" => Ok(Field::Text),
            "note" => Ok(Field::Note),
            "category" => Ok(Field::Category),
            "schedule_time" => Ok(Field::ScheduleTime),
            "schedule_interval_value" => Ok(Field::ScheduleIntervalValue),
            "schedule_interval_type" => Ok(Field::ScheduleIntervalType),
            "completed" => Ok(Field::Completed),
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

struct TaskVisitor;

impl<'de> Visitor<'de> for TaskVisitor {
    type Value = Task;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("task struct")
    }

    fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
    where
        V: MapAccess<'de>,
    {
        let mut user_id = None;
        let mut id = None;
        let mut text = None;
        let mut note = None;
        let mut category = None;
        let mut schedule_time = None;
        let mut schedule_interval_value = None;
        let mut schedule_interval_type = None;
        let mut completed = None;
        while let Some(key) = map.next_key()? {
            match key {
                Field::UserId => user_id = Some(map.next_value()?),
                Field::Id => id = Some(map.next_value()?),
                Field::Text => text = Some(map.next_value()?),
                Field::Note => note = Some(map.next_value()?),
                Field::Category => category = Some(map.next_value()?),
                Field::ScheduleTime => {
                    schedule_time = Some(
                        map.next_value::<String>()?
                            .parse()
                            .map_err(de::Error::custom)?,
                    )
                }
                Field::ScheduleIntervalValue => schedule_interval_value = Some(map.next_value()?),
                Field::ScheduleIntervalType => schedule_interval_type = Some(map.next_value()?),
                Field::Completed => completed = Some(map.next_value()?),
            }
        }
        let user_id = user_id.ok_or_else(|| de::Error::missing_field("user_id"))?;
        let id = id.ok_or_else(|| de::Error::missing_field("id"))?;
        let text = text.ok_or_else(|| de::Error::missing_field("text"))?;
        let note = note.ok_or_else(|| de::Error::missing_field("note"))?;
        let category = category.ok_or_else(|| de::Error::missing_field("category"))?;
        let schedule_time =
            schedule_time.ok_or_else(|| de::Error::missing_field("schedule_time"))?;
        let schedule_interval_value = schedule_interval_value
            .ok_or_else(|| de::Error::missing_field("schedule_interval_value"))?;
        let schedule_interval_type = schedule_interval_type
            .ok_or_else(|| de::Error::missing_field("schedule_interval_type"))?;
        let completed = completed.ok_or_else(|| de::Error::missing_field("completed"))?;
        Ok(Task {
            id,
            user_id,
            text,
            note,
            category,
            schedule_time,
            schedule_interval_value,
            schedule_interval_type,
            completed,
        })
    }
}

impl<'de> Deserialize<'de> for Task {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_struct("Task", FIELDS, TaskVisitor)
    }
}

impl Serialize for Task {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Task", 9)?;
        state.serialize_field("user_id", &self.user_id)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("text", &self.text)?;
        state.serialize_field("note", &self.note)?;
        state.serialize_field("category", &self.category)?;
        state.serialize_field("schedule_time", &self.schedule_time.to_rfc3339())?;
        state.serialize_field("schedule_interval_value", &self.schedule_interval_value)?;
        state.serialize_field("schedule_interval_type", &self.schedule_interval_type)?;
        state.serialize_field("completed", &self.completed)?;
        state.end()
    }
}
