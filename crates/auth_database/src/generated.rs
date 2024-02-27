// !
// ! This file is generated.
// ! Do not modify it manually.
// ! Regenerate it by running `cargo build`.
// !

pub mod activity {
    use database_toolkit::Table;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct Activity;

    impl Table for Activity {
        const NAME: &'static str = "activity";
    }

    pub type PrimaryKey = crate::identity::PrimaryKey<Activity>;

    pub type Identity = crate::identity::Identity<Activity>;

    impl identity::Prefix for Activity {
        const PREFIX: &'static str = "activity_";
    }

    pub type Column = database_toolkit::Column<Activity>;

    pub const ALL_COLUMNS: [Column; 3] = [
        columns::ACTIVITY_PK,
        columns::ID,
        columns::NAME,
    ];

    pub mod columns {
        pub const ACTIVITY_PK: super::Column = super::Column::new("activity_pk");
        pub const ID: super::Column = super::Column::new("id");
        pub const NAME: super::Column = super::Column::new("name");

        pub type ActivityPrimaryKey = crate::generated::activity::PrimaryKey;

        pub type ActivityIdentity = super::Identity;

        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
        pub struct ActivityName(String);

        impl std::ops::Deref for ActivityName {
            type Target = str;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl<'de> serde::Deserialize<'de> for ActivityName {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let source = String::deserialize(deserializer)?;
                if source.len() < 1 {
                    return Err(serde::de::Error::custom("too short"));
                }
                if source.len() > 64 {
                    return Err(serde::de::Error::custom("too long"));
                }
                Ok(Self(source))
            }
        }

        impl From<String> for ActivityName {
            #[inline]
            fn from(source: String) -> Self {
                Self(source)
            }
        }

        impl From<&str> for ActivityName {
            #[inline]
            fn from(source: &str) -> Self {
                Self(source.to_owned())
            }
        }

        impl ActivityName {
            #[inline]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }
    }
}

pub mod agreement {
    use database_toolkit::Table;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct Agreement;

    impl Table for Agreement {
        const NAME: &'static str = "agreement";
    }

    pub type PrimaryKey = crate::identity::PrimaryKey<Agreement>;

    pub type Identity = crate::identity::Identity<Agreement>;

    impl identity::Prefix for Agreement {
        const PREFIX: &'static str = "agreement_";
    }

    pub type Column = database_toolkit::Column<Agreement>;

    pub const ALL_COLUMNS: [Column; 4] = [
        columns::AGREEMENT_PK,
        columns::ID,
        columns::NAME,
        columns::POLICY_PK,
    ];

    pub mod columns {
        pub const AGREEMENT_PK: super::Column = super::Column::new("agreement_pk");
        pub const ID: super::Column = super::Column::new("id");
        pub const NAME: super::Column = super::Column::new("name");
        pub const POLICY_PK: super::Column = super::Column::new("policy_pk");

        pub type AgreementPrimaryKey = crate::generated::agreement::PrimaryKey;

        pub type AgreementIdentity = super::Identity;

        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
        pub struct AgreementName(String);

        impl std::ops::Deref for AgreementName {
            type Target = str;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl<'de> serde::Deserialize<'de> for AgreementName {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let source = String::deserialize(deserializer)?;
                if source.len() < 1 {
                    return Err(serde::de::Error::custom("too short"));
                }
                if source.len() > 64 {
                    return Err(serde::de::Error::custom("too long"));
                }
                Ok(Self(source))
            }
        }

        impl From<String> for AgreementName {
            #[inline]
            fn from(source: String) -> Self {
                Self(source)
            }
        }

        impl From<&str> for AgreementName {
            #[inline]
            fn from(source: &str) -> Self {
                Self(source.to_owned())
            }
        }

        impl AgreementName {
            #[inline]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        pub type PolicyPrimaryKey = crate::generated::policy::PrimaryKey;
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum KnownKind {
        PrimaryPolicy,
        TermsOfService,
    }

    impl From<PrimaryKey> for KnownKind {
        fn from(primary_key: PrimaryKey) -> Self {
            match primary_key {
                PrimaryKey::PRIMARY_POLICY => Self::PrimaryPolicy,
                PrimaryKey::TERMS_OF_SERVICE => Self::TermsOfService,
                _ => std::unreachable!(),
            }
        }
    }

    impl From<KnownKind> for PrimaryKey {
        fn from(known_kind: KnownKind) -> Self {
            match known_kind {
                KnownKind::PrimaryPolicy => Self::PRIMARY_POLICY,
                KnownKind::TermsOfService => Self::TERMS_OF_SERVICE,
            }
        }
    }

    impl From<Identity> for KnownKind {
        fn from(identity: Identity) -> Self {
            match identity {
                Identity::PRIMARY_POLICY => Self::PrimaryPolicy,
                Identity::TERMS_OF_SERVICE => Self::TermsOfService,
                _ => std::unreachable!(),
            }
        }
    }

    impl From<KnownKind> for Identity {
        fn from(known_kind: KnownKind) -> Self {
            match known_kind {
                KnownKind::PrimaryPolicy => Self::PRIMARY_POLICY,
                KnownKind::TermsOfService => Self::TERMS_OF_SERVICE,
            }
        }
    }

    impl PrimaryKey {
        #[inline]
        pub fn known_kind(self) -> KnownKind {
            KnownKind::from(self)
        }

        #[inline]
        pub fn identity(self) -> Identity {
            Identity::from(self.known_kind())
        }
    }

    impl Identity {
        #[inline]
        pub fn known_kind(self) -> KnownKind {
            KnownKind::from(self)
        }

        #[inline]
        pub fn primary_key(self) -> PrimaryKey {
            PrimaryKey::from(self.known_kind())
        }
    }

    impl PrimaryKey {
        pub const PRIMARY_POLICY: Self =
            unsafe { Self::new_unchecked(0x018d9bd868347d8e9e279e77f5a4fdfb) };
        pub const TERMS_OF_SERVICE: Self =
            unsafe { Self::new_unchecked(0x9aacb6a19f1940d2985401c33876fee5) };
    }

    impl Identity {
        pub const PRIMARY_POLICY: Self =
            unsafe { Self::new_unchecked(0x018d9bd868347bda8b6e2f979a47d57b) };
        pub const TERMS_OF_SERVICE: Self =
            unsafe { Self::new_unchecked(0xa866525497a54a9bab7d1bcd98ad7d59) };
    }

    impl PrimaryKey {
        pub const ALL: [Self; 2] = [
            Self::PRIMARY_POLICY,
            Self::TERMS_OF_SERVICE,
        ];
    }

