mod suites;

use hivesim::{run_suite, Simulation, Suite, TestSpec};

use crate::suites::beacon::{
    interop::test_portal_beacon_interop, mesh::test_portal_beacon_mesh,
    rpc_compat::run_rpc_compat_beacon_test_suite,
};
use crate::suites::history::{
    interop::test_portal_history_interop, mesh::test_portal_history_mesh,
    rpc_compat::run_rpc_compat_history_test_suite, trin_bridge::test_portal_bridge,
};
use crate::suites::state::{
    interop::test_portal_state_interop, rpc_compat::run_rpc_compat_state_test_suite,
};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let mut history_rpc_compat = Suite {
        name: "history-rpc-compat".to_string(),
        description: "The RPC-compatibility test suite runs a set of RPC related tests against a
        running node. It tests client implementations of the JSON-RPC API for
        conformance with the portal network API specification."
            .to_string(),
        tests: vec![],
    };

    fn create_suite(name: &str, description: &str) -> Suite {
        Suite {
            name: name.to_string(),
            description: description.to_string(),
            tests: vec![],
        }
    }

    fn add_test(suite: &mut Suite, name: &str, description: &str, run: fn()) {
        suite.add(TestSpec {
            name: name.to_string(),
            description: description.to_string(),
            always_run: false,
            run,
            client: None,
        });
    }

    history_rpc_compat.add(TestSpec {
        name: "client launch".to_string(),
        description: "This test launches the client and collects its logs.".to_string(),
        always_run: false,
        run: run_rpc_compat_history_test_suite,
        client: None,
    });

    let mut history_interop = Suite {
        name: "history-interop".to_string(),
        description:
            "The interop test suite runs a set of scenarios to test interoperability between
        portal network clients"
                .to_string(),
        tests: vec![],
    };

    history_interop.add(TestSpec {
        name: "client launch".to_string(),
        description: "This test launches the client and collects its logs.".to_string(),
        always_run: false,
        run: test_portal_history_interop,
        client: None,
    });

    let mut history_mesh = Suite {
        name: "history-mesh".to_string(),
        description: "The portal mesh test suite runs a set of scenarios to test 3 clients"
            .to_string(),
        tests: vec![],
    };

    history_mesh.add(TestSpec {
        name: "client launch".to_string(),
        description: "This test launches the client and collects its logs.".to_string(),
        always_run: false,
        run: test_portal_history_mesh,
        client: None,
    });

    let mut history_trin_bridge = Suite {
        name: "history-trin-bridge".to_string(),
        description: "The portal bridge test suite".to_string(),
        tests: vec![],
    };

    history_trin_bridge.add(TestSpec {
        name: "client launch".to_string(),
        description: "This test launches the client and collects its logs.".to_string(),
        always_run: false,
        run: test_portal_bridge,
        client: None,
    });

    let mut state_rpc_compat = create_suite(
        "state-rpc-compat",
        "The RPC-compatibility test suite runs a set of RPC related tests against a running node. It tests client implementations of the JSON-RPC API for conformance with the portal network API specification."
    );

    add_test(
        &mut state_rpc_compat,
        "client launch",
        "This test launches the client and collects its logs.",
        run_rpc_compat_state_test_suite,
        None
    );

    let mut state_interop = create_suite(
        "state-interop",
        "The interop test suite runs a set of scenarios to test interoperability between portal network clients"
    );

    add_test(
        &mut state_interop,
        "client launch",
        "This test launches the client and collects its logs.",
        test_portal_state_interop,
        None
    );

    let mut beacon_rpc_compat = create_suite(
        "beacon-rpc-compat",
        "The RPC-compatibility test suite runs a set of RPC related tests against a running node. It tests client implementations of the JSON-RPC API for conformance with the portal network API specification."
    );

    add_test(
        &mut beacon_rpc_compat,
        "client launch",
        "This test launches the client and collects its logs.",
        run_rpc_compat_beacon_test_suite,
        None
    );

    let mut beacon_interop = create_suite(
        "beacon-interop",
        "The interop test suite runs a set of scenarios to test interoperability between portal network clients"
    );

    add_test(
        &mut beacon_interop,
        "client launch",
        "This test launches the client and collects its logs.",
        test_portal_beacon_interop,
        None
    );

    let mut beacon_mesh = create_suite(
        "beacon-mesh",
        "The portal mesh test suite runs a set of scenarios to test 3 clients"
    );

    add_test(
        &mut beacon_mesh,
        "client launch",
        "This test launches the client and collects its logs.",
        test_portal_beacon_mesh,
        None
    );

    let sim = Simulation::new();
    run_suite(
        sim,
        vec![
            history_rpc_compat,
            history_interop,
            history_mesh,
            history_trin_bridge,
            state_rpc_compat,
            state_interop,
            beacon_rpc_compat,
            beacon_interop,
            beacon_mesh,
        ],
    )
    .await;
}
