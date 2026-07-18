//! Domain enums stored as TEXT in PostgreSQL.
//!
//! The `string_enum!` macro generates, for each enum:
//!   * serde `Serialize`/`Deserialize` (snake_case)
//!   * `Display` / `FromStr`
//!   * SQLx `Type`/`Encode`/`Decode` for Postgres so the enum maps transparently
//!     to/from a TEXT column — no parallel "row" structs required.

macro_rules! string_enum {
    (
        $(#[$meta:meta])*
        $name:ident { $($variant:ident => $s:literal),+ $(,)? }
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
        #[serde(rename_all = "snake_case")]
        pub enum $name {
            $($variant),+
        }

        impl $name {
            pub fn as_str(&self) -> &'static str {
                match self { $(Self::$variant => $s),+ }
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(self.as_str())
            }
        }

        impl std::str::FromStr for $name {
            type Err = crate::error::AppError;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $($s => Ok(Self::$variant),)+
                    other => Err(crate::error::AppError::Validation(format!(
                        concat!("invalid ", stringify!($name), ": '{}'"), other
                    ))),
                }
            }
        }

        impl sqlx::Type<sqlx::Postgres> for $name {
            fn type_info() -> sqlx::postgres::PgTypeInfo {
                <str as sqlx::Type<sqlx::Postgres>>::type_info()
            }
            fn compatible(ty: &sqlx::postgres::PgTypeInfo) -> bool {
                <str as sqlx::Type<sqlx::Postgres>>::compatible(ty)
            }
        }

        impl<'r> sqlx::Decode<'r, sqlx::Postgres> for $name {
            fn decode(
                value: sqlx::postgres::PgValueRef<'r>,
            ) -> Result<Self, sqlx::error::BoxDynError> {
                let s = <&str as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
                s.parse().map_err(|e: crate::error::AppError| {
                    Box::new(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        e.to_string(),
                    )) as sqlx::error::BoxDynError
                })
            }
        }

        impl<'q> sqlx::Encode<'q, sqlx::Postgres> for $name {
            fn encode_by_ref(
                &self,
                buf: &mut sqlx::postgres::PgArgumentBuffer,
            ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
                <&str as sqlx::Encode<sqlx::Postgres>>::encode(self.as_str(), buf)
            }
        }
    };
}

string_enum! {
    /// Athlete's self-reported experience level; drives training volume & intensity.
    ExperienceLevel {
        Beginner => "beginner",
        Intermediate => "intermediate",
        Advanced => "advanced",
    }
}

string_enum! {
    /// Supported triathlon race distances.
    RaceDistance {
        Sprint => "sprint",
        Olympic => "olympic",
        HalfIronman => "half_ironman",
        Ironman => "ironman",
    }
}

string_enum! {
    Discipline {
        Swim => "swim",
        Bike => "bike",
        Run => "run",
        Brick => "brick",
        Strength => "strength",
        Rest => "rest",
    }
}

string_enum! {
    Intensity {
        Recovery => "recovery",
        Endurance => "endurance",
        Tempo => "tempo",
        Threshold => "threshold",
        Vo2max => "vo2max",
    }
}

string_enum! {
    WorkoutStatus {
        Scheduled => "scheduled",
        Completed => "completed",
        Skipped => "skipped",
    }
}

string_enum! {
    PlanStatus {
        Active => "active",
        Archived => "archived",
    }
}

string_enum! {
    GeneratedBy {
        Gemini => "gemini",
        RuleBased => "rule_based",
    }
}

string_enum! {
    Platform {
        Ios => "ios",
        Android => "android",
    }
}

impl RaceDistance {
    /// A sensible minimum number of training weeks for this distance.
    #[allow(dead_code)] // surfaced by the client for onboarding guidance
    pub fn recommended_weeks(&self) -> i32 {
        match self {
            RaceDistance::Sprint => 8,
            RaceDistance::Olympic => 12,
            RaceDistance::HalfIronman => 16,
            RaceDistance::Ironman => 24,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            RaceDistance::Sprint => "Sprint",
            RaceDistance::Olympic => "Olympic",
            RaceDistance::HalfIronman => "Half Ironman (70.3)",
            RaceDistance::Ironman => "Ironman (140.6)",
        }
    }
}