    impl Identity {
        pub const ALL: [Self; 2] = [
            Self::PRIMARY_POLICY,
            Self::TERMS_OF_SERVICE,
        ];
    }
}

pub mod before_new_password {
    use database_toolkit::Table;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct BeforeNewPassword;

    impl Table for BeforeNewPassword {
        const NAME: &'static str = "before_new_password";
    }

    pub type PrimaryKey = crate::identity::PrimaryKey<BeforeNewPassword>;

    pub type Identity = crate::identity::Identity<BeforeNewPassword>;

    impl identity::Prefix for BeforeNewPassword {
        const PREFIX: &'static str = "before_new_password_";
    }

    pub type Column = database_toolkit::Column<BeforeNewPassword>;

    pub const ALL_COLUMNS: [Column; 6] = [
        columns::BEFORE_NEW_PASSWORD_PK,
        columns::COMPLETED_AT,
        columns::EXPIRED_AT,
        columns::ID,
        columns::PAYLOAD,
        columns::USER_CREDENTIAL_PK,
    ];

    pub mod columns {
        pub const BEFORE_NEW_PASSWORD_PK: super::Column = super::Column::new("before_new_password_pk");
        pub const COMPLETED_AT: super::Column = super::Column::new("completed_at");
        pub const EXPIRED_AT: super::Column = super::Column::new("expired_at");
        pub const ID: super::Column = super::Column::new("id");
        pub const PAYLOAD: super::Column = super::Column::new("payload");
        pub const USER_CREDENTIAL_PK: super::Column = super::Column::new("user_credential_pk");

        pub type BeforeNewPasswordPrimaryKey = crate::generated::before_new_password::PrimaryKey;

        #[derive(Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct BeforeNewPasswordCompletedAt(chrono::DateTime<chrono::Utc>);

        impl From<chrono::DateTime<chrono::Utc>> for BeforeNewPasswordCompletedAt {
            #[inline]
            fn from(source: chrono::DateTime<chrono::Utc>) -> Self {
                Self(source)
            }
        }

        impl From<chrono::NaiveDateTime> for BeforeNewPasswordCompletedAt {
            #[inline]
            fn from(source: chrono::NaiveDateTime) -> Self {
                Self(source.and_utc())
            }
        }

        impl std::ops::Deref for BeforeNewPasswordCompletedAt {
            type Target = chrono::DateTime<chrono::Utc>;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        #[derive(Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct BeforeNewPasswordExpiredAt(chrono::DateTime<chrono::Utc>);

        impl From<chrono::DateTime<chrono::Utc>> for BeforeNewPasswordExpiredAt {
            #[inline]
            fn from(source: chrono::DateTime<chrono::Utc>) -> Self {
                Self(source)
            }
        }

        impl From<chrono::NaiveDateTime> for BeforeNewPasswordExpiredAt {
            #[inline]
            fn from(source: chrono::NaiveDateTime) -> Self {
                Self(source.and_utc())
            }
        }

        impl std::ops::Deref for BeforeNewPasswordExpiredAt {
            type Target = chrono::DateTime<chrono::Utc>;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        pub type BeforeNewPasswordIdentity = super::Identity;

        #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
        pub struct BeforeNewPasswordJson<T>(T);

        pub type UserCredentialPrimaryKey = crate::generated::user_credential::PrimaryKey;
    }
}

pub mod before_signup {
    use database_toolkit::Table;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct BeforeSignup;

    impl Table for BeforeSignup {
        const NAME: &'static str = "before_signup";
    }

    pub type PrimaryKey = crate::identity::PrimaryKey<BeforeSignup>;

    pub type Identity = crate::identity::Identity<BeforeSignup>;

    impl identity::Prefix for BeforeSignup {
        const PREFIX: &'static str = "before_signup_";
    }

    pub type Column = database_toolkit::Column<BeforeSignup>;

    pub const ALL_COLUMNS: [Column; 5] = [
        columns::BEFORE_SIGNUP_PK,
        columns::COMPLETED_AT,
        columns::EXPIRED_AT,
        columns::ID,
        columns::PAYLOAD,
    ];

    pub mod columns {
        pub const BEFORE_SIGNUP_PK: super::Column = super::Column::new("before_signup_pk");
        pub const COMPLETED_AT: super::Column = super::Column::new("completed_at");
        pub const EXPIRED_AT: super::Column = super::Column::new("expired_at");
        pub const ID: super::Column = super::Column::new("id");
        pub const PAYLOAD: super::Column = super::Column::new("payload");

        pub type BeforeSignupPrimaryKey = crate::generated::before_signup::PrimaryKey;

        #[derive(Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct BeforeSignupCompletedAt(chrono::DateTime<chrono::Utc>);

        impl From<chrono::DateTime<chrono::Utc>> for BeforeSignupCompletedAt {
            #[inline]
            fn from(source: chrono::DateTime<chrono::Utc>) -> Self {
                Self(source)
            }
        }

        impl From<chrono::NaiveDateTime> for BeforeSignupCompletedAt {
            #[inline]
            fn from(source: chrono::NaiveDateTime) -> Self {
                Self(source.and_utc())
            }
        }

        impl std::ops::Deref for BeforeSignupCompletedAt {
            type Target = chrono::DateTime<chrono::Utc>;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        #[derive(Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct BeforeSignupExpiredAt(chrono::DateTime<chrono::Utc>);

        impl From<chrono::DateTime<chrono::Utc>> for BeforeSignupExpiredAt {
            #[inline]
            fn from(source: chrono::DateTime<chrono::Utc>) -> Self {
                Self(source)
            }
        }

        impl From<chrono::NaiveDateTime> for BeforeSignupExpiredAt {
            #[inline]
            fn from(source: chrono::NaiveDateTime) -> Self {
                Self(source.and_utc())
            }
        }

        impl std::ops::Deref for BeforeSignupExpiredAt {
            type Target = chrono::DateTime<chrono::Utc>;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        pub type BeforeSignupIdentity = super::Identity;

        #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
        pub struct BeforeSignupJson<T>(T);
    }
}

pub mod credential {
    use database_toolkit::Table;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct Credential;

    impl Table for Credential {
        const NAME: &'static str = "credential";
    }

    pub type PrimaryKey = crate::identity::PrimaryKey<Credential>;

    pub type Identity = crate::identity::Identity<Credential>;

    impl identity::Prefix for Credential {
        const PREFIX: &'static str = "credential_";
    }

    pub type Column = database_toolkit::Column<Credential>;

    pub const ALL_COLUMNS: [Column; 3] = [
        columns::CREDENTIAL_PK,
        columns::ID,
        columns::NAME,
    ];

    pub mod columns {
        pub const CREDENTIAL_PK: super::Column = super::Column::new("credential_pk");
        pub const ID: super::Column = super::Column::new("id");
        pub const NAME: super::Column = super::Column::new("name");

