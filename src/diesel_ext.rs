pub(crate) mod dsl {
    use diesel::expression::{AsExpression, Expression};
    use diesel::sql_types::Text;

    mod predicates {
        use diesel::sqlite::Sqlite;
        diesel::infix_operator!(Matches, " match ", backend: Sqlite);
    }

    use self::predicates::*;

    pub(crate) trait MatchExpressionMethods: Expression<SqlType = Text> + Sized {
        fn matches<T: AsExpression<Text>>(self, other: T) -> Matches<Self, T::Expression> {
            Matches::new(self, other.as_expression())
        }
    }

    impl<T: Expression<SqlType = Text>> MatchExpressionMethods for T {}
}
