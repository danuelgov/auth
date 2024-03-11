use auth_database::{
    agreement::columns::AgreementPrimaryKey,
    user::columns::UserPrimaryKey,
    user_agreement::{self, UserAgreement},
};
use database_toolkit::{QueryBuilder, Transaction};

pub trait CreateUserAgreementContract {
    async fn create_user_agreement(
        &self,
        transaction: &mut Transaction,
        user_pk: UserPrimaryKey,
        agreements: Vec<AgreementPrimaryKey>,
    ) -> Result<(), CreateUserAgreementError>;
}

#[derive(Debug)]
pub enum CreateUserAgreementError {
    Database(sqlx::Error),
}

impl CreateUserAgreementContract for super::Repository {
    async fn create_user_agreement(
        &self,
        transaction: &mut Transaction,
        user_pk: UserPrimaryKey,
        agreements: Vec<AgreementPrimaryKey>,
    ) -> Result<(), CreateUserAgreementError> {
        if let Some(mut builder) = query(user_pk, agreements) {
            builder
                .build()
                .execute(&mut **transaction)
                .await
                .map_err(CreateUserAgreementError::Database)?;
        }

        Ok(())
    }
}

fn query<'args>(
    user_pk: UserPrimaryKey,
    agreements: Vec<AgreementPrimaryKey>,
) -> Option<QueryBuilder<'args>> {
    let mut agreements = agreements.into_iter();
    let Some(agreement) = agreements.next() else {
        return None;
    };
    let builder = QueryBuilder::new()
        .insert_into(
            UserAgreement,
            &[
                user_agreement::columns::USER_PK,
                user_agreement::columns::AGREEMENT_PK,
            ],
        )
        .values(|builder| {
            let user_pk: Vec<u8> = user_pk.into();
            let builder = builder.nested(|builder| {
                let user_pk = user_pk.to_owned();
                let agreement: Vec<u8> = agreement.into();

                builder.value(user_pk).comma().value(agreement)
            });
            agreements.fold(builder, |builder, agreement| {
                let user_pk = user_pk.to_owned();
                let agreement: Vec<u8> = agreement.into();

                builder
                    .comma()
                    .nested(|builder| builder.value(user_pk).comma().value(agreement))
            })
        });

    Some(builder)
}