        pub type CredentialPrimaryKey = crate::generated::credential::PrimaryKey;

        pub type CredentialIdentity = super::Identity;

        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
        pub struct CredentialName(String);

        impl std::ops::Deref for CredentialName {
            type Target = str;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl<'de> serde::Deserialize<'de> for CredentialName {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let source = String::deserialize(deserializer)?;
                if source.len() < 1 {
                    return Err(serde::de::Error::custom("too short"));
                }
                if source.len() > 64 {
                    return Err(serde::de::Error::custom("too long"));
                }
                Ok(Self(source))
            }
        }

        impl From<String> for CredentialName {
            #[inline]
            fn from(source: String) -> Self {
                Self(source)
            }
        }

        impl From<&str> for CredentialName {
            #[inline]
            fn from(source: &str) -> Self {
                Self(source.to_owned())
            }
        }

        impl CredentialName {
            #[inline]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum KnownKind {
        Email,
        OnetimePassword,
    }

    impl From<PrimaryKey> for KnownKind {
        fn from(primary_key: PrimaryKey) -> Self {
            match primary_key {
                PrimaryKey::EMAIL => Self::Email,
                PrimaryKey::ONETIME_PASSWORD => Self::OnetimePassword,
                _ => std::unreachable!(),
            }
        }
    }

    impl From<KnownKind> for PrimaryKey {
        fn from(known_kind: KnownKind) -> Self {
            match known_kind {
                KnownKind::Email => Self::EMAIL,
                KnownKind::OnetimePassword => Self::ONETIME_PASSWORD,
            }
        }
    }

    impl From<Identity> for KnownKind {
        fn from(identity: Identity) -> Self {
            match identity {
                Identity::EMAIL => Self::Email,
                Identity::ONETIME_PASSWORD => Self::OnetimePassword,
                _ => std::unreachable!(),
            }
        }
    }

    impl From<KnownKind> for Identity {
        fn from(known_kind: KnownKind) -> Self {
            match known_kind {
                KnownKind::Email => Self::EMAIL,
                KnownKind::OnetimePassword => Self::ONETIME_PASSWORD,
            }
        }
    }

    impl PrimaryKey {
        #[inline]
        pub fn known_kind(self) -> KnownKind {
            KnownKind::from(self)
        }

        #[inline]
        pub fn identity(self) -> Identity {
            Identity::from(self.known_kind())
        }
    }

    impl Identity {
        #[inline]
        pub fn known_kind(self) -> KnownKind {
            KnownKind::from(self)
        }

        #[inline]
        pub fn primary_key(self) -> PrimaryKey {
            PrimaryKey::from(self.known_kind())
        }
    }

    impl PrimaryKey {
        pub const EMAIL: Self =
            unsafe { Self::new_unchecked(0x018d938bdf7e73029d9fee4b47762169) };
        pub const ONETIME_PASSWORD: Self =
            unsafe { Self::new_unchecked(0xd2d8c8f225e14666a9007d241cc22c1b) };
    }

    impl Identity {
        pub const EMAIL: Self =
            unsafe { Self::new_unchecked(0x018d938bdf7e7d27a440bc2ae08ed412) };
        pub const ONETIME_PASSWORD: Self =
            unsafe { Self::new_unchecked(0xa6ae875660b64b418356eb856bb5cca2) };
    }

    impl PrimaryKey {
        pub const ALL: [Self; 2] = [
            Self::EMAIL,
            Self::ONETIME_PASSWORD,
        ];
    }

    impl Identity {
        pub const ALL: [Self; 2] = [
            Self::EMAIL,
            Self::ONETIME_PASSWORD,
        ];
    }
}

pub mod hasher {
    use database_toolkit::Table;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct Hasher;

    impl Table for Hasher {
        const NAME: &'static str = "hasher";
    }

    pub type PrimaryKey = crate::identity::PrimaryKey<Hasher>;

    pub type Column = database_toolkit::Column<Hasher>;

    pub const ALL_COLUMNS: [Column; 2] = [
        columns::HASHER_PK,
        columns::NAME,
    ];

    pub mod columns {
        pub const HASHER_PK: super::Column = super::Column::new("hasher_pk");
        pub const NAME: super::Column = super::Column::new("name");

        pub type HasherPrimaryKey = crate::generated::hasher::PrimaryKey;

        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
        pub struct HasherName(String);

        impl std::ops::Deref for HasherName {
            type Target = str;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl<'de> serde::Deserialize<'de> for HasherName {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let source = String::deserialize(deserializer)?;
                if source.len() < 1 {
                    return Err(serde::de::Error::custom("too short"));
                }
                if source.len() > 64 {
                    return Err(serde::de::Error::custom("too long"));
                }
                Ok(Self(source))
            }
        }

        impl From<String> for HasherName {
            #[inline]
            fn from(source: String) -> Self {
                Self(source)
            }
        }

        impl From<&str> for HasherName {
            #[inline]
            fn from(source: &str) -> Self {
                Self(source.to_owned())
            }
        }

        impl HasherName {
            #[inline]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum KnownKind {
        Argon2,
        Bcrypt,
    }

    impl From<PrimaryKey> for KnownKind {
        fn from(primary_key: PrimaryKey) -> Self {
            match primary_key {
                PrimaryKey::ARGON2 => Self::Argon2,
                PrimaryKey::BCRYPT => Self::Bcrypt,
                _ => std::unreachable!(),
            }
        }
    }

    impl From<KnownKind> for PrimaryKey {
        fn from(known_kind: KnownKind) -> Self {
            match known_kind {
                KnownKind::Argon2 => Self::ARGON2,
                KnownKind::Bcrypt => Self::BCRYPT,
            }
        }
    }

    impl PrimaryKey {
        #[inline]
        pub fn known_kind(self) -> KnownKind {
            KnownKind::from(self)
        }
    }

    impl PrimaryKey {
        pub const ARGON2: Self =
            unsafe { Self::new_unchecked(0x018d938bdf7e761199bcf12458f48155) };
        pub const BCRYPT: Self =
            unsafe { Self::new_unchecked(0x018d938bdf7e761199bcf12458f48156) };
    }

    impl PrimaryKey {
        pub const ALL: [Self; 2] = [
            Self::ARGON2,
            Self::BCRYPT,
        ];
    }
}

pub mod permission {
    use database_toolkit::Table;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct Permission;

    impl Table for Permission {
        const NAME: &'static str = "permission";
    }

    pub type Identity = crate::identity::Identity<Permission>;

    impl identity::Prefix for Permission {
        const PREFIX: &'static str = "permission_";
    }

    pub type PrimaryKey = crate::identity::PrimaryKey<Permission>;

