# SGX Test

Crates to assist in testing rust code used in an SGX enclave

This consists of three crates:

1. `sgx-test` The host executable to be invoked by cargo
2. `sgx-test-runner` The target entry point which will exercise each called out test function
3. `sgx-test-macro` The macro logic to decorate functions as test functions
