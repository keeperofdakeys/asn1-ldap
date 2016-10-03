#[macro_use]
extern crate asn1_cereal;

struct LDAPMessage {
  messageID: i32,
  protocolOp: ProtocolOp,
  controls: Option<Controls>,
}

ber_sequence!(
  LDAPMessage,
  "LDAPMessage",
  messageID;
  protocolOp;
  controls ([0] OPTIONAL);
);

enum ProtocolOp {
  bindRequest(BindRequest),
}

ber_choice!(
  ProtocolOp,
  "CHOICE",
  bindRequest
);

struct LdapDN(String);
ber_alias!(LdapDN ::= String, "LdapDN");

struct LdapOID(String);
ber_alias!(LdapOID ::= String, "LdapOID");

struct Controls(Vec<Control>);

ber_alias!(Controls ::= Vec<Control>, "Controls");

struct Control {
  controlType: LdapOID,
  criticality: bool,
  controlValue: Option<String>,
}

ber_sequence!(
  Control,
  "Control",
  controlType;
  criticality (DEFAULT false);
  controlValue ([0] OPTIONAL);
);

struct BindRequest {
  version: u32,
  name: LdapDN,
  authentication: AuthenticationChoice,
}

ber_sequence!(
  BindRequest,
  [APPLICATION 0],
  "BindRequest",
  version;
  name;
  authentication;
);

enum AuthenticationChoice {
  simple(String),
}

ber_choice!(
  AuthenticationChoice,
  "AuthenticationChoice",
  simple
);

