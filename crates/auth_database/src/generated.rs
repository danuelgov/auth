// !
// ! This file is generated.
// ! Do not modify it manually.
// ! Regenerate it by running `cargo build`.
// !

pub mod activity {
    use database_toolkit::Table;

    pub const TABLE_NAME: Table = Table("activity");

    pub mod columns {
        use database_toolkit::Column;

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

        impl ActivityName {
            #[inline]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        pub const ALL: [Column; 3] = [
            ACTIVITY_PK,
            ID,
            NAME,
        ];

        pub const ACTIVITY_PK: Column = Column("activity_pk");
        pub const ID: Column = Column("id");
        pub const NAME: Column = Column("name");
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct Activity;

    pub type PrimaryKey = crate::identity::PrimaryKey<Activity>;

    pub type Identity = crate::identity::Identity<Activity>;

    impl identity::Prefix for Activity {
        const PREFIX: &'static str = "activity_";
    }
}

pub mod agreement {
    use database_toolkit::Table;

    pub const TABLE_NAME: Table = Table("agreement");

    pub mod columns {
        use database_toolkit::Column;

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

        impl AgreementName {
            #[inline]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        pub type PolicyPrimaryKey = crate::generated::policy::PrimaryKey;

        pub const ALL: [Column; 4] = [
            AGREEMENT_PK,
            ID,
            NAME,
            POLICY_PK,
        ];

        pub const AGREEMENT_PK: Column = Column("agreement_pk");
        pub const ID: Column = Column("id");
        pub const NAME: Column = Column("name");
        pub const POLICY_PK: Column = Column("policy_pk");
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct Agreement;

    pub type PrimaryKey = crate::identity::PrimaryKey<Agreement>;

    pub type Identity = crate::identity::Identity<Agreement>;

    impl identity::Prefix for Agreement {
        const PREFIX: &'static str = "agreement_";
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

    pub const TABLE_NAME: Table = Table("before_new_password");

    pub mod columns {
        use database_toolkit::Column;

        pub type BeforeNewPasswordPrimaryKey = crate::generated::before_new_password::PrimaryKey;

        #[derive(Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct BeforeNewPasswordCompletedAt(chrono::DateTime<chrono::Utc>);

        #[derive(Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct BeforeNewPasswordExpiredAt(chrono::DateTime<chrono::Utc>);

        pub type BeforeNewPasswordIdentity = super::Identity;

        #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
        pub struct BeforeNewPasswordJson<T>(T);

        pub const ALL: [Column; 5] = [
            BEFORE_NEW_PASSWORD_PK,
            COMPLETED_AT,
            EXPIRED_AT,
            ID,
            PAYLOAD,
        ];

        pub const BEFORE_NEW_PASSWORD_PK: Column = Column("before_new_password_pk");
        pub const COMPLETED_AT: Column = Column("completed_at");
        pub const EXPIRED_AT: Column = Column("expired_at");
        pub const ID: Column = Column("id");
        pub const PAYLOAD: Column = Column("payload");
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct BeforeNewPassword;

    pub type PrimaryKey = crate::identity::PrimaryKey<BeforeNewPassword>;

    pub type Identity = crate::identity::Identity<BeforeNewPassword>;

    impl identity::Prefix for BeforeNewPassword {
        const PREFIX: &'static str = "before_new_password_";
    }
}

pub mod before_signup {
    use database_toolkit::Table;

    pub const TABLE_NAME: Table = Table("before_signup");

    pub mod columns {
        use database_toolkit::Column;

        pub type BeforeSignupPrimaryKey = crate::generated::before_signup::PrimaryKey;

        #[derive(Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct BeforeSignupCompletedAt(chrono::DateTime<chrono::Utc>);

        #[derive(Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct BeforeSignupExpiredAt(chrono::DateTime<chrono::Utc>);

        pub type BeforeSignupIdentity = super::Identity;

        #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
        pub struct BeforeSignupJson<T>(T);

        pub const ALL: [Column; 5] = [
            BEFORE_SIGNUP_PK,
            COMPLETED_AT,
            EXPIRED_AT,
            ID,
            PAYLOAD,
        ];

        pub const BEFORE_SIGNUP_PK: Column = Column("before_signup_pk");
        pub const COMPLETED_AT: Column = Column("completed_at");
        pub const EXPIRED_AT: Column = Column("expired_at");
        pub const ID: Column = Column("id");
        pub const PAYLOAD: Column = Column("payload");
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct BeforeSignup;

