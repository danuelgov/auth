pub const AUTH_TOPIC: Topic = Topic::new("arn:aws:sns:ap-northeast-2:467626204726:DanuelGov-Auth");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Topic {
    arn: &'static str,
}

impl Topic {
    #[inline]
    pub const fn new(arn: &'static str) -> Self {
        Self { arn }
    }

    #[inline]
    pub const fn arn(&self) -> &str {
        self.arn
    }
}