    pub type Column = database_toolkit::Column<Permission>;

    pub const ALL_COLUMNS: [Column; 3] = [
        columns::ID,
        columns::NAME,
        columns::PERMISSION_PK,
    ];

    pub mod columns {
        pub const ID: super::Column = super::Column::new("id");
        pub const NAME: super::Column = super::Column::new("name");
        pub const PERMISSION_PK: super::Column = super::Column::new("permission_pk");

        pub type PermissionIdentity = super::Identity;

        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
        pub struct PermissionName(String);

        impl std::ops::Deref for PermissionName {
            type Target = str;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl<'de> serde::Deserialize<'de> for PermissionName {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let source = String::deserialize(deserializer)?;
                if source.len() < 1 {
                    return Err(serde::de::Error::custom("too short"));
                }
                if source.len() > 64 {
                    return Err(serde::de::Error::custom("too long"));
                }
                Ok(Self(source))
            }
        }

        impl From<String> for PermissionName {
            #[inline]
            fn from(source: String) -> Self {
                Self(source)
            }
        }

        impl From<&str> for PermissionName {
            #[inline]
            fn from(source: &str) -> Self {
                Self(source.to_owned())
            }
        }

        impl PermissionName {
            #[inline]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        pub type PermissionPrimaryKey = crate::generated::permission::PrimaryKey;
    }
}

pub mod policy {
    use database_toolkit::Table;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct Policy;

    impl Table for Policy {
        const NAME: &'static str = "policy";
    }

    pub type PrimaryKey = crate::identity::PrimaryKey<Policy>;

    pub type Column = database_toolkit::Column<Policy>;

    pub const ALL_COLUMNS: [Column; 2] = [
        columns::NAME,
        columns::POLICY_PK,
    ];

    pub mod columns {
        pub const NAME: super::Column = super::Column::new("name");
        pub const POLICY_PK: super::Column = super::Column::new("policy_pk");

        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
        pub struct PolicyName(String);

        impl std::ops::Deref for PolicyName {
            type Target = str;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl<'de> serde::Deserialize<'de> for PolicyName {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let source = String::deserialize(deserializer)?;
                if source.len() < 1 {
                    return Err(serde::de::Error::custom("too short"));
                }
                if source.len() > 64 {
                    return Err(serde::de::Error::custom("too long"));
                }
                Ok(Self(source))
            }
        }

        impl From<String> for PolicyName {
            #[inline]
            fn from(source: String) -> Self {
                Self(source)
            }
        }

        impl From<&str> for PolicyName {
            #[inline]
            fn from(source: &str) -> Self {
                Self(source.to_owned())
            }
        }

        impl PolicyName {
            #[inline]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        pub type PolicyPrimaryKey = crate::generated::policy::PrimaryKey;
    }
}

pub mod role {
    use database_toolkit::Table;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct Role;

    impl Table for Role {
        const NAME: &'static str = "role";
    }

    pub type Identity = crate::identity::Identity<Role>;

    impl identity::Prefix for Role {
        const PREFIX: &'static str = "role_";
    }

    pub type PrimaryKey = crate::identity::PrimaryKey<Role>;

    pub type Column = database_toolkit::Column<Role>;

    pub const ALL_COLUMNS: [Column; 3] = [
        columns::ID,
        columns::NAME,
        columns::ROLE_PK,
    ];

    pub mod columns {
        pub const ID: super::Column = super::Column::new("id");
        pub const NAME: super::Column = super::Column::new("name");
        pub const ROLE_PK: super::Column = super::Column::new("role_pk");

        pub type RoleIdentity = super::Identity;

        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
        pub struct RoleName(String);

        impl std::ops::Deref for RoleName {
            type Target = str;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl<'de> serde::Deserialize<'de> for RoleName {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let source = String::deserialize(deserializer)?;
                if source.len() < 1 {
                    return Err(serde::de::Error::custom("too short"));
                }
                if source.len() > 64 {
                    return Err(serde::de::Error::custom("too long"));
                }
                Ok(Self(source))
            }
        }

        impl From<String> for RoleName {
            #[inline]
            fn from(source: String) -> Self {
                Self(source)
            }
        }

        impl From<&str> for RoleName {
            #[inline]
            fn from(source: &str) -> Self {
                Self(source.to_owned())
            }
        }

        impl RoleName {
            #[inline]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        pub type RolePrimaryKey = crate::generated::role::PrimaryKey;
    }
}

pub mod user {
    use database_toolkit::Table;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct User;

    impl Table for User {
        const NAME: &'static str = "user";
    }

    pub type Identity = crate::identity::Identity<User>;

    impl identity::Prefix for User {
        const PREFIX: &'static str = "user_";
    }

    pub type PrimaryKey = crate::identity::PrimaryKey<User>;

    pub type Column = database_toolkit::Column<User>;

    pub const ALL_COLUMNS: [Column; 3] = [
        columns::DEACTIVATED_AT,
        columns::ID,
        columns::USER_PK,
    ];

    pub mod columns {
        pub const DEACTIVATED_AT: super::Column = super::Column::new("deactivated_at");
        pub const ID: super::Column = super::Column::new("id");
        pub const USER_PK: super::Column = super::Column::new("user_pk");

        #[derive(Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct UserDeactivatedAt(chrono::DateTime<chrono::Utc>);

        impl From<chrono::DateTime<chrono::Utc>> for UserDeactivatedAt {
            #[inline]
            fn from(source: chrono::DateTime<chrono::Utc>) -> Self {
                Self(source)
            }
        }

        impl From<chrono::NaiveDateTime> for UserDeactivatedAt {
            #[inline]
            fn from(source: chrono::NaiveDateTime) -> Self {
                Self(source.and_utc())
            }
        }

        impl std::ops::Deref for UserDeactivatedAt {
            type Target = chrono::DateTime<chrono::Utc>;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        pub type UserIdentity = super::Identity;

        pub type UserPrimaryKey = crate::generated::user::PrimaryKey;
    }
}

pub mod user_activity {
    use database_toolkit::Table;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct UserActivity;

    impl Table for UserActivity {
        const NAME: &'static str = "user_activity";
    }

    pub type PrimaryKey = crate::identity::PrimaryKey<UserActivity>;

    pub type Identity = crate::identity::Identity<UserActivity>;

    impl identity::Prefix for UserActivity {
        const PREFIX: &'static str = "user_activity_";
    }

    pub type Column = database_toolkit::Column<UserActivity>;

    pub const ALL_COLUMNS: [Column; 4] = [
        columns::ACTIVITY_PK,
        columns::ID,
        columns::IP_ADDRESS,
        columns::USER_PK,
    ];