    pub type PrimaryKey = crate::identity::PrimaryKey<BeforeSignup>;

    pub type Identity = crate::identity::Identity<BeforeSignup>;

    impl identity::Prefix for BeforeSignup {
        const PREFIX: &'static str = "before_signup_";
    }
}

pub mod credential {
    use database_toolkit::Table;

    pub const TABLE_NAME: Table = Table("credential");

    pub mod columns {
        use database_toolkit::Column;

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

        impl CredentialName {
            #[inline]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        pub const ALL: [Column; 3] = [
            CREDENTIAL_PK,
            ID,
            NAME,
        ];

        pub const CREDENTIAL_PK: Column = Column("credential_pk");
        pub const ID: Column = Column("id");
        pub const NAME: Column = Column("name");
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct Credential;

    pub type PrimaryKey = crate::identity::PrimaryKey<Credential>;

    pub type Identity = crate::identity::Identity<Credential>;

    impl identity::Prefix for Credential {
        const PREFIX: &'static str = "credential_";
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

    pub const TABLE_NAME: Table = Table("hasher");

    pub mod columns {
        use database_toolkit::Column;

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

        impl HasherName {
            #[inline]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        pub const ALL: [Column; 2] = [
            HASHER_PK,
            NAME,
        ];

        pub const HASHER_PK: Column = Column("hasher_pk");
        pub const NAME: Column = Column("name");
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct Hasher;

    pub type PrimaryKey = crate::identity::PrimaryKey<Hasher>;

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

    pub const TABLE_NAME: Table = Table("permission");

    pub mod columns {
        use database_toolkit::Column;

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

        impl PermissionName {
            #[inline]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        pub type PermissionPrimaryKey = crate::generated::permission::PrimaryKey;

        pub const ALL: [Column; 3] = [
            ID,
            NAME,
            PERMISSION_PK,
        ];

        pub const ID: Column = Column("id");
        pub const NAME: Column = Column("name");
        pub const PERMISSION_PK: Column = Column("permission_pk");
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct Permission;

    pub type Identity = crate::identity::Identity<Permission>;

    impl identity::Prefix for Permission {
        const PREFIX: &'static str = "permission_";
    }

    pub type PrimaryKey = crate::identity::PrimaryKey<Permission>;
}

pub mod policy {
    use database_toolkit::Table;

    pub const TABLE_NAME: Table = Table("policy");

    pub mod columns {
        use database_toolkit::Column;

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

        impl PolicyName {
            #[inline]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        pub type PolicyPrimaryKey = crate::generated::policy::PrimaryKey;

        pub const ALL: [Column; 2] = [
            NAME,
            POLICY_PK,
        ];

        pub const NAME: Column = Column("name");
        pub const POLICY_PK: Column = Column("policy_pk");
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct Policy;

    pub type PrimaryKey = crate::identity::PrimaryKey<Policy>;
}

pub mod role {
    use database_toolkit::Table;

    pub const TABLE_NAME: Table = Table("role");

    pub mod columns {
        use database_toolkit::Column;

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

        impl RoleName {
            #[inline]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        pub type RolePrimaryKey = crate::generated::role::PrimaryKey;

        pub const ALL: [Column; 3] = [
            ID,
            NAME,
            ROLE_PK,
        ];

        pub const ID: Column = Column("id");
        pub const NAME: Column = Column("name");
        pub const ROLE_PK: Column = Column("role_pk");
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct Role;

    pub type Identity = crate::identity::Identity<Role>;

    impl identity::Prefix for Role {
        const PREFIX: &'static str = "role_";
    }

    pub type PrimaryKey = crate::identity::PrimaryKey<Role>;
}

pub mod user {
    use database_toolkit::Table;

    pub const TABLE_NAME: Table = Table("user");

    pub mod columns {
        use database_toolkit::Column;

        #[derive(Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct UserDeactivatedAt(chrono::DateTime<chrono::Utc>);

        pub type UserIdentity = super::Identity;

        pub type UserPrimaryKey = crate::generated::user::PrimaryKey;

        pub const ALL: [Column; 3] = [
            DEACTIVATED_AT,
            ID,
            USER_PK,
        ];

        pub const DEACTIVATED_AT: Column = Column("deactivated_at");
        pub const ID: Column = Column("id");
        pub const USER_PK: Column = Column("user_pk");
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct User;

    pub type Identity = crate::identity::Identity<User>;

    impl identity::Prefix for User {
        const PREFIX: &'static str = "user_";
    }

