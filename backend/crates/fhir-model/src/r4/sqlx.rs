use crate::r4::generated::resources::ResourceType;
use sqlx::{
    Database, Decode, Encode, Postgres,
    encode::IsNull,
    error::BoxDynError,
    postgres::{PgArgumentBuffer, PgTypeInfo, PgValueRef},
};
use std::io::Write;

/// FHIR resource wrapper for SQLx JSONB serialization.
///
/// This type implements SQLx trait bounds to enable automatic serialization and
/// deserialization of FHIR resources to/from PostgreSQL JSONB columns.
///
/// # Generic Parameters
/// * `T` - The FHIR type to serialize/deserialize (typically a `Resource` enum variant)
///
/// # Example
/// ```ignore
/// let resource: FHIRJson<Patient> = FHIRJson(patient_data);
/// // Can now be used with sqlx query parameters
/// ```
#[derive(Debug, Clone)]
pub struct FHIRJson<T: ?Sized>(pub T);

impl<T> sqlx::Type<Postgres> for FHIRJson<T>
where
    T: serde::Serialize + serde::de::DeserializeOwned,
{
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("jsonb")
    }

    fn compatible(ty: &PgTypeInfo) -> bool {
        *ty == PgTypeInfo::with_name("json") || *ty == PgTypeInfo::with_name("jsonb")
    }
}

/// Decoder implementation for FHIR resources from PostgreSQL JSONB.
///
/// Handles the PostgreSQL JSONB binary format by stripping the first byte marker
/// and deserializing the remaining JSON data.
impl<'r, T: 'r> Decode<'r, Postgres> for FHIRJson<T>
where
    T: serde::Serialize + serde::Deserialize<'r>,
{
    fn decode(value: PgValueRef<'r>) -> Result<Self, BoxDynError> {
        let buf = value.as_bytes()?;
        // Need to remove first byte which is a marker for JSONB binary.
        let resource = serde_json::from_slice::<T>(&buf[1..]);
        Ok(FHIRJson::<T>(resource?))
    }
}

// More effecient impl to avoid cloning the value. No need to own as writing bytes and non mutating.

/// Reference wrapper for FHIR resources optimized for SQLx encoding.
///
/// Similar to [`FHIRJson`] but holds a reference instead of ownership. This avoids
/// cloning when encoding FHIR resources to PostgreSQL JSONB columns, improving
/// performance for repeated queries.
///
/// # Generic Parameters
/// * `'a` - The lifetime of the reference to the FHIR type
/// * `T` - The FHIR type to serialize (typically a `Resource` enum variant)
///
/// # Example
/// ```ignore
/// let resource: FHIRJsonRef<Patient> = FHIRJsonRef(&patient_data);
/// // Can now be used with sqlx query parameters without cloning
/// ```
pub struct FHIRJsonRef<'a, T: ?Sized>(pub &'a T);

/// SQLx type information for FHIR resource references.
///
/// Specifies that this type maps to PostgreSQL JSONB binary format.
impl<'a, T> sqlx::Type<Postgres> for FHIRJsonRef<'a, T>
where
    T: serde::Serialize + serde::de::DeserializeOwned,
{
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("jsonb")
    }

    fn compatible(ty: &PgTypeInfo) -> bool {
        *ty == PgTypeInfo::with_name("json") || *ty == PgTypeInfo::with_name("jsonb")
    }
}

/// Encoder implementation for FHIR resource references to PostgreSQL JSONB.
///
/// Serializes the referenced FHIR resource to JSON and writes it to the PostgreSQL
/// buffer with the JSONB format marker. This implementation avoids cloning for
/// better performance in repeated query operations.
impl<'q, T> Encode<'q, Postgres> for FHIRJsonRef<'q, T>
where
    T: serde::Serialize + serde::de::DeserializeOwned,
{
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> Result<IsNull, BoxDynError> {
        // we have a tiny amount of dynamic behavior depending if we are resolved to be JSON
        // instead of JSONB

        // buf.patch(|buf, ty: &PgTypeInfo| {
        //     if *ty == PgTypeInfo::JSON || *ty == PgTypeInfo::JSON_ARRAY {
        //         buf[0] = b' ';
        //     }
        // });

        // JSONB version (as of 2020-03-20)
        buf.push(1);

        // the JSON data written to the buffer is the same regardless of parameter type
        serde_json::to_writer(&mut **buf, &*self.0)?;

        Ok(IsNull::No)
    }
}

/// Decoder for ResourceType enum from database strings.
///
/// Converts PostgreSQL string values into FHIR ResourceType enum variants,
/// supporting any database backend that implements string decoding.
impl<'r, DB: Database> Decode<'r, DB> for ResourceType
where
    &'r str: Decode<'r, DB>,
{
    fn decode(
        value: <DB as Database>::ValueRef<'r>,
    ) -> Result<ResourceType, Box<dyn std::error::Error + 'static + Send + Sync>> {
        let value = <&str as Decode<DB>>::decode(value)?;
        Ok(ResourceType::try_from(value).unwrap())
    }
}

impl<'r> Encode<'r, Postgres> for ResourceType {
    fn encode_by_ref(
        &self,
        buf: &mut PgArgumentBuffer,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        buf.write(self.as_ref().as_bytes())?;
        Ok(sqlx::encode::IsNull::No)
    }
}