    pub mod columns {
        pub const ACTIVITY_PK: super::Column = super::Column::new("activity_pk");
        pub const ID: super::Column = super::Column::new("id");
        pub const IP_ADDRESS: super::Column = super::Column::new("ip_address");
        pub const USER_PK: super::Column = super::Column::new("user_pk");

        pub type ActivityPrimaryKey = crate::generated::activity::PrimaryKey;

        pub type UserActivityIdentity = super::Identity;

        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
        pub struct UserActivityIpAddr(new_type::IpAddr);

        impl std::ops::Deref for UserActivityIpAddr {
            type Target = new_type::IpAddr;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl std::str::FromStr for UserActivityIpAddr {
            type Err = std::net::AddrParseError;

            #[inline]
            fn from_str(source: &str) -> Result<Self, Self::Err> {
                Ok(Self(source.parse()?))
            }
        }

        impl From<[u8; 4]> for UserActivityIpAddr {
            #[inline]
            fn from(value: [u8; 4]) -> Self {
                Self(value.into())
            }
        }

        impl From<[u8; 16]> for UserActivityIpAddr {
            #[inline]
            fn from(value: [u8; 16]) -> Self {
                Self(value.into())
            }
        }

        impl From<[u16; 8]> for UserActivityIpAddr {
            #[inline]
            fn from(value: [u16; 8]) -> Self {
                Self(value.into())
            }
        }

        impl From<u32> for UserActivityIpAddr {
            #[inline]
            fn from(value: u32) -> Self {
                Self(value.into())
            }
        }

        impl From<u128> for UserActivityIpAddr {
            #[inline]
            fn from(value: u128) -> Self {
                Self(value.into())
            }
        }

        impl From<UserActivityIpAddr> for Vec<u8> {
            #[inline]
            fn from(value: UserActivityIpAddr) -> Self {
                value.0.into()
            }
        }

        impl std::convert::TryFrom<&[u8]> for UserActivityIpAddr {
            type Error = Vec<u8>;

            #[inline]
            fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
                Ok(Self(value.try_into()?))
            }
        }

        pub type UserPrimaryKey = crate::generated::user::PrimaryKey;
    }
}

pub mod user_agreement {
    use database_toolkit::Table;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct UserAgreement;

    impl Table for UserAgreement {
        const NAME: &'static str = "user_agreement";
    }

    pub type PrimaryKey = crate::identity::PrimaryKey<UserAgreement>;

    pub type Column = database_toolkit::Column<UserAgreement>;

    pub const ALL_COLUMNS: [Column; 3] = [
        columns::AGREEMENT_PK,
        columns::EXPIRED_AT,
        columns::USER_PK,
    ];

    pub mod columns {
        pub const AGREEMENT_PK: super::Column = super::Column::new("agreement_pk");
        pub const EXPIRED_AT: super::Column = super::Column::new("expired_at");
        pub const USER_PK: super::Column = super::Column::new("user_pk");

        pub type AgreementPrimaryKey = crate::generated::agreement::PrimaryKey;

        #[derive(Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct UserAgreementExpiredAt(chrono::DateTime<chrono::Utc>);

        impl From<chrono::DateTime<chrono::Utc>> for UserAgreementExpiredAt {
            #[inline]
            fn from(source: chrono::DateTime<chrono::Utc>) -> Self {
                Self(source)
            }
        }

        impl From<chrono::NaiveDateTime> for UserAgreementExpiredAt {
            #[inline]
            fn from(source: chrono::NaiveDateTime) -> Self {
                Self(source.and_utc())
            }
        }

        impl std::ops::Deref for UserAgreementExpiredAt {
            type Target = chrono::DateTime<chrono::Utc>;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        pub type UserPrimaryKey = crate::generated::user::PrimaryKey;
    }
}

pub mod user_credential {
    use database_toolkit::Table;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct UserCredential;

    impl Table for UserCredential {
        const NAME: &'static str = "user_credential";
    }

    pub type PrimaryKey = crate::identity::PrimaryKey<UserCredential>;

    pub type Column = database_toolkit::Column<UserCredential>;

    pub const ALL_COLUMNS: [Column; 4] = [
        columns::CREDENTIAL_PK,
        columns::EXTERNAL_ID,
        columns::USER_CREDENTIAL_PK,
        columns::USER_PK,
    ];

    pub mod columns {
        pub const CREDENTIAL_PK: super::Column = super::Column::new("credential_pk");
        pub const EXTERNAL_ID: super::Column = super::Column::new("external_id");
        pub const USER_CREDENTIAL_PK: super::Column = super::Column::new("user_credential_pk");
        pub const USER_PK: super::Column = super::Column::new("user_pk");

        pub type CredentialPrimaryKey = crate::generated::credential::PrimaryKey;

        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
        pub struct UserCredentialExternalId(String);

        impl std::ops::Deref for UserCredentialExternalId {
            type Target = str;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl<'de> serde::Deserialize<'de> for UserCredentialExternalId {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let source = String::deserialize(deserializer)?;
                if source.len() < 1 {
                    return Err(serde::de::Error::custom("too short"));
                }
                if source.len() > 255 {
                    return Err(serde::de::Error::custom("too long"));
                }
                Ok(Self(source))
            }
        }

        impl From<String> for UserCredentialExternalId {
            #[inline]
            fn from(source: String) -> Self {
                Self(source)
            }
        }

        impl From<&str> for UserCredentialExternalId {
            #[inline]
            fn from(source: &str) -> Self {
                Self(source.to_owned())
            }
        }

        impl UserCredentialExternalId {
            #[inline]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        pub type UserCredentialPrimaryKey = crate::generated::user_credential::PrimaryKey;

        pub type UserPrimaryKey = crate::generated::user::PrimaryKey;
    }
}

pub mod user_credential__has__hasher {
    use database_toolkit::Table;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct UserCredentialHasHasher;

    impl Table for UserCredentialHasHasher {
        const NAME: &'static str = "user_credential__has__hasher";
    }

    pub type PrimaryKey = crate::identity::PrimaryKey<UserCredentialHasHasher>;

    pub type Column = database_toolkit::Column<UserCredentialHasHasher>;

    pub const ALL_COLUMNS: [Column; 4] = [
        columns::EXPIRED_AT,
        columns::HASH,
        columns::HASHER_PK,
        columns::USER_CREDENTIAL_PK,
    ];

    pub mod columns {
        pub const EXPIRED_AT: super::Column = super::Column::new("expired_at");
        pub const HASH: super::Column = super::Column::new("hash");
        pub const HASHER_PK: super::Column = super::Column::new("hasher_pk");
        pub const USER_CREDENTIAL_PK: super::Column = super::Column::new("user_credential_pk");