    pub type PrimaryKey = crate::identity::PrimaryKey<User>;
}

pub mod user_activity {
    use database_toolkit::Table;

    pub const TABLE_NAME: Table = Table("user_activity");

    pub mod columns {
        use database_toolkit::Column;

        pub type ActivityPrimaryKey = crate::generated::activity::PrimaryKey;

        pub type UserActivityIdentity = super::Identity;

        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
        pub struct UserActivityIpAddr(new_type::IpAddr);

        pub type UserPrimaryKey = crate::generated::user::PrimaryKey;

        pub const ALL: [Column; 4] = [
            ACTIVITY_PK,
            ID,
            IP_ADDRESS,
            USER_PK,
        ];

        pub const ACTIVITY_PK: Column = Column("activity_pk");
        pub const ID: Column = Column("id");
        pub const IP_ADDRESS: Column = Column("ip_address");
        pub const USER_PK: Column = Column("user_pk");
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct UserActivity;

    pub type PrimaryKey = crate::identity::PrimaryKey<UserActivity>;

    pub type Identity = crate::identity::Identity<UserActivity>;

    impl identity::Prefix for UserActivity {
        const PREFIX: &'static str = "user_activity_";
    }
}

pub mod user_agreement {
    use database_toolkit::Table;

    pub const TABLE_NAME: Table = Table("user_agreement");

    pub mod columns {
        use database_toolkit::Column;

        pub type AgreementPrimaryKey = crate::generated::agreement::PrimaryKey;

        #[derive(Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct UserAgreementExpiredAt(chrono::DateTime<chrono::Utc>);

        pub type UserPrimaryKey = crate::generated::user::PrimaryKey;

        pub const ALL: [Column; 3] = [
            AGREEMENT_PK,
            EXPIRED_AT,
            USER_PK,
        ];

        pub const AGREEMENT_PK: Column = Column("agreement_pk");
        pub const EXPIRED_AT: Column = Column("expired_at");
        pub const USER_PK: Column = Column("user_pk");
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct UserAgreement;

    pub type PrimaryKey = crate::identity::PrimaryKey<UserAgreement>;
}

pub mod user_credential {
    use database_toolkit::Table;

    pub const TABLE_NAME: Table = Table("user_credential");

    pub mod columns {
        use database_toolkit::Column;

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

        impl UserCredentialExternalId {
            #[inline]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        pub type UserCredentialPrimaryKey = crate::generated::user_credential::PrimaryKey;

        pub type UserPrimaryKey = crate::generated::user::PrimaryKey;

        pub const ALL: [Column; 4] = [
            CREDENTIAL_PK,
            EXTERNAL_ID,
            USER_CREDENTIAL_PK,
            USER_PK,
        ];

        pub const CREDENTIAL_PK: Column = Column("credential_pk");
        pub const EXTERNAL_ID: Column = Column("external_id");
        pub const USER_CREDENTIAL_PK: Column = Column("user_credential_pk");
        pub const USER_PK: Column = Column("user_pk");
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct UserCredential;

    pub type PrimaryKey = crate::identity::PrimaryKey<UserCredential>;
}

pub mod user_credential__has__hasher {
    use database_toolkit::Table;

    pub const TABLE_NAME: Table = Table("user_credential__has__hasher");

    pub mod columns {
        use database_toolkit::Column;

        #[derive(Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct UserCredentialHasHasherExpiredAt(chrono::DateTime<chrono::Utc>);

        #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
        pub struct UserCredentialHasHasherHash(new_type::Hash);

        pub type HasherPrimaryKey = crate::generated::hasher::PrimaryKey;

        #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
        pub struct UserCredentialHasHasherSalt(new_type::Salt);

        pub type UserCredentialPrimaryKey = crate::generated::user_credential::PrimaryKey;

        pub const ALL: [Column; 5] = [
            EXPIRED_AT,
            HASH,
            HASHER_PK,
            SALT,
            USER_CREDENTIAL_PK,
        ];

        pub const EXPIRED_AT: Column = Column("expired_at");
        pub const HASH: Column = Column("hash");
        pub const HASHER_PK: Column = Column("hasher_pk");
        pub const SALT: Column = Column("salt");
        pub const USER_CREDENTIAL_PK: Column = Column("user_credential_pk");
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct UserCredentialHasHasher;

    pub type PrimaryKey = crate::identity::PrimaryKey<UserCredentialHasHasher>;
}

pub mod user_group {
    use database_toolkit::Table;

    pub const TABLE_NAME: Table = Table("user_group");

