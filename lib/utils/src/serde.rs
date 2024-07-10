pub mod jsontime {
    #[derive(Clone, Debug)]
    pub struct UnixMilliString(pub time::OffsetDateTime);

    impl serde::Serialize for UnixMilliString {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            let milli_time_int = self.0.unix_timestamp_nanos() / 1_000_000;
            serializer.serialize_str(milli_time_int.to_string().as_str())
        }
    }

    impl<'de> serde::Deserialize<'de> for UnixMilliString {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let milli_time_string = String::deserialize(deserializer)?;
            let milli_time_int: i128 = milli_time_string
                .parse()
                .map_err(serde::de::Error::custom)?;

            let parsed_time =
                time::OffsetDateTime::from_unix_timestamp_nanos(milli_time_int * 1_000_000)
                    .map_err(serde::de::Error::custom)?;

            Ok(UnixMilliString(parsed_time))
        }
    }

    #[derive(Clone, Debug)]
    pub struct UnixMicroString(pub time::OffsetDateTime);

    impl serde::Serialize for UnixMicroString {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            let micro_time_int = self.0.unix_timestamp_nanos() / 1_000;
            serializer.serialize_str(micro_time_int.to_string().as_str())
        }
    }

    impl<'de> serde::Deserialize<'de> for UnixMicroString {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let micro_time_string = String::deserialize(deserializer)?;
            let micro_time_int: i128 = micro_time_string
                .parse()
                .map_err(serde::de::Error::custom)?;

            let parsed_time =
                time::OffsetDateTime::from_unix_timestamp_nanos(micro_time_int * 1_000)
                    .map_err(serde::de::Error::custom)?;

            Ok(UnixMicroString(parsed_time))
        }
    }

    #[derive(Clone, Debug)]
    pub struct UnixNanoString(pub time::OffsetDateTime);

    impl serde::Serialize for UnixNanoString {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            let nano_time_int = self.0.unix_timestamp_nanos();
            serializer.serialize_str(nano_time_int.to_string().as_str())
        }
    }

    impl<'de> serde::Deserialize<'de> for UnixNanoString {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let nano_time_string = String::deserialize(deserializer)?;
            let nano_time_int: i128 = nano_time_string.parse().map_err(serde::de::Error::custom)?;

            let parsed_time = time::OffsetDateTime::from_unix_timestamp_nanos(nano_time_int)
                .map_err(serde::de::Error::custom)?;

            Ok(UnixNanoString(parsed_time))
        }
    }

    #[derive(Clone, Debug)]
    pub struct UnixString(pub time::OffsetDateTime);

    impl serde::Serialize for UnixString {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            let time_int = self.0.unix_timestamp();
            serializer.serialize_str(time_int.to_string().as_str())
        }
    }

    impl<'de> serde::Deserialize<'de> for UnixString {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let time_string = String::deserialize(deserializer)?;
            let time_int: i64 = time_string.parse().map_err(serde::de::Error::custom)?;

            let parsed_time = time::OffsetDateTime::from_unix_timestamp(time_int)
                .map_err(serde::de::Error::custom)?;

            Ok(UnixString(parsed_time))
        }
    }
}