        #[derive(Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct UserCredentialHasHasherExpiredAt(chrono::DateTime<chrono::Utc>);

        impl From<chrono::DateTime<chrono::Utc>> for UserCredentialHasHasherExpiredAt {
            #[inline]
            fn from(source: chrono::DateTime<chrono::Utc>) -> Self {
                Self(source)
            }
        }

        impl From<chrono::NaiveDateTime> for UserCredentialHasHasherExpiredAt {
            #[inline]
            fn from(source: chrono::NaiveDateTime) -> Self {
                Self(source.and_utc())
            }
        }

        impl std::ops::Deref for UserCredentialHasHasherExpiredAt {
            type Target = chrono::DateTime<chrono::Utc>;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
        pub struct UserCredentialHasHasherHash(new_type::Hash);

        impl From<String> for UserCredentialHasHasherHash {
            #[inline]
            fn from(hash: String) -> Self {
                Self(hash.into())
            }
        }

        impl From<&str> for UserCredentialHasHasherHash {
            #[inline]
            fn from(hash: &str) -> Self {
                Self(hash.into())
            }
        }

        impl From<UserCredentialHasHasherHash> for new_type::Hash {
            #[inline]
            fn from(hash: UserCredentialHasHasherHash) -> Self {
                hash.0
            }
        }

        impl AsRef<new_type::Hash> for UserCredentialHasHasherHash {
            #[inline]
            fn as_ref(&self) -> &new_type::Hash {
                &self.0
            }
        }

        pub type HasherPrimaryKey = crate::generated::hasher::PrimaryKey;

        pub type UserCredentialPrimaryKey = crate::generated::user_credential::PrimaryKey;
    }
}

pub mod user_group {
    use database_toolkit::Table;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct UserGroup;

    impl Table for UserGroup {
        const NAME: &'static str = "user_group";
    }

    pub type Identity = crate::identity::Identity<UserGroup>;

    impl identity::Prefix for UserGroup {
        const PREFIX: &'static str = "user_group_";
    }

    pub type PrimaryKey = crate::identity::PrimaryKey<UserGroup>;

    pub type Column = database_toolkit::Column<UserGroup>;

    pub const ALL_COLUMNS: [Column; 3] = [
        columns::ID,
        columns::NAME,
        columns::USER_GROUP_PK,
    ];

    pub mod columns {
        pub const ID: super::Column = super::Column::new("id");
        pub const NAME: super::Column = super::Column::new("name");
        pub const USER_GROUP_PK: super::Column = super::Column::new("user_group_pk");

        pub type UserGroupIdentity = super::Identity;

        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
        pub struct UserGroupName(String);

        impl std::ops::Deref for UserGroupName {
            type Target = str;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl<'de> serde::Deserialize<'de> for UserGroupName {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let source = String::deserialize(deserializer)?;
                if source.len() < 1 {
                    return Err(serde::de::Error::custom("too short"));
                }
                if source.len() > 64 {
                    return Err(serde::de::Error::custom("too long"));
                }
                Ok(Self(source))
            }
        }

        impl From<String> for UserGroupName {
            #[inline]
            fn from(source: String) -> Self {
                Self(source)
            }
        }

        impl From<&str> for UserGroupName {
            #[inline]
            fn from(source: &str) -> Self {
                Self(source.to_owned())
            }
        }

        impl UserGroupName {
            #[inline]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        pub type UserGroupPrimaryKey = crate::generated::user_group::PrimaryKey;
    }
}

pub mod user_group__has__permission {
    use database_toolkit::Table;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct UserGroupHasPermission;

    impl Table for UserGroupHasPermission {
        const NAME: &'static str = "user_group__has__permission";
    }

    pub type Identity = crate::identity::Identity<UserGroupHasPermission>;

    impl identity::Prefix for UserGroupHasPermission {
        const PREFIX: &'static str = "user_group__has__permission_";
    }

    pub type PrimaryKey = crate::identity::PrimaryKey<UserGroupHasPermission>;

    pub type Column = database_toolkit::Column<UserGroupHasPermission>;

    pub const ALL_COLUMNS: [Column; 4] = [
        columns::CREATED_AT,
        columns::ID,
        columns::PERMISSION_PK,
        columns::USER_GROUP_PK,
    ];

    pub mod columns {
        pub const CREATED_AT: super::Column = super::Column::new("created_at");
        pub const ID: super::Column = super::Column::new("id");
        pub const PERMISSION_PK: super::Column = super::Column::new("permission_pk");
        pub const USER_GROUP_PK: super::Column = super::Column::new("user_group_pk");

        #[derive(Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct UserGroupHasPermissionCreatedAt(chrono::DateTime<chrono::Utc>);

        impl From<chrono::DateTime<chrono::Utc>> for UserGroupHasPermissionCreatedAt {
            #[inline]
            fn from(source: chrono::DateTime<chrono::Utc>) -> Self {
                Self(source)
            }
        }

        impl From<chrono::NaiveDateTime> for UserGroupHasPermissionCreatedAt {
            #[inline]
            fn from(source: chrono::NaiveDateTime) -> Self {
                Self(source.and_utc())
            }
        }

        impl std::ops::Deref for UserGroupHasPermissionCreatedAt {
            type Target = chrono::DateTime<chrono::Utc>;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        pub type UserGroupHasPermissionIdentity = super::Identity;

        pub type PermissionPrimaryKey = crate::generated::permission::PrimaryKey;

        pub type UserGroupPrimaryKey = crate::generated::user_group::PrimaryKey;
    }
}

pub mod user_group__has__role {
    use database_toolkit::Table;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct UserGroupHasRole;

    impl Table for UserGroupHasRole {
        const NAME: &'static str = "user_group__has__role";
    }

    pub type Identity = crate::identity::Identity<UserGroupHasRole>;

    impl identity::Prefix for UserGroupHasRole {
        const PREFIX: &'static str = "user_group__has__role_";
    }

    pub type PrimaryKey = crate::identity::PrimaryKey<UserGroupHasRole>;

    pub type Column = database_toolkit::Column<UserGroupHasRole>;

    pub const ALL_COLUMNS: [Column; 4] = [
        columns::CREATED_AT,
        columns::ID,
        columns::ROLE_PK,
        columns::USER_GROUP_PK,
    ];

    pub mod columns {
        pub const CREATED_AT: super::Column = super::Column::new("created_at");
        pub const ID: super::Column = super::Column::new("id");
        pub const ROLE_PK: super::Column = super::Column::new("role_pk");
        pub const USER_GROUP_PK: super::Column = super::Column::new("user_group_pk");

        #[derive(Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct UserGroupHasRoleCreatedAt(chrono::DateTime<chrono::Utc>);