    pub mod columns {
        use database_toolkit::Column;

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

        impl UserGroupName {
            #[inline]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        pub type UserGroupPrimaryKey = crate::generated::user_group::PrimaryKey;

        pub const ALL: [Column; 3] = [
            ID,
            NAME,
            USER_GROUP_PK,
        ];

        pub const ID: Column = Column("id");
        pub const NAME: Column = Column("name");
        pub const USER_GROUP_PK: Column = Column("user_group_pk");
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct UserGroup;

    pub type Identity = crate::identity::Identity<UserGroup>;

    impl identity::Prefix for UserGroup {
        const PREFIX: &'static str = "user_group_";
    }

    pub type PrimaryKey = crate::identity::PrimaryKey<UserGroup>;
}

pub mod user_group__has__permission {
    use database_toolkit::Table;

    pub const TABLE_NAME: Table = Table("user_group__has__permission");

    pub mod columns {
        use database_toolkit::Column;

        #[derive(Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct UserGroupHasPermissionCreatedAt(chrono::DateTime<chrono::Utc>);

        pub type UserGroupHasPermissionIdentity = super::Identity;

        pub type PermissionPrimaryKey = crate::generated::permission::PrimaryKey;

        pub type UserGroupPrimaryKey = crate::generated::user_group::PrimaryKey;

        pub const ALL: [Column; 4] = [
            CREATED_AT,
            ID,
            PERMISSION_PK,
            USER_GROUP_PK,
        ];

        pub const CREATED_AT: Column = Column("created_at");
        pub const ID: Column = Column("id");
        pub const PERMISSION_PK: Column = Column("permission_pk");
        pub const USER_GROUP_PK: Column = Column("user_group_pk");
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct UserGroupHasPermission;

    pub type Identity = crate::identity::Identity<UserGroupHasPermission>;

    impl identity::Prefix for UserGroupHasPermission {
        const PREFIX: &'static str = "user_group__has__permission_";
    }

    pub type PrimaryKey = crate::identity::PrimaryKey<UserGroupHasPermission>;
}

pub mod user_group__has__role {
    use database_toolkit::Table;

    pub const TABLE_NAME: Table = Table("user_group__has__role");

    pub mod columns {
        use database_toolkit::Column;

        #[derive(Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct UserGroupHasRoleCreatedAt(chrono::DateTime<chrono::Utc>);

        pub type UserGroupHasRoleIdentity = super::Identity;

        pub type RolePrimaryKey = crate::generated::role::PrimaryKey;

        pub type UserGroupPrimaryKey = crate::generated::user_group::PrimaryKey;

        pub const ALL: [Column; 4] = [
            CREATED_AT,
            ID,
            ROLE_PK,
            USER_GROUP_PK,
        ];

        pub const CREATED_AT: Column = Column("created_at");
        pub const ID: Column = Column("id");
        pub const ROLE_PK: Column = Column("role_pk");
        pub const USER_GROUP_PK: Column = Column("user_group_pk");
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct UserGroupHasRole;

    pub type Identity = crate::identity::Identity<UserGroupHasRole>;

    impl identity::Prefix for UserGroupHasRole {
        const PREFIX: &'static str = "user_group__has__role_";
    }

    pub type PrimaryKey = crate::identity::PrimaryKey<UserGroupHasRole>;
}

pub mod user_group__has__user {
    use database_toolkit::Table;

    pub const TABLE_NAME: Table = Table("user_group__has__user");

    pub mod columns {
        use database_toolkit::Column;

        #[derive(Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct UserGroupHasUserCreatedAt(chrono::DateTime<chrono::Utc>);

        pub type UserGroupHasUserIdentity = super::Identity;

        pub type UserGroupPrimaryKey = crate::generated::user_group::PrimaryKey;

        pub type UserPrimaryKey = crate::generated::user::PrimaryKey;

        pub const ALL: [Column; 4] = [
            CREATED_AT,
            ID,
            USER_GROUP_PK,
            USER_PK,
        ];

        pub const CREATED_AT: Column = Column("created_at");
        pub const ID: Column = Column("id");
        pub const USER_GROUP_PK: Column = Column("user_group_pk");
        pub const USER_PK: Column = Column("user_pk");
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct UserGroupHasUser;

    pub type Identity = crate::identity::Identity<UserGroupHasUser>;

    impl identity::Prefix for UserGroupHasUser {
        const PREFIX: &'static str = "user_group__has__user_";
    }

    pub type PrimaryKey = crate::identity::PrimaryKey<UserGroupHasUser>;
}

pub mod user_permission {
    use database_toolkit::Table;

