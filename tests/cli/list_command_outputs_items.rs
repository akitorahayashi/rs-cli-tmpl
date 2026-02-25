use crate::harness::TestContext;
use predicates::prelude::*;
use serial_test::serial;

#[test]
#[serial]
fn list_command_outputs_items() {
    let ctx = TestContext::new();

    ctx.cli().args(["add", "first", "--content", "one"]).assert().success();
    ctx.cli().args(["add", "second", "--content", "two"]).assert().success();

    ctx.cli()
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("first").and(predicate::str::contains("second")));
}