        impl From<chrono::DateTime<chrono::Utc>> for UserGroupHasRoleCreatedAt {
            #[inline]
            fn from(source: chrono::DateTime<chrono::Utc>) -> Self {
                Self(source)
            }
        }

        impl From<chrono::NaiveDateTime> for UserGroupHasRoleCreatedAt {
            #[inline]
            fn from(source: chrono::NaiveDateTime) -> Self {
                Self(source.and_utc())
            }
        }

        impl std::ops::Deref for UserGroupHasRoleCreatedAt {
            type Target = chrono::DateTime<chrono::Utc>;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        pub type UserGroupHasRoleIdentity = super::Identity;

        pub type RolePrimaryKey = crate::generated::role::PrimaryKey;

        pub type UserGroupPrimaryKey = crate::generated::user_group::PrimaryKey;
    }
}

pub mod user_group__has__user {
    use database_toolkit::Table;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct UserGroupHasUser;

    impl Table for UserGroupHasUser {
        const NAME: &'static str = "user_group__has__user";
    }

    pub type Identity = crate::identity::Identity<UserGroupHasUser>;

    impl identity::Prefix for UserGroupHasUser {
        const PREFIX: &'static str = "user_group__has__user_";
    }

    pub type PrimaryKey = crate::identity::PrimaryKey<UserGroupHasUser>;

    pub type Column = database_toolkit::Column<UserGroupHasUser>;

    pub const ALL_COLUMNS: [Column; 4] = [
        columns::CREATED_AT,
        columns::ID,
        columns::USER_GROUP_PK,
        columns::USER_PK,
    ];

    pub mod columns {
        pub const CREATED_AT: super::Column = super::Column::new("created_at");
        pub const ID: super::Column = super::Column::new("id");
        pub const USER_GROUP_PK: super::Column = super::Column::new("user_group_pk");
        pub const USER_PK: super::Column = super::Column::new("user_pk");

        #[derive(Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct UserGroupHasUserCreatedAt(chrono::DateTime<chrono::Utc>);

        impl From<chrono::DateTime<chrono::Utc>> for UserGroupHasUserCreatedAt {
            #[inline]
            fn from(source: chrono::DateTime<chrono::Utc>) -> Self {
                Self(source)
            }
        }

        impl From<chrono::NaiveDateTime> for UserGroupHasUserCreatedAt {
            #[inline]
            fn from(source: chrono::NaiveDateTime) -> Self {
                Self(source.and_utc())
            }
        }

        impl std::ops::Deref for UserGroupHasUserCreatedAt {
            type Target = chrono::DateTime<chrono::Utc>;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        pub type UserGroupHasUserIdentity = super::Identity;

        pub type UserGroupPrimaryKey = crate::generated::user_group::PrimaryKey;

        pub type UserPrimaryKey = crate::generated::user::PrimaryKey;
    }
}

pub mod user_permission {
    use database_toolkit::Table;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct UserPermission;

    impl Table for UserPermission {
        const NAME: &'static str = "user_permission";
    }

    pub type Identity = crate::identity::Identity<UserPermission>;

    impl identity::Prefix for UserPermission {
        const PREFIX: &'static str = "user_permission_";
    }

    pub type PrimaryKey = crate::identity::PrimaryKey<UserPermission>;

    pub type Column = database_toolkit::Column<UserPermission>;

    pub const ALL_COLUMNS: [Column; 4] = [
        columns::CREATED_AT,
        columns::ID,
        columns::PERMISSION_PK,
        columns::USER_PK,
    ];

    pub mod columns {
        pub const CREATED_AT: super::Column = super::Column::new("created_at");
        pub const ID: super::Column = super::Column::new("id");
        pub const PERMISSION_PK: super::Column = super::Column::new("permission_pk");
        pub const USER_PK: super::Column = super::Column::new("user_pk");

        #[derive(Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct UserPermissionCreatedAt(chrono::DateTime<chrono::Utc>);

        impl From<chrono::DateTime<chrono::Utc>> for UserPermissionCreatedAt {
            #[inline]
            fn from(source: chrono::DateTime<chrono::Utc>) -> Self {
                Self(source)
            }
        }

        impl From<chrono::NaiveDateTime> for UserPermissionCreatedAt {
            #[inline]
            fn from(source: chrono::NaiveDateTime) -> Self {
                Self(source.and_utc())
            }
        }

        impl std::ops::Deref for UserPermissionCreatedAt {
            type Target = chrono::DateTime<chrono::Utc>;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        pub type UserPermissionIdentity = super::Identity;

        pub type PermissionPrimaryKey = crate::generated::permission::PrimaryKey;

        pub type UserPrimaryKey = crate::generated::user::PrimaryKey;
    }
}

pub mod user_profile {
    use database_toolkit::Table;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct UserProfile;

    impl Table for UserProfile {
        const NAME: &'static str = "user_profile";
    }

    pub type PrimaryKey = crate::identity::PrimaryKey<UserProfile>;

    pub type Column = database_toolkit::Column<UserProfile>;

    pub const ALL_COLUMNS: [Column; 5] = [
        columns::BIO,
        columns::HANDLE,
        columns::IMAGE,
        columns::NAME,
        columns::USER_PK,
    ];

    pub mod columns {
        pub const BIO: super::Column = super::Column::new("bio");
        pub const HANDLE: super::Column = super::Column::new("handle");
        pub const IMAGE: super::Column = super::Column::new("image");
        pub const NAME: super::Column = super::Column::new("name");
        pub const USER_PK: super::Column = super::Column::new("user_pk");

        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
        pub struct UserProfileBio(String);

        impl std::ops::Deref for UserProfileBio {
            type Target = str;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl<'de> serde::Deserialize<'de> for UserProfileBio {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let source = String::deserialize(deserializer)?;
                if source.len() > 255 {
                    return Err(serde::de::Error::custom("too long"));
                }
                Ok(Self(source))
            }
        }

        impl From<String> for UserProfileBio {
            #[inline]
            fn from(source: String) -> Self {
                Self(source)
            }
        }

        impl From<&str> for UserProfileBio {
            #[inline]
            fn from(source: &str) -> Self {
                Self(source.to_owned())
            }
        }

        impl UserProfileBio {
            #[inline]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
        pub struct UserProfileHandle(new_type::Handle);

        impl std::str::FromStr for UserProfileHandle {
            type Err = new_type::HandleError;

            #[inline]
            fn from_str(source: &str) -> Result<Self, Self::Err> {
                Ok(Self(source.parse()?))
            }
        }

        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
        pub struct UserProfileImage(String);

        impl std::ops::Deref for UserProfileImage {
            type Target = str;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl<'de> serde::Deserialize<'de> for UserProfileImage {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let source = String::deserialize(deserializer)?;
                if source.len() > 255 {
                    return Err(serde::de::Error::custom("too long"));
                }
                Ok(Self(source))
            }
        }

