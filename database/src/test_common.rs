use test_context::{AsyncTestContext, TestContext};

pub(crate) struct Context {}

impl TestContext for Context {
    fn setup() -> Self {
        Self {}
    }

    fn teardown(self) {}
}

pub(crate) struct ContextAsync {}

impl AsyncTestContext for ContextAsync {
    async fn setup() -> Self {
        Self {}
    }

    async fn teardown(self) {}
}
