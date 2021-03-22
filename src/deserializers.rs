pub mod origin {
    use serde::{de, ser, Deserialize, Deserializer, Serializer};

    fn integer_to_str<S>(input: &i32) -> Result<&str, S::Error>
        where
            S: Serializer,
    {
        if *input == 0 {
            Ok("p2p")
        } else if *input == 1 {
            Ok("radio")
        } else {
            Err(ser::Error::custom(
                "Invalid integer for serializing origin field",
            ))
        }
    }

    pub fn serialize<S>(input: &i32, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let s = integer_to_str::<S>(input)?;
        serializer.serialize_str(s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<i32, D::Error>
        where
            D: Deserializer<'de>,
    {
        let s = <&str>::deserialize(deserializer)?;
        if s == "p2p" {
            Ok(0)
        } else if s == "radio" {
            Ok(1)
        } else {
            Err(de::Error::custom(
                "Invalid string for deserializing origin field",
            ))
        }
    }
}

pub mod reward_type {
    use serde::{de, ser, Deserialize, Deserializer, Serializer};

    fn integer_to_str<S>(input: &i32) -> Result<&str, S::Error>
        where
            S: Serializer,
    {
        if *input == 0 {
            Ok("securities")
        } else if *input == 1 {
            Ok("data_credits")
        } else if *input == 2 {
            Ok("poc_challengees")
        } else if *input == 3 {
            Ok("poc_challengers")
        } else if *input == 4 {
            Ok("poc_witnesses")
        } else if *input == 5 {
            Ok("consensus")
        } else {
            Err(ser::Error::custom(
                "Invalid integer for serializing origin field",
            ))
        }
    }

    pub fn serialize<S>(input: &i32, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let s = integer_to_str::<S>(input)?;
        serializer.serialize_str(s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<i32, D::Error>
        where
            D: Deserializer<'de>,
    {
        let s = <&str>::deserialize(deserializer)?;
        if s == "securities" {
            Ok(0)
        } else if s == "data_credits" {
            Ok(1)
        } else if s == "poc_challengees" {
            Ok(2)
        } else if s == "poc_challengers" {
            Ok(3)
        } else if s == "poc_witnesses" {
            Ok(4)
        } else if s == "consensus" {
            Ok(5)
        } else {
            Err(de::Error::custom(
                "Invalid string for deserializing origin field",
            ))
        }
    }
}

pub mod base58 {
    extern crate bs58;
    use serde::{de, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(bytes: &[u8], serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_str(&bs58::encode(bytes).into_string())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
        where
            D: Deserializer<'de>,
    {
        let opt: Option<&str> = Option::deserialize(deserializer)?;
        if let Some(s) = opt {
            bs58::decode(s).into_vec().map_err(de::Error::custom)
        } else {
            Ok(Vec::new())
        }
    }
}

pub mod multikeys {
    extern crate bs58;
    use serde::{
        de::{SeqAccess, Visitor},
        Deserializer, Serializer,
    };
    use std::fmt;

    pub fn serialize<S>(keys: &[Vec<u8>], serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let mut s = String::new();
        s.push('[');
        for (i, key) in keys.iter().enumerate() {
            let mut new_key = key.clone();
            new_key.insert(0, 0);
            s.push('"');
            s.push_str(&bs58::encode(&new_key).with_check().into_string());
            s.push('"');
            if i != keys.len() - 1 {
                s.push_str(", ");
            }
        }
        s.push(']');
        serializer.serialize_str(&s)
    }

    // warning: this part isn't really tested
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<Vec<u8>>, D::Error>
        where
            D: Deserializer<'de>,
    {
        struct KeyParser;
        impl<'de> Visitor<'de> for KeyParser {
            type Value = Vec<Vec<u8>>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("Vec<Vec<u8>>")
            }

            fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
                let mut ret = Vec::new();

                while let Some(s) = seq.next_element::<String>()? {
                    ret.push(bs58::decode(s).into_vec().unwrap());
                }
                Ok(ret)
            }
        }
        deserializer.deserialize_any(KeyParser {})
    }
}

pub mod str_list {
    use serde::{
        Deserializer, Serializer,
    };

    pub fn serialize<S>(keys: &[Vec<u8>], serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let mut s = String::new();
        s.push('[');
        for (i, key) in keys.iter().enumerate() {
            // TODO: should find how to map into err
            s.push('"');
            s.push_str(&std::str::from_utf8(key).unwrap());
            s.push('"');
            if i != keys.len() - 1 {
                s.push_str(", ");
            }
        }
        s.push(']');
        serializer.serialize_str(&s)
    }

    // warning: this part isn't really tested
    pub fn deserialize<'de, D>(_deserializer: D) -> Result<Vec<Vec<u8>>, D::Error>
        where
            D: Deserializer<'de>,
    {
        panic!("var::deserialize untested")
    }
}

pub mod base64_url_list {
    extern crate base64;
    use serde::{Deserializer, Serializer};


    pub fn serialize<S>(keys: &[Vec<u8>], serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let mut s = String::new();
        s.push('[');
        for (i, bytes) in keys.iter().enumerate() {
            // TODO: should find how to map into err
            s.push('"');
            s.push_str(&base64::encode_config(bytes, base64::URL_SAFE));
            s.push('"');
            if i != keys.len() - 1 {
                s.push_str(", ");
            }
        }
        s.push(']');
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(_deserializer: D) -> Result<Vec<Vec<u8>>, D::Error>
        where
            D: Deserializer<'de>,
    {
        panic!("base64_url_list deserialize unimplemented")
    }
}

pub mod base64 {
    extern crate base64;
    use serde::{de, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(bytes: &[u8], serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_str(&base64::encode(bytes))
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
        where
            D: Deserializer<'de>,
    {
        let s = <&str>::deserialize(deserializer)?;
        base64::decode(s).map_err(de::Error::custom)
    }
}

pub mod base64_url {
    extern crate base64;
    use serde::{de, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(bytes: &[u8], serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        if bytes.is_empty() {
            serializer.serialize_str("null")
        } else {
            serializer.serialize_str(&base64::encode_config(bytes, base64::URL_SAFE))

        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
        where
            D: Deserializer<'de>,
    {
        let s = <&str>::deserialize(deserializer)?;
        base64::decode_config(s, base64::URL_SAFE).map_err(de::Error::custom)
    }
}

pub mod u64_base64 {
    extern crate base64;
    use byteorder::{BigEndian, ByteOrder};
    use serde::{de, Deserialize, Deserializer, Serializer};
    use byteorder::WriteBytesExt;

    pub fn serialize<S>(word: &u64, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let mut vec = Vec::new();
        match vec.write_u64::<BigEndian>(*word) {
            Ok(()) => serializer.serialize_str(&base64::encode(vec)),
            // TODO: better error handling
            Err(_) => panic!("Error serializing u64 to base64"),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<u64, D::Error>
        where
            D: Deserializer<'de>,
    {
        let s = <&str>::deserialize(deserializer)?;
        match base64::decode(s) {
            Ok(vec) => Ok(BigEndian::read_u64(&vec)),
            Err(e) => Err(de::Error::custom(e)),
        }
    }
}