        impl From<String> for UserProfileImage {
            #[inline]
            fn from(source: String) -> Self {
                Self(source)
            }
        }

        impl From<&str> for UserProfileImage {
            #[inline]
            fn from(source: &str) -> Self {
                Self(source.to_owned())
            }
        }

        impl UserProfileImage {
            #[inline]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
        pub struct UserProfileName(String);

        impl std::ops::Deref for UserProfileName {
            type Target = str;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl<'de> serde::Deserialize<'de> for UserProfileName {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let source = String::deserialize(deserializer)?;
                if source.len() < 1 {
                    return Err(serde::de::Error::custom("too short"));
                }
                if source.len() > 64 {
                    return Err(serde::de::Error::custom("too long"));
                }
                Ok(Self(source))
            }
        }

        impl From<String> for UserProfileName {
            #[inline]
            fn from(source: String) -> Self {
                Self(source)
            }
        }

        impl From<&str> for UserProfileName {
            #[inline]
            fn from(source: &str) -> Self {
                Self(source.to_owned())
            }
        }

        impl UserProfileName {
            #[inline]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        pub type UserPrimaryKey = crate::generated::user::PrimaryKey;
    }
}

pub mod user_role {
    use database_toolkit::Table;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct UserRole;

    impl Table for UserRole {
        const NAME: &'static str = "user_role";
    }

    pub type Identity = crate::identity::Identity<UserRole>;

    impl identity::Prefix for UserRole {
        const PREFIX: &'static str = "user_role_";
    }

    pub type PrimaryKey = crate::identity::PrimaryKey<UserRole>;

    pub type Column = database_toolkit::Column<UserRole>;

    pub const ALL_COLUMNS: [Column; 4] = [
        columns::CREATED_AT,
        columns::ID,
        columns::ROLE_PK,
        columns::USER_PK,
    ];

    pub mod columns {
        pub const CREATED_AT: super::Column = super::Column::new("created_at");
        pub const ID: super::Column = super::Column::new("id");
        pub const ROLE_PK: super::Column = super::Column::new("role_pk");
        pub const USER_PK: super::Column = super::Column::new("user_pk");

        #[derive(Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct UserRoleCreatedAt(chrono::DateTime<chrono::Utc>);

        impl From<chrono::DateTime<chrono::Utc>> for UserRoleCreatedAt {
            #[inline]
            fn from(source: chrono::DateTime<chrono::Utc>) -> Self {
                Self(source)
            }
        }

        impl From<chrono::NaiveDateTime> for UserRoleCreatedAt {
            #[inline]
            fn from(source: chrono::NaiveDateTime) -> Self {
                Self(source.and_utc())
            }
        }

        impl std::ops::Deref for UserRoleCreatedAt {
            type Target = chrono::DateTime<chrono::Utc>;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        pub type UserRoleIdentity = super::Identity;

        pub type RolePrimaryKey = crate::generated::role::PrimaryKey;

        pub type UserPrimaryKey = crate::generated::user::PrimaryKey;
    }
}

pub mod user_session {
    use database_toolkit::Table;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct UserSession;

    impl Table for UserSession {
        const NAME: &'static str = "user_session";
    }

    pub type Identity = crate::identity::Identity<UserSession>;

    impl identity::Prefix for UserSession {
        const PREFIX: &'static str = "user_session_";
    }

    pub type PrimaryKey = crate::identity::PrimaryKey<UserSession>;

    pub type Column = database_toolkit::Column<UserSession>;

    pub const ALL_COLUMNS: [Column; 5] = [
        columns::EXPIRED_AT,
        columns::ID,
        columns::IP_ADDRESS,
        columns::USER_PK,
        columns::USER_SESSION_PK,
    ];

    pub mod columns {
        pub const EXPIRED_AT: super::Column = super::Column::new("expired_at");
        pub const ID: super::Column = super::Column::new("id");
        pub const IP_ADDRESS: super::Column = super::Column::new("ip_address");
        pub const USER_PK: super::Column = super::Column::new("user_pk");
        pub const USER_SESSION_PK: super::Column = super::Column::new("user_session_pk");

        #[derive(Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct UserSessionExpiredAt(chrono::DateTime<chrono::Utc>);

        impl From<chrono::DateTime<chrono::Utc>> for UserSessionExpiredAt {
            #[inline]
            fn from(source: chrono::DateTime<chrono::Utc>) -> Self {
                Self(source)
            }
        }

        impl From<chrono::NaiveDateTime> for UserSessionExpiredAt {
            #[inline]
            fn from(source: chrono::NaiveDateTime) -> Self {
                Self(source.and_utc())
            }
        }

        impl std::ops::Deref for UserSessionExpiredAt {
            type Target = chrono::DateTime<chrono::Utc>;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        pub type UserSessionIdentity = super::Identity;

        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
        pub struct UserSessionIpAddr(new_type::IpAddr);

        impl std::ops::Deref for UserSessionIpAddr {
            type Target = new_type::IpAddr;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl std::str::FromStr for UserSessionIpAddr {
            type Err = std::net::AddrParseError;

            #[inline]
            fn from_str(source: &str) -> Result<Self, Self::Err> {
                Ok(Self(source.parse()?))
            }
        }

        impl From<[u8; 4]> for UserSessionIpAddr {
            #[inline]
            fn from(value: [u8; 4]) -> Self {
                Self(value.into())
            }
        }

        impl From<[u8; 16]> for UserSessionIpAddr {
            #[inline]
            fn from(value: [u8; 16]) -> Self {
                Self(value.into())
            }
        }

        impl From<[u16; 8]> for UserSessionIpAddr {
            #[inline]
            fn from(value: [u16; 8]) -> Self {
                Self(value.into())
            }
        }

        impl From<u32> for UserSessionIpAddr {
            #[inline]
            fn from(value: u32) -> Self {
                Self(value.into())
            }
        }

        impl From<u128> for UserSessionIpAddr {
            #[inline]
            fn from(value: u128) -> Self {
                Self(value.into())
            }
        }

        impl From<UserSessionIpAddr> for Vec<u8> {
            #[inline]
            fn from(value: UserSessionIpAddr) -> Self {
                value.0.into()
            }
        }

        impl std::convert::TryFrom<&[u8]> for UserSessionIpAddr {
            type Error = Vec<u8>;

            #[inline]
            fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
                Ok(Self(value.try_into()?))
            }
        }

        pub type UserPrimaryKey = crate::generated::user::PrimaryKey;

        pub type UserSessionPrimaryKey = crate::generated::user_session::PrimaryKey;
    }
}
