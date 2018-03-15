rpc_api! {
    metadata {
        name = poll;
        version = "0.1.0";
        client_attestation_required = false;
    }
    rpc vote(VoteRequest) -> VoteResponse;

}
