//! If serde is enabled we need to have the ability to serialize and deserialize all objects in the library.

use serde::de::{self, MapAccess, SeqAccess, Visitor};
use serde::ser::SerializeStruct;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::marker::PhantomData;

use crate::shapes::{Cuboid, Sphere};
use crate::Container;
use PackedVolume;

impl Serialize for Sphere {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Sphere", 2)?;
        state.serialize_field("center", &self.center)?;
        state.serialize_field("radius", &self.radius)?;
        state.end()
    }
}

impl Serialize for Cuboid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Cuboid", 1)?;
        state.serialize_field("half_extents", &self.half_extents)?;
        state.end()
    }
}

impl<C: Container + Serialize> Serialize for PackedVolume<C> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("PackedVolume", 2)?;
        state.serialize_field("spheres", &self.spheres)?;
        state.serialize_field("container", &self.container)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for Sphere {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field {
            Center,
            Radius,
        };

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`center` or `radius`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "center" => Ok(Field::Center),
                            "radius" => Ok(Field::Radius),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct SphereVisitor;

        impl<'de> Visitor<'de> for SphereVisitor {
            type Value = Sphere;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Sphere")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Sphere, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let center = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let radius = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                Ok(Sphere::new(center, radius).map_err(de::Error::custom)?)
            }

            fn visit_map<V>(self, mut map: V) -> Result<Sphere, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut center = None;
                let mut radius = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Center => {
                            if center.is_some() {
                                return Err(de::Error::duplicate_field("center"));
                            }
                            center = Some(map.next_value()?);
                        }
                        Field::Radius => {
                            if radius.is_some() {
                                return Err(de::Error::duplicate_field("radius"));
                            }
                            radius = Some(map.next_value()?);
                        }
                    }
                }
                let center = center.ok_or_else(|| de::Error::missing_field("center"))?;
                let radius = radius.ok_or_else(|| de::Error::missing_field("radius"))?;
                Ok(Sphere::new(center, radius).map_err(de::Error::custom)?)
            }
        }

        const FIELDS: &'static [&'static str] = &["center", "radius"];
        deserializer.deserialize_struct("Sphere", FIELDS, SphereVisitor)
    }
}

impl<'de> Deserialize<'de> for Cuboid {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field {
            HalfExtents,
        };

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`half_extents`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "half_extents" => Ok(Field::HalfExtents),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct CuboidVisitor;

        impl<'de> Visitor<'de> for CuboidVisitor {
            type Value = Cuboid;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Cuboid")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Cuboid, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let half_extents = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                Ok(Cuboid::from_vec(half_extents).map_err(de::Error::custom)?)
            }

            fn visit_map<V>(self, mut map: V) -> Result<Cuboid, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut half_extents = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::HalfExtents => {
                            if half_extents.is_some() {
                                return Err(de::Error::duplicate_field("half_extents"));
                            }
                            half_extents = Some(map.next_value()?);
                        }
                    }
                }
                let half_extents =
                    half_extents.ok_or_else(|| de::Error::missing_field("half_extents"))?;
                Ok(Cuboid::from_vec(half_extents).map_err(de::Error::custom)?)
            }
        }

        const FIELDS: &'static [&'static str] = &["half_extents"];
        deserializer.deserialize_struct("Cuboid", FIELDS, CuboidVisitor)
    }
}

impl<'de, C: Container + Deserialize<'de>> Deserialize<'de> for PackedVolume<C> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field {
            Spheres,
            Container,
        };

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`spheres` or `container`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "spheres" => Ok(Field::Spheres),
                            "container" => Ok(Field::Container),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct PackedVolumeVisitor<P>(PhantomData<fn() -> P>);

        impl<'de, P> Visitor<'de> for PackedVolumeVisitor<P>
        where
            P: Deserialize<'de> + Container,
        {
            type Value = PackedVolume<P>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct PackedVolume")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<PackedVolume<P>, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let spheres = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let container = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                Ok(PackedVolume::from_vec(spheres, container))
            }

            fn visit_map<V>(self, mut map: V) -> Result<PackedVolume<P>, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut spheres = None;
                let mut container = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Spheres => {
                            if spheres.is_some() {
                                return Err(de::Error::duplicate_field("spheres"));
                            }
                            spheres = Some(map.next_value()?);
                        }
                        Field::Container => {
                            if container.is_some() {
                                return Err(de::Error::duplicate_field("container"));
                            }
                            container = Some(map.next_value()?);
                        }
                    }
                }
                let spheres = spheres.ok_or_else(|| de::Error::missing_field("spheres"))?;
                let container = container.ok_or_else(|| de::Error::missing_field("container"))?;
                Ok(PackedVolume::from_vec(spheres, container))
            }
        }

        const FIELDS: &'static [&'static str] = &["spheres", "container"];
        deserializer.deserialize_struct("PackedVolume", FIELDS, PackedVolumeVisitor(PhantomData))
    }
}
