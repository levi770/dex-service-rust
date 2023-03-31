//! # JSON serialize for hex encoded byte arrays (without '0x' prefix)
/// Macro to generate hex serialazable byte arrays
macro_rules! byte_array_struct {
    ($name:ident, $num:expr) => {
        #[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
        ///
        pub struct $name([u8; $num]);

        impl ::std::ops::Deref for $name {
            type Target = [u8];

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl From<[u8; $num]> for $name {
            fn from(bytes: [u8; $num]) -> Self {
                $name(bytes)
            }
        }

        impl Into<[u8; $num]> for $name {
            fn into(self) -> [u8; $num] {
                self.0
            }
        }

        impl<'de> ::serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<$name, D::Error>
            where
                D: ::serde::Deserializer<'de>,
            {
                use hex::FromHex;
                let v = String::deserialize(deserializer)
                    .and_then(|s| Vec::from_hex(s).map_err(::serde::de::Error::custom))?;

                if v.len() != $num {
                    return Err(::serde::de::Error::custom(&format!(
                        "Byte array invalid length: {}",
                        v.len()
                    )));
                }

                let mut bytes = [0u8; $num];
                bytes.copy_from_slice(&v);

                Ok($name(bytes))
            }
        }

        impl ::serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                use hex;
                serializer.serialize_str(&hex::encode(&self.0))
            }
        }
        //
        //        impl ::rustc_serialize::Decodable for $name {
        //            fn decode<D: ::rustc_serialize::Decoder>(d: &mut D) -> Result<$name, D::Error> {
        //                use hex::FromHex;
        //                let v = d.read_str()
        //                    .and_then(|s| Vec::from_hex(s).map_err(|e| d.error(&e.to_string())))?;
        //
        //                if v.len() != $num {
        //                    return Err(d.error(&format!("Byte array invalid length: {}", v.len())));
        //                }
        //
        //                let mut bytes = [0u8; $num];
        //
        //                bytes.copy_from_slice(&v);
        //
        //                Ok($name(bytes))
        //            }
        //        }
        //
        //        impl ::rustc_serialize::Encodable for $name {
        //            fn encode<S: ::rustc_serialize::Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        //                s.emit_str(&self.0.to_hex())
        //            }
        //        }
    };
}