    pub const TABLE_NAME: Table = Table("user_permission");

    pub mod columns {
        use database_toolkit::Column;

        #[derive(Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct UserPermissionCreatedAt(chrono::DateTime<chrono::Utc>);

        pub type UserPermissionIdentity = super::Identity;

        pub type PermissionPrimaryKey = crate::generated::permission::PrimaryKey;

        pub type UserPrimaryKey = crate::generated::user::PrimaryKey;

        pub const ALL: [Column; 4] = [
            CREATED_AT,
            ID,
            PERMISSION_PK,
            USER_PK,
        ];

        pub const CREATED_AT: Column = Column("created_at");
        pub const ID: Column = Column("id");
        pub const PERMISSION_PK: Column = Column("permission_pk");
        pub const USER_PK: Column = Column("user_pk");
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct UserPermission;

    pub type Identity = crate::identity::Identity<UserPermission>;

    impl identity::Prefix for UserPermission {
        const PREFIX: &'static str = "user_permission_";
    }

    pub type PrimaryKey = crate::identity::PrimaryKey<UserPermission>;
}

pub mod user_profile {
    use database_toolkit::Table;

    pub const TABLE_NAME: Table = Table("user_profile");

    pub mod columns {
        use database_toolkit::Column;

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

        impl UserProfileBio {
            #[inline]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
        pub struct UserProfileHandle(new_type::Handle);

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

        impl UserProfileName {
            #[inline]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        pub type UserPrimaryKey = crate::generated::user::PrimaryKey;

        pub const ALL: [Column; 5] = [
            BIO,
            HANDLE,
            IMAGE,
            NAME,
            USER_PK,
        ];

        pub const BIO: Column = Column("bio");
        pub const HANDLE: Column = Column("handle");
        pub const IMAGE: Column = Column("image");
        pub const NAME: Column = Column("name");
        pub const USER_PK: Column = Column("user_pk");
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct UserProfile;

    pub type PrimaryKey = crate::identity::PrimaryKey<UserProfile>;
}

pub mod user_role {
    use database_toolkit::Table;

    pub const TABLE_NAME: Table = Table("user_role");

    pub mod columns {
        use database_toolkit::Column;

        #[derive(Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct UserRoleCreatedAt(chrono::DateTime<chrono::Utc>);

        pub type UserRoleIdentity = super::Identity;

        pub type RolePrimaryKey = crate::generated::role::PrimaryKey;

        pub type UserPrimaryKey = crate::generated::user::PrimaryKey;

        pub const ALL: [Column; 4] = [
            CREATED_AT,
            ID,
            ROLE_PK,
            USER_PK,
        ];

        pub const CREATED_AT: Column = Column("created_at");
        pub const ID: Column = Column("id");
        pub const ROLE_PK: Column = Column("role_pk");
        pub const USER_PK: Column = Column("user_pk");
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct UserRole;

    pub type Identity = crate::identity::Identity<UserRole>;

    impl identity::Prefix for UserRole {
        const PREFIX: &'static str = "user_role_";
    }

    pub type PrimaryKey = crate::identity::PrimaryKey<UserRole>;
}

pub mod user_session {
    use database_toolkit::Table;

    pub const TABLE_NAME: Table = Table("user_session");

    pub mod columns {
        use database_toolkit::Column;

        #[derive(Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct UserSessionExpiredAt(chrono::DateTime<chrono::Utc>);

        pub type UserSessionIdentity = super::Identity;

        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
        pub struct UserSessionIpAddr(new_type::IpAddr);

        pub type UserPrimaryKey = crate::generated::user::PrimaryKey;

        pub type UserSessionPrimaryKey = crate::generated::user_session::PrimaryKey;

        pub const ALL: [Column; 5] = [
            EXPIRED_AT,
            ID,
            IP_ADDRESS,
            USER_PK,
            USER_SESSION_PK,
        ];

        pub const EXPIRED_AT: Column = Column("expired_at");
        pub const ID: Column = Column("id");
        pub const IP_ADDRESS: Column = Column("ip_address");
        pub const USER_PK: Column = Column("user_pk");
        pub const USER_SESSION_PK: Column = Column("user_session_pk");
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct UserSession;

    pub type Identity = crate::identity::Identity<UserSession>;

    impl identity::Prefix for UserSession {
        const PREFIX: &'static str = "user_session_";
    }

    pub type PrimaryKey = crate::identity::PrimaryKey<UserSession>;
}